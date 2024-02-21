use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use rustls_pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

// 加载证书
pub fn load_cert(path:&Path)-> io::Result<Vec<Certificate>>{
    let mut buf_reader = BufReader::new(File::open(path)?);
    certs(&mut buf_reader)
        .map_err(|_|io::Error::new(io::ErrorKind::InvalidInput,"invalid cert"))
        .map(|mut cert| cert.drain(..).map(Certificate).collect())
}
pub fn load_keys(path:&Path)->io::Result<Vec<PrivateKey>>{
    let mut buf_reader = BufReader::new(File::open(path)?);
    rsa_private_keys(&mut buf_reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput,"invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

pub fn init(cert_path:&str, key_path:&str) ->io::Result<TlsAcceptor>{
    let certs  =  load_cert(Path::new(cert_path))?;
    let mut keys = load_keys(Path::new(key_path))?;

    let server_conf = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs,keys.remove(0))
        .map_err(|_|io::Error::new(io::ErrorKind::InvalidInput,"TLS cert loading error"))?;

    Ok(TlsAcceptor::from(std::sync::Arc::new(server_conf)))
}