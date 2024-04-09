#[cfg(test)]
mod tests {
    use tokio::sync;
    use tokio::time::{sleep, Duration};

    // 多生产者，单一消费者模型
    #[tokio::test]
    async fn mpsc() {
        let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

        tokio::spawn(async move {
            for i in 0..20 {
                let _rc = tx.send(i).await;
            }
        });

        while let Some(rs) = rx.recv().await {
            println!("接收到值:{}", rs);
        }
    }

    #[tokio::test]
    async fn broadcast() {
        let (tx, mut rx) = sync::broadcast::channel::<i32>(10);
        let mut rx2 = tx.subscribe();

        let task1 = tokio::spawn(async move {
            while let Ok(rs) = rx.recv().await {
                println!("rx1 接收到值{}", rs);
            }
        });

        let task2 = tokio::spawn(async move {
            while let Ok(rs) = rx2.recv().await {
                println!("rx2 接收到值{}", rs);
            }
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();

        let _ = tokio::join!(task1, task2);
    }

    #[tokio::test]
    async fn oneshot() {
        let (tx, rx) = sync::oneshot::channel::<i32>();

        tokio::spawn(async move {
            if let Err(_) = tx.send(3) {
                println!("the receiver dropped");
            }
        });

        match rx.await {
            Ok(v) => println!("got = {:?}", v),
            Err(_) => println!("the sender dropped"),
        }
    }

    #[tokio::test]
    async fn watch() {
        let (tx, mut rx) = sync::watch::channel("hello");

        tokio::spawn(async move {
            loop {
                println!("{}! ", *rx.borrow_and_update());
                if rx.changed().await.is_ok() {
                    println!("值发生改变{}! ", *rx.borrow());
                    break;
                }
            }
        });

        sleep(Duration::from_millis(100)).await;
        tx.send("world").unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}
