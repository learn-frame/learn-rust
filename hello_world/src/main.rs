use ferris_says::say;
use std::io::{stdout, BufWriter};

// main 函数是一个特殊的函数, 在可执行的 Rust 程序中, 它总是最先运行的代码
fn main() {
    // println 是个普通的函数, 后面跟上了 ! 的就叫做"宏(macro)"
    // 这句话相当于把字符串 `Hello, world!`作为参数传递给宏 println!
    println!("Hello, world!");
    print_ferris();

    // println 有多种参数:
    println!("{}", 2); // 大括号里没东西代表 Display, 后面的值必须实现 Display trait.
    println!("{:?}", 2); // ? 代表 Debug
    println!("{:o}", 2); // o 代表八进制
    println!("{:x}", 2); // x 代表十六进制小写
    println!("{:X}", 2); // X 代表十六进制大写
    println!("{:p}", ""); // p 代表指针
    println!("{:b}", 2); // b 代表二进制
    println!("{:e}", 2); // e 代表指数小写
    println!("{:E}", 2); // b 代表指数大写
}

fn print_ferris() {
    let stdout = stdout();
    let message = String::from("垃圾螃蟹!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
