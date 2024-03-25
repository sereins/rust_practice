// 默认启动的线程和操作系统的核心数一样
// 当创建一个runtime时，会启动一个executor 和 scheduler, task queue

// worker thread 用于异步任务的线程和 blocking thread用于阻赛队列的线程，比如sleep很长一段时间的线程,不会接收tokio
// 调度器的调度，而是接收操作系统的调度。

#[allow(dead_code)]
pub fn runtime() {
    let _rt = tokio::runtime::Runtime::new().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));
}

#[test]
fn t_runtime() {
    runtime();
}

