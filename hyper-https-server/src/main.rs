use core::convert::Infallible;
use futures::stream::{StreamExt, TryStreamExt};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use piping_server::piping_server::PipingServer;
use piping_server::req_res_handler::req_res_handler;
use piping_server::util;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bind_address = args[1].clone();

    let piping_server = &PipingServer::new();

    // Set default log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let key_path = "./ssl_certs/server.key";
    let crt_path = "./ssl_certs/server.crt";

    let tls_cfg_rwlock_arc: Arc<RwLock<Arc<rustls::ServerConfig>>> =
        util::hot_reload_tls_cfg(crt_path, key_path);
    // Create a TCP listener via tokio.
    let mut tcp: TcpListener = TcpListener::bind(bind_address.clone()).await?;
    // Prepare a long-running future stream to accept and serve clients.
    let incoming_tls_stream = util::TokioIncoming::new(&mut tcp)
        .map_err(|e| util::make_io_error(format!("Incoming failed: {:?}", e)))
        // (base: https://github.com/cloudflare/wrangler/pull/1485/files)
        .filter_map(|s| async {
            let client = match s {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed to accept client: {}", e);
                    return None;
                }
            };

            let tls_cfg: Arc<rustls::ServerConfig> = (*tls_cfg_rwlock_arc.read().unwrap()).clone();
            match TlsAcceptor::from(tls_cfg).accept(client).await {
                Ok(x) => Some(Ok::<_, std::io::Error>(x)),
                Err(e) => {
                    log::error!("Client connection error: {}", e);
                    None
                }
            }
        });
    let https_svc = make_service_fn(move |_| {
        let piping_server = piping_server.clone();
        let handler =
            req_res_handler(move |req, res_sender| piping_server.handler(true, req, res_sender));
        futures::future::ok::<_, Infallible>(service_fn(handler))
    });
    let https_server = Server::builder(util::HyperAcceptor {
        acceptor: incoming_tls_stream,
    })
    .serve(https_svc);

    log::info!("HTTPS server is running on {}...", bind_address);
    https_server.await?;
    Ok(())
}
