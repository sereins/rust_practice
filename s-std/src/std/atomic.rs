use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

/// atomic type 是rust std标准库定义了哪些类型可以进行原子操作而无需锁的操作。
/// order 用来控制和其他线程的顺序操作,比如:等待其他现场写,读。

pub fn counter() {
    let counter = Arc::new(AtomicUsize::new(0));

    // 10个线程，每个线程1000次，对计数器进行原子操作
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.load(Ordering::SeqCst));
}

pub fn acquire() {
    let ready = Arc::new(AtomicBool::new(false));
    let r = ready.clone();

    let handle = thread::spawn(move || {
        while !r.load(Ordering::Acquire) {
            println!("Waiting for data to be ready...");
        }
        println!("Data is ready!");
    });

    // sleep 5 microseconds
    thread::sleep(std::time::Duration::from_micros(5));

    // Do some work
    ready.store(true, Ordering::Release);
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        counter()
    }

    #[test]
    fn test_acquire() {
        acquire()
    }
}
