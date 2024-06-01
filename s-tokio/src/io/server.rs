use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub async fn server() -> io::Result<()> {
    let server = TcpListener::bind(("127.0.0.1", 8888_u16)).await?;

    while let Ok((stream, socket)) = server.accept().await {
        println!("accept client {:?}", socket);
        tokio::spawn(async move {
            process_connect(stream).await;
        });
    }
    Ok(())
}

pub async fn process_connect(stream: TcpStream) {
    let (client_reader, client_writer) = stream.into_split();
    let (msg_tx, msg_rx) = mpsc::channel::<String>(100);

    // 从客户端读取的异步子任务
    let mut read_task = tokio::spawn(async move {
        read_from_client(client_reader, msg_tx).await;
    });

    // 向客户端写入的异步子任务
    let mut write_task = tokio::spawn(async move {
        write_to_client(client_writer, msg_rx).await;
    });

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}

/// 从客户端读取
pub async fn read_from_client(reader: OwnedReadHalf, msg_tx: mpsc::Sender<String>) {
    let mut buf_reader = io::BufReader::new(reader);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => {
                eprintln!("read from client error");
                break;
            }
            // 遇到了EOF
            Ok(0) => {
                println!("client closed");
                break;
            }
            Ok(n) => {
                // read_line()读取时会包含换行符，因此去除行尾换行符
                // 将buf.drain(。。)会将buf清空，下一次read_line读取的内容将从头填充而不是追加
                buf.pop();
                let content = buf.drain(..).as_str().to_string();
                println!("read {} bytes from client. content: {}", n, content);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if msg_tx.send(content).await.is_err() {
                    eprintln!("receiver closed");
                    break;
                }
            }
        }
    }
}

/// 写给客户端
async fn write_to_client(writer: OwnedWriteHalf, mut msg_rx: mpsc::Receiver<String>) {
    let mut buf_writer = io::BufWriter::new(writer);
    while let Some(mut str) = msg_rx.recv().await {
        str.push('\n');
        println!("send to client{:?}", str);
        if let Err(e) = buf_writer.write_all(str.as_bytes()).await {
            eprintln!("write to client failed: {}", e);
            break;
        }
        let _c = buf_writer.flush().await;
    }
}

#[cfg(test)]
mod tests {
    use crate::io::server::server;

    #[tokio::test]
    async fn test_server() {
        let _a = server().await.unwrap();
    }
}
