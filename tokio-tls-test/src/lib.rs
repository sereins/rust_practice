use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio_rustls::rustls::{Certificate, PrivateKey, RootCertStore};

// 证书的配置
pub struct TlsCert {
    pub root_cert_path: String,
    pub cert_path: String,
    pub key_path: String,
}

impl TlsCert {
    pub fn new<'a>(root_cert_path: &str, cert_path: &str, key_path: &str) -> Self {
        Self {
            root_cert_path: root_cert_path.to_string(),
            cert_path: cert_path.to_string(),
            key_path: key_path.to_string(),
        }
    }
}

// 加载证书
pub fn load_cert(path: &Path) -> io::Result<Vec<Certificate>> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    certs(&mut buf_reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut cert| cert.drain(..).map(Certificate).collect())
}

// 加载密钥
pub fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let mut buf_reader = BufReader::new(File::open(path)?);
    pkcs8_private_keys(&mut buf_reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

// 加载根证书
pub fn load_root_cert(path: &Path) -> RootCertStore {
    let mut root_store = RootCertStore::empty();
    let cert = load_cert(Path::new(path)).unwrap();
    for core in cert {
        root_store.add(&core).expect("load cert file");
    }
    root_store
}