use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // 原子引用计数 Arc<T> 线程安全带有性能惩罚在必要时才为此买单
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // counter是不可变的,但是提供了内部可变性
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
