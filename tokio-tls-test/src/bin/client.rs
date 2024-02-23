use std::io;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, ReadHalf, split, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::{rustls, TlsConnector};
use tokio_rustls::client::TlsStream;
use tokio_tls_test::{load_cert, load_keys};

#[tokio::main]
async fn main() -> io::Result<()> {
    let allow_insecure = false;

    let sni = "example.com";
    let dst_addr = "127.0.0.1";
    let dst_port = 8099;
    let content = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", sni);

    let (mut reader, mut writer) = connect(dst_addr, dst_port, sni, allow_insecure).await?;
    writer.write_all(content.as_bytes()).await?;

    let mut stdout = tokio::io::stdout();
    tokio::io::copy(&mut reader, &mut stdout).await?;
    Ok(())
}

async fn connect(
    dst_addr: &str,
    dst_port: u16,
    sni: &str,
    _allow_insecure: bool,
) -> io::Result<(ReadHalf<TlsStream<TcpStream>>, WriteHalf<TlsStream<TcpStream>>)> {
    let socket_addr = (dst_addr, dst_port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    let mut root_store = RootCertStore::empty();
    let cert = load_cert(Path::new("root.crt")).unwrap();
    for core in cert {
        root_store.add(&core).expect("load cert file");
    }

    let client_pem = load_cert(Path::new("client.crt")).unwrap();
    let key = load_keys(Path::new("client.key")).unwrap().pop().unwrap();

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_single_cert(client_pem, key).unwrap();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&socket_addr).await?;

    let domain = rustls::ServerName::try_from(sni)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dns name"))?;

    let stream = connector.connect(domain, stream).await?;

    Ok(split(stream))
}
