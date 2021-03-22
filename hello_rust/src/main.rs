extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = "垃圾螃蟹!".as_bytes();
    let width = 12;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();

    // 通常情况下, `{}` 会被任意变量内容所替换.
    // 变量内容会转化成字符串.
    println!("{} days", 31);

    // 位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}
