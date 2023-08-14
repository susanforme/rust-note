use std::{thread, time::Duration};

fn main() {
    // 这个程序的输出可能每次都略有不同
    let handle = thread::spawn(|| {
        // 只会打印到5,主线程结束
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            // sleep
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} form the main thread!",);
        thread::sleep(Duration::from_millis(1));
    }
    // join等待线程结束
    handle.join().unwrap();
}
