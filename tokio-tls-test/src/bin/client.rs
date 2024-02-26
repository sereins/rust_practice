use std::io;
use std::net::{SocketAddr};
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, ReadHalf, split, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{ClientConfig};
use tokio_rustls::{rustls, TlsConnector};
use tokio_rustls::client::TlsStream;
use tokio_tls_test::{load_cert, load_keys, load_root_cert, TlsCert};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sni = "example.com";
    let addr = SocketAddr::from(([127, 0, 0, 1], 8099));

    let tls_cert = TlsCert::new("root.crt", "client.crt", "client.key");
    let (mut reader, mut writer) = connect(addr, sni, tls_cert).await?;

    let content = "client content";
    writer.write_all(content.as_bytes()).await?;
    let mut stdout = tokio::io::stdout();
    tokio::io::copy(&mut reader, &mut stdout).await?;
    Ok(())
}

async fn connect(
    addr: SocketAddr,
    sni: &str,
    tls_cert: TlsCert,
) -> io::Result<(ReadHalf<TlsStream<TcpStream>>, WriteHalf<TlsStream<TcpStream>>)> {
    let root_store = load_root_cert(Path::new(&tls_cert.root_cert_path));
    let client_pem = load_cert(Path::new(&tls_cert.cert_path))?;
    let key = load_keys(Path::new(&tls_cert.key_path))?
        .pop()
        .unwrap();

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_single_cert(client_pem, key).unwrap();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = rustls::ServerName::try_from(sni)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dns name"))?;

    // 感觉本质上是在原有的stream套了一层 tls
    let stream = connector.connect(domain, stream).await?;

    Ok(split(stream))
}
