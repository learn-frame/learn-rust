//! 多线程会导致下列问题:
//! 竞争状态(Race conditions), 多个线程以不一致的顺序访问数据或资源
//! 死锁(Deadlocks), 两个线程相互等待对方停止使用其所拥有的资源, 这会阻止它们继续运行
//! 只会发生在特定情况且难以稳定重现和修复的 bug
//!
//! 很多操作系统提供了创建新线程的 API. 这种由编程语言调用操作系统 API
//! 创建线程的模型有时被称为 1:1, 一个 OS 线程对应一个语言线程.
//!
//! 很多编程语言提供了自己特殊的线程实现, 编程语言提供的线程被称为绿色(green)线程,
//! 使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们. 为此, 绿色线程模式
//! 被称为 M:N 模型: M 个绿色线程对应 N 个 OS 线程, 这里 M 和 N 不必相同.
use std::thread;
use std::time::Duration;

pub fn entry() {
    create_new_thread();
    wait_for_all_thread();
    wait_for_all_thread1();
    use_move();
}

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
pub fn create_new_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 稍微复习下: 1..5 等价于 [1, 2, 3, 4]
    // 当主线程结束时, 新线程也会结束, 而不管其是否执行完毕
    // 因此主线程走完 4 之后就结束了, 新线程走一次 5 就结束了
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 2 from the main thread!
// hi number 3 from the spawned thread!
// hi number 3 from the main thread!
// hi number 4 from the spawned thread!
// hi number 4 from the main thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
pub fn wait_for_all_thread() {
    // thread::spawn 的返回值类型是 JoinHandle.
    // JoinHandle 是一个拥有所有权的值, 当对其调用 join 方法时, 它会等待其线程结束
    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束.
    // 阻塞(Blocking) 线程意味着阻止该线程执行工作或退出. 因为我们将 join 调用放在了主线程的 for 循环之后
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 3 from the main thread!
// hi number 4 from the main thread!
pub fn wait_for_all_thread1() {
    // 主线程会等待直到新建线程执行完毕之后才开始执行 for 循环,
    // 所以输出将不会交替出现
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn use_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // Rust 不知道这个新建线程会执行多久, 所以无法知晓 v 的引用是否一直有效
        // 使用 move 关键字强制获取它使用的值的所有权
        println!("{:?}", v); // [1, 2, 3]
    });

    handle.join().unwrap();
}
