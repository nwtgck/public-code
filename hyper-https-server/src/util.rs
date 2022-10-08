use core::pin::Pin;
use core::task::{Context, Poll};
use futures::ready;
use pin_project_lite::pin_project;

use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

fn read_private_keys(rd: &mut dyn std::io::BufRead) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let mut keys = Vec::<Vec<u8>>::new();

    loop {
        match rustls_pemfile::read_one(rd)? {
            None => return Ok(keys),
            Some(
                rustls_pemfile::Item::RSAKey(key)
                | rustls_pemfile::Item::PKCS8Key(key)
                | rustls_pemfile::Item::ECKey(key),
            ) => keys.push(key),
            _ => {}
        };
    }
}

// (base: https://github.com/ctz/hyper-rustls/blob/5f073724f7b5eee3a2d72f0a86094fc2718b51cd/examples/server.rs)
pub fn load_tls_config(
    cert_path: impl AsRef<std::path::Path>,
    key_path: impl AsRef<std::path::Path> + std::fmt::Display,
) -> anyhow::Result<rustls::ServerConfig> {
    // Load public certificate.
    let mut cert_reader = std::io::BufReader::new(std::fs::File::open(cert_path)?);
    let certs = rustls_pemfile::certs(&mut cert_reader)?;
    // Load private key.
    let mut key_reader = std::io::BufReader::new(std::fs::File::open(key_path)?);
    // Load and return a single private key.
    let mut private_keys = read_private_keys(&mut key_reader)?;
    let key = if private_keys.len() > 0 {
        Ok(private_keys.remove(0))
    } else {
        Err(anyhow::anyhow!("failed to get private key".to_owned()))
    }?;
    let certificates: Vec<rustls::Certificate> =
        certs.into_iter().map(rustls::Certificate).collect();
    let mut cfg = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certificates, rustls::PrivateKey(key))?;
    // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
    cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    Ok(cfg)
}

pin_project! {
    pub struct HyperAcceptor<S> {
        #[pin]
        pub acceptor: S,
    }
}

impl<S> hyper::server::accept::Accept for HyperAcceptor<S>
where
    S: futures::stream::Stream<Item = Result<TlsStream<TcpStream>, std::io::Error>>,
{
    type Conn = TlsStream<TcpStream>;
    type Error = std::io::Error;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        self.project().acceptor.as_mut().poll_next(cx)
    }
}

// (base: https://github.com/tokio-rs/tokio/blob/tokio-0.2.22/tokio/src/net/tcp/incoming.rs)
/// Stream returned by the `TcpListener::incoming` function representing the
/// stream of sockets received from a listener.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct TokioIncoming<'a> {
    inner: &'a mut tokio::net::TcpListener,
}

impl TokioIncoming<'_> {
    pub fn new(listener: &mut tokio::net::TcpListener) -> TokioIncoming<'_> {
        TokioIncoming { inner: listener }
    }
}

impl futures::stream::Stream for TokioIncoming<'_> {
    type Item = tokio::io::Result<TcpStream>;

    #[allow(unused_mut)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (socket, _) = ready!(self.inner.poll_accept(cx))?;
        Poll::Ready(Some(Ok(socket)))
    }
}
