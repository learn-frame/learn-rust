/// 在很多情况下, Rust 要求你承认出错的可能性, 并在编译代码之前就采取行动
/// Rust 将错误组合成两个主要类别: 可恢复错误(recoverable) 和不可恢复错误(unrecoverable)
/// 可恢复错误通常代表向用户报告错误和重试操作是合理的情况, 比如未找到文件
/// 不可恢复错误其实就是 bug, 比如尝试访问超过数组结尾的位置
///
/// 大大部分语言并不区分这两类错误, 并采用类似异常这样方式统一处理他们.
/// Rust 并没有异常, 但是, 有可恢复错误 Result<T, E> , 和不可恢复(遇到错误时停止程序执行)错误 panic!

pub fn unrecoverable_errors_with_panic() {
    // thread 'main' panicked at 'crash and burn', src/main.rs:12:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // panic!("crash and burn");

    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:17:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let v = vec![1, 2, 3];
    v[99];

    // 通过 `RUST_BACKTRACE=1 cargo run` 可以拿到回溯的堆栈信息
}
