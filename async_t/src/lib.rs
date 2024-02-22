use std::sync::mpsc::sync_channel;
use std::thread;
use crate::executor::{Executor, Spawner};

pub mod time_futures;
pub mod executor;

/// 创建一个执行器以及任务生成器
fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (tx, rx) = sync_channel(100);

    println!("[{:?}] 生成新的Executor 和 Spawner ", thread::current().id());

    (Executor { ready_queue:rx }, Spawner { task_sender:tx })
}

#[cfg(test)]
mod tests{
    use std::thread;
    use std::time::Duration;
    use crate::time_futures::TimerFuture;
    use super::*;

    #[test]
    fn async_work(){
        let (executor, spawner) = new_executor_and_spawner();

        // 生成一个任务
        spawner.spawn(async {
            println!("[{:?}]howdy!",thread::current().id());
            // 创建定时器Future，并等待它完成
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("[{:?}]done!",thread::current().id());
        });

        // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
        drop(spawner);

        // 运行执行器直到任务队列为空
        // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
        executor.run();
    }
}