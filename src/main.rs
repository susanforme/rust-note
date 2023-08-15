use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
    // tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写
    let (tx, rx) = mpsc::channel();
    // 通过克隆发送者来创建多个生产者
    let other_tx = tx.clone();
    let start_str = String::from("welcome to channel!");
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        tx.send(start_str).unwrap();
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // 编译失败所有权已转移
        // println!("val is {}", val);
    });
    handle.join().unwrap();
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            other_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // recv阻塞主进程直到接受到值
    for received in rx {
        println!("Got :{}", received);
    }
}
