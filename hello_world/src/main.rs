use ferris_says::say;
use std::io::{stdout, BufWriter};

// main 函数是一个特殊的函数, 在可执行的 Rust 程序中, 它总是最先运行的代码
fn main() {
    // println 是个普通的函数, 后面跟上了 ! 的就叫做"宏(macro)"
    // 这句话相当于把字符串 `Hello, world!`作为参数传递给宏 println!
    println!("Hello, world!");
    print_ferris();
}

fn print_ferris() {
    let stdout = stdout();
    let message = String::from("垃圾螃蟹!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
