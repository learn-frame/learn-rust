//! 编程中的通道有两部分组成, 一个发送者(transmitter)和一个接收者(receiver)
//! 发送者位于上游, 接收者则位于下游
//! 当发送者或接收者任一被丢弃时可以认为通道被关闭(close)了
use chrono::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn entry() {
    one_sender_one_receiver();
    send_multi_values();
    multi_senders_one_receiver();
}

pub fn create_channel<T>() -> (Sender<T>, Receiver<T>) {
    // mpsc 是多个生产者, 单个消费者(multiple producer, single consumer)的缩写
    // Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送(sending)端,
    // 但只能有一个消费这些值的接收(receiving)端. 想象一下多条小河小溪最终汇聚成大河
    let channel = channel();
    channel
}

pub fn one_sender_one_receiver() {
    let (tx, rx) = create_channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let val = String::from("hi");
        tx.send(val).unwrap();

        // 😈: 当 val 被 send 调用, val 的所有权就被 move 了, 因此下面无法打印出 val
        // println!("val is {}", val);
    });

    // recv() 会阻塞主线程, 直到 sender 的数据到来
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // try_recv() 返回一个 Result<T, TryRecvError>
    // 因为我们的代码是 1s 后才去 send, 因此你需要一个使用 loop 来密集地调用 try_recv()
    // 当然如果执行了上面的代码,因为已经收到了, 所以下面这个 loop 永远不会收到消息
    let start_time = Utc::now().timestamp();
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("{}", msg);
                break;
            }
            Err(_) => {
                let curr_time = Utc::now().timestamp();
                if curr_time - start_time >= 3 {
                    println!("no receive!");
                    break;
                }
            }
        }
    }
}

pub fn send_multi_values() {
    let (tx, rx) = create_channel();

    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    for received in rx {
        println!("Got: {}", received);
    }
}

// Got: more
// Got: hi
// Got: from
// Got: messages
// Got: the
// Got: for
// Got: you
// Got: thread
pub fn multi_senders_one_receiver() {
    let (tx, rx) = create_channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
