//! 并发编程(Concurrent programming), 代表程序的不同部分相互独立的执行,
//! 而并行编程(parallel programming) 代表程序不同部分于同时执行

mod extensible_concurrency_with_sync_and_send;
mod message_passing;
mod shared_state;
mod threads;

fn main() {
    threads::entry();
    message_passing::entry();
    shared_state::entry();
    extensible_concurrency_with_sync_and_send::entry();
}
