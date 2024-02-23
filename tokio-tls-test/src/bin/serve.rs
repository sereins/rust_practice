use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, split};
use tokio::net::TcpListener;
use tokio_tls_test::init;

lazy_static::lazy_static! {
    static ref RESPONSE_403:&'static str = concat!(
        "HTTP/1.1 403 Forbidden\r\n" ,"Content-Length: 0\r\n" ,"Connection: closed\r\n\r\n"
    );
    static ref RESPONSE_200:&'static str = concat!(
        "HTTP/1.1 200 OK\r\n" ,
        "Content-Length: 11\r\n" ,
        "Connection: closed\r\n\r\n",
        "Are you OK?",
    );
}

#[tokio::main]
async fn main()->io::Result<()> {
    let addr = SocketAddr::from(([127,0,0,1],8099));
    let listener = TcpListener::bind(&addr).await?;

    let acceptor = init("server.crt", "server.key")?;

    loop {

        let accept = listener.accept().await;
        match accept {
            Ok((stream,_peer_addr)) => {
                let acceptor = acceptor.clone();
                match acceptor.accept(stream).await {
                    Ok(stream) => {
                        tokio::spawn(async move {
                            if let Err(_err) = handle_stream(stream).await {
                                eprintln!("TLS Handler err: {:?}", _err);
                            }
                        });
                    }
                    Err(err) => {eprintln!("Tcp0 err :{:?}",err)}
                }
            }
            Err(err) => eprintln!("Tcp err :{:?}",err)
        }
    }
}

async fn handle_stream<IO>(stream:IO) -> io::Result<()>
where IO:AsyncRead + AsyncWrite + Unpin + AsyncWriteExt
{
    let (mut local_reader ,mut local_writer) = split(stream);

    let mut head = [0u8; 2048];
    let n = local_reader.read(&mut head[..]).await?;

    if n == 2048 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Receive a unexpected big size of header!!",
        ));
    }
    let head_str = std::str::from_utf8(&head[..n])
        .map_err(|x| io::Error::new(io::ErrorKind::Interrupted, x))?;
    println!("\r\nReceived request from client: \r\n{}\r\n", head_str);

    // 回复200OK
    local_writer.write_all(RESPONSE_200.as_bytes()).await?;
    local_writer.shutdown().await?;
    Ok(()) as io::Result<()>
}
