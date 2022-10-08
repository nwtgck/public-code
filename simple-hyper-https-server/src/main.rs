use core::convert::Infallible;
use futures::stream::{StreamExt, TryStreamExt};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use simple_hyper_https_server::util;

async fn handle(
    _: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("Hello, World!\n".into()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bind_address = args[1].clone();

    // Set default log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let key_path = "./ssl_certs/server.key";
    let crt_path = "./ssl_certs/server.crt";

    let tls_cfg = Arc::new(util::load_tls_config(crt_path, key_path)?);
    // Create a TCP listener via tokio.
    let mut tcp: TcpListener = TcpListener::bind(bind_address.clone()).await?;
    // Prepare a long-running future stream to accept and serve clients.
    let incoming_tls_stream = util::TokioIncoming::new(&mut tcp)
        .map_err(|e| anyhow::anyhow!(format!("Incoming failed: {:?}", e)))
        // (base: https://github.com/cloudflare/wrangler/pull/1485/files)
        .filter_map(|s| async {
            let client = match s {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed to accept client: {}", e);
                    return None;
                }
            };

            match TlsAcceptor::from(tls_cfg.clone()).accept(client).await {
                Ok(x) => Some(Ok::<_, std::io::Error>(x)),
                Err(e) => {
                    log::error!("Client connection error: {}", e);
                    None
                }
            }
        });
    let https_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let https_server = Server::builder(util::HyperAcceptor {
        acceptor: incoming_tls_stream,
    })
    .serve(https_svc);

    log::info!("HTTPS server is running on {}...", bind_address);
    https_server.await?;
    Ok(())
}
