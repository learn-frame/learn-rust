fn main() {
    println!("Hello, cargo!");
}

// 构建与执行
// Rust 是一种预编译静态类型, 即编译和运行是彼此独立的步骤, 通过 `rustc main.rs` 会生成一个名叫 `main` 的二进制可执行文件.
// 如果你使用 cargo 管理项目(即使用 `cargo new hello_cargo`), 你可以通过 cargo build 来构建代码, 然后直接使用 `./target/debug/hello_cargo` 运行程序即可
// 当然也可以使用 cargo run 来编译 + 执行
// cargo check 用来手工检查代码是否能通过编译
// cargo build --release 用来构建发布版本
