use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::io::{AsyncWriteExt, ReadHalf, split, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{Certificate, ClientConfig, Error, OwnedTrustAnchor, RootCertStore, ServerName};
use tokio_rustls::rustls::client::{ServerCertVerified, ServerCertVerifier};
use tokio_rustls::{rustls, TlsConnector};
use tokio_rustls::client::TlsStream;

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
    allow_insecure: bool,
) -> io::Result<(ReadHalf<TlsStream<TcpStream>>, WriteHalf<TlsStream<TcpStream>>)> {
    let socket_addr = (dst_addr, dst_port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    let mut root_store = RootCertStore::empty();

    let a = webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    });
    root_store.add_server_trust_anchors(a);

    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // 如果不需要
    if allow_insecure {
        config.dangerous().set_certificate_verifier(Arc::new(NoCertVerifier {}));
    }

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&socket_addr).await?;

    let domain = rustls::ServerName::try_from(sni)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dns name"))?;

    let stream = connector.connect(domain, stream).await?;

    Ok(split(stream))
}

struct NoCertVerifier {}

impl ServerCertVerifier for NoCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item=&[u8]>,
        _ocsp_response: &[u8], now: SystemTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }
}

