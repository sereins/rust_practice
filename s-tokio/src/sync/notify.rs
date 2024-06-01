// cargo test -p tokio-test sync::notify::test -- --nocapture

// 类似于标准库中的par、unpark 都是用于来唤醒另一个线程来执行任务.
// 也可以把他看作具有初始值为0 的Semaphore
#[cfg(test)]
mod test {
    use std::sync::Arc;
    use tokio::sync::Notify;

    #[tokio::test]
    async fn noitfy() {
        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        let task = tokio::spawn(async move {
            notify2.notified().await;
            println!("reveive notify");
        });

        // 如果没有接收到消息是会等待
        notify.notify_one();
        let _rs = task.await;
    }
}
