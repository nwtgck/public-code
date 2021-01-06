use async_stream::stream;
use core::convert::Infallible;
use futures_util::future::TryFutureExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

async fn hello_world(
    _req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("hello, world\n".into()))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // NOTE: Create .crt and .key by: openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -sha256 -nodes --subj '/CN=localhost/'
    let tls_cfg = load_tls_config("./ssl_certs/server.crt", "./ssl_certs/server.key")?;

    let https_port = 8443;
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], https_port).into();
    // Create a TCP listener via tokio.
    let tcp = TcpListener::bind(&addr).await?;
    let tls_acceptor = TlsAcceptor::from(std::sync::Arc::new(tls_cfg));
    // Prepare a long-running future stream to accept and serve clients.
    // (base: https://github.com/ctz/hyper-rustls/blob/5a30ca520ab382bdeb06ba37a1401b6f5aeb971f/examples/server.rs#L60-L72)
    let incoming_tls_stream = stream! {
        loop {
            let (socket, _) = tcp.accept().await?;
            let stream = tls_acceptor.accept(socket).map_err(|e| {
                println!("[!] Voluntary server halt due to client-connection error...");
                // Errors could be handled here, instead of server aborting.
                // Ok(None)
                error(format!("TLS Error: {:?}", e))
            });
            yield stream.await;
        }
    };
    let https_svc = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    let https_server = Server::builder(HyperAcceptor {
        acceptor: Box::pin(incoming_tls_stream),
    })
        .serve(https_svc);

    println!("Listening on {}...", https_port);
    if let Err(e) = https_server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

// (base: https://github.com/ctz/hyper-rustls/blob/5a30ca520ab382bdeb06ba37a1401b6f5aeb971f/examples/server.rs#L85-L99)
pub struct HyperAcceptor<S> {
    pub acceptor: core::pin::Pin<Box<S>>,
}

impl<S> hyper::server::accept::Accept for HyperAcceptor<S>
    where
        S: futures::stream::Stream<
            Item = Result<tokio_rustls::server::TlsStream<tokio::net::TcpStream>, std::io::Error>,
        >,
{
    type Conn = tokio_rustls::server::TlsStream<tokio::net::TcpStream>;
    type Error = std::io::Error;

    fn poll_accept(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context,
    ) -> core::task::Poll<Option<Result<Self::Conn, Self::Error>>> {
        self.acceptor.as_mut().poll_next(cx)
    }
}

// (base: https://github.com/ctz/hyper-rustls/blob/5f073724f7b5eee3a2d72f0a86094fc2718b51cd/examples/server.rs)
pub fn load_tls_config(
    cert_path: impl AsRef<std::path::Path>,
    key_path: impl AsRef<std::path::Path> + std::fmt::Display,
) -> std::io::Result<rustls::ServerConfig> {
    // Load public certificate.
    let mut cert_reader = std::io::BufReader::new(std::fs::File::open(cert_path)?);
    let certs = rustls::internal::pemfile::certs(&mut cert_reader)
        .map_err(|_| error("unable to load certificate".to_owned()))?;
    // Load private key.
    let mut key_reader = std::io::BufReader::new(std::fs::File::open(key_path)?);
    // Load and return a single private key.
    let key = rustls::internal::pemfile::pkcs8_private_keys(&mut key_reader)
        .map_err(|_| error("unable to load private key".to_owned()))?
        .remove(0);
    // Do not use client certificate authentication.
    let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    // Select a certificate to use.
    cfg.set_single_cert(certs, key).unwrap();
    // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
    cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
    Ok(cfg)
}

fn error(err: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}
