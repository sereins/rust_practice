use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub async fn connect() -> io::Result<()> {
    let connect = TcpStream::connect("127.0.0.1:8888").await?;

    let (read_half, mut write_half) = connect.into_split();

    let content = format!("hello server,cc\n");
    let _a = write_half.write_all(content.as_bytes()).await?;
    println!("发送数据到服务端");

    let mut buf_reader = BufReader::new(read_half);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(e) => {
                eprintln!("读取错误{:?}", e);
                break;
            }
            Ok(0) => {
                break;
            }
            Ok(_n) => {
                buf.pop();
                let content = buf.drain(..).as_str().to_string();
                println!("come from server:{:?}", content);
                break;
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::io::client::connect;

    #[tokio::test]
    async fn test_connect() {
        let _c = connect().await.unwrap();
    }
}
