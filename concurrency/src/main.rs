//! 并发编程(Concurrent programming), 代表程序的不同部分相互独立的执行,
//! 而并行编程(parallel programming) 代表程序不同部分于同时执行

pub mod message_passing;
pub mod shared_state;
pub mod threads;

fn main() {
    threads::entry();
    message_passing::entry();
    shared_state::entry();
}
