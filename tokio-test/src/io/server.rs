use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

async fn server() -> io::Result<()> {
    let server = TcpListener::bind(([127, 0, 0, 1], 8888)).await?;

    while let Ok((stream, socket)) = server.accept().await {

        println!("accept client {:?}", socket);
        tokio::spawn(async move {
            process_connect(stream).await;
        });
    };
    Ok(())
}

async fn process_connect(stream: TcpStream) {
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


#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn server() {
        println!("hello world");
    }
}
