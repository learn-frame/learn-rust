//! 互斥器(mutex) 是 mutual exclusion 的缩写, 也就是说, 任意时刻, 其只允许一个线程访问某些数据.
//! 为了访问互斥器中的数据, 线程首先需要通过获取互斥器的 锁(lock) 来表明其希望访问数据.
//! 锁是一个作为互斥器一部分的数据结构, 它记录谁有数据的排他访问权. 因此, 我们描述互斥器为通过锁系统 保护(guarding) 其数据.
//!
//! 互斥器以难以使用著称, 因为你不得不记住:
//! 1. 在使用数据之前尝试获取锁.
//! 2. 处理完被互斥器所保护的数据之后, 必须解锁数据, 这样其他线程才能够获取锁.

use std::sync::{Arc, Mutex};
use std::thread;

pub fn entry() {
    learn_mutex();
    multi_mutex();
}

pub fn learn_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn multi_mutex() {
    // 原子引用计数 Arc<T>, 可以理解为多线程级别的 Rc<T>
    // 因为 counter 锁的所有权不能直接移动到多个线程中, 所以要祭出 Rc<T>
    // 但 Rc<T> 不是线程安全的, 所以需要用 Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Result: 10
}

// 两个 Rc<T> 值相互引用, 造成内存泄漏.
// 同理, Mutex<T> 也有造成 死锁(deadlock) 的风险. 这发生于当一个操作需要锁住两个资源而两个线程各持一个锁, 这会造成它们永远相互等待
// TODO: 标准库中 Mutex<T> 和 MutexGuard 的 API 文档有这种
