use s_tls::{load_cert, load_keys, load_root_cert, TlsCert};
use std::io;
use std::net::SocketAddr;
use std::path::Path;
use tokio::io::{split, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::server::AllowAnyAuthenticatedClient;
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8099));
    let listener = TcpListener::bind(&addr).await?;

    let tls_cert = TlsCert::new("root.crt", "server.crt", "server.key");
    let acceptor = tls_acceptor(tls_cert)?;

    loop {
        let accept = listener.accept().await;
        match accept {
            Ok((stream, _peer_addr)) => {
                let acceptor = acceptor.clone();
                // 在原有的stream上套一层,使用tls的acceptor
                match acceptor.accept(stream).await {
                    Ok(stream) => {
                        tokio::spawn(async move {
                            if let Err(_err) = handle_stream(stream).await {
                                eprintln!("TLS Handler err: {:?}", _err);
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("Tcp0 err :{:?}", err)
                    }
                }
            }
            Err(err) => eprintln!("Tcp err :{:?}", err),
        }
    }
}

fn tls_acceptor(tls_cert: TlsCert) -> io::Result<TlsAcceptor> {
    let certs = load_cert(Path::new(&tls_cert.cert_path))?;
    let mut keys = load_keys(Path::new(&tls_cert.key_path))?;

    let root_store = load_root_cert(Path::new(&tls_cert.root_cert_path));
    let root_cert = AllowAnyAuthenticatedClient::new(root_store);

    let server_conf = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(root_cert)
        .with_single_cert(certs, keys.remove(0))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "TLS cert loading error"))?;

    Ok(TlsAcceptor::from(std::sync::Arc::new(server_conf)))
}

async fn handle_stream<IO>(stream: IO) -> io::Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin + AsyncWriteExt,
{
    let (mut local_reader, mut local_writer) = split(stream);

    // let mut buffer = [0u8; 2048];
    // loop {
    //     let n = local_reader.read(&mut buffer).await?;
    //     let res = std::str::from_utf8(&buffer[..n]).unwrap();
    //     println!("\r\nReceived message:{}", res);
    // }

    let mut buffer = [0u8; 2048];
    let n = local_reader.read(&mut buffer[..]).await?;
    if n == 2048 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Receive a unexpected big size of header!!",
        ));
    }
    let head_str = std::str::from_utf8(&buffer[..n])
        .map_err(|x| io::Error::new(io::ErrorKind::Interrupted, x))?;
    println!("\r\nReceived request from client: \r\n{}\r\n", head_str);

    let content = "This is server response content";
    local_writer.write_all(content.as_bytes()).await?;

    local_writer.shutdown().await?;
    Ok(()) as io::Result<()>
}
