// grep 最简单的使用场景是在特定文件中搜索指定字符串
// grep 获取一个文件名和一个字符串作为参数, 接着读取文件并找到其中包含字符串参数的行, 然后打印出这些行

// 逻辑提取到了 src/lib.rs
// 并将所有的参数解析和错误处理留在了 src/main.rs 中

// 大部分终端都提供了两种输出: 标准输出(standard output, stdout)对应一般信息,
// 标准错误(standard error, stderr), 而 println! 只能输出到标准输出

extern crate dotenv;

use dotenv::dotenv;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    dotenv().ok();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
