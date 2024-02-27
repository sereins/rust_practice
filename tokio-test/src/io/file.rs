use tokio::io;
use tokio::io::AsyncBufReadExt;

#[tokio::test]
async fn test_read_line() -> io::Result<()> {
    let file = tokio::fs::File::open("a.log").await?;

    // let mut buf_reader = tokio::io::BufReader::new(file).lines();
    // while let Some(line) = buf_reader.next_line().await.unwrap() {
    //     println!("{}", line);
    // }
    //
    let mut buf = String::new();
    let mut buf_reader = io::BufReader::new(file);

    loop {
        match buf_reader.read_line(&mut buf).await {
            Ok(0) => break,
            Ok(_n) => {
                println!("buf{}", buf);
                buf.clear();
            }
            Err(e) => { println!("error:{:?}", e) }
        };
    }
    //

    // let mut buf_reader = tokio::io::BufReader::new(file);
    // let mut buf = String::new();
    //
    // loop {
    //     match buf_reader.read_line(&mut buf).await {
    //         Err(_e) => panic!("read file error"),
    //         // 遇到了文件结尾，即EOF
    //         Ok(0) => break,
    //         Ok(_n) => {
    //             // read_line()总是保留行尾换行符(如果有的话)，因此使用print!()而不是println!()
    //             print!("{}", buf);
    //             // read_line()总是将读取的内容追加到buf，因此每次读取完之后要清空buf
    //             buf.clear();
    //         }
    //     }
    // }
    Ok(())
}

