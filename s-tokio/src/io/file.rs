use tokio::io;
use tokio::io::AsyncBufReadExt;

pub async fn read_line() -> io::Result<()> {
    let file = tokio::fs::File::open("a.log").await?;

    let mut buf = String::new();
    let mut buf_reader = io::BufReader::new(file);

    loop {
        match buf_reader.read_line(&mut buf).await {
            Ok(0) => break,
            Ok(_n) => {
                println!("buf{}", buf);
                buf.clear();
            }
            Err(e) => {
                println!("error:{:?}", e)
            }
        };
    }
    Ok(())
}
#[cfg(test)]
mod test {
    use super::read_line;
    use tokio::io;

    #[tokio::test]
    async fn test_read_line() -> io::Result<()> {
        read_line().await
    }
}
