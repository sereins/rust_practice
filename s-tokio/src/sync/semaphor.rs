// cargo test -p tokio-test sync::semaphor::test -- --nocapture
#[cfg(test)]
mod test {
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[tokio::test]
    async fn semaphor() {
        // 信号通知机制
        let semaphor = Arc::new(Semaphore::new(3));

        let _permit1 = semaphor.acquire().await.unwrap();
        println!("get one permit");

        let permit2 = semaphor.clone().acquire_many_owned(2).await.unwrap();
        println!("get two permit");

        tokio::spawn(async move {
            // 等待3秒释放permit2
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            drop(permit2);
        });

        let _permit3 = semaphor.acquire().await.unwrap();
        println!("get three permit");
    }
}
