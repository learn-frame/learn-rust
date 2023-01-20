use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;

// 很多方法实现了 Result
// 即结果成功了就走 OK, 失败了走 Err
// 跟 try...catch 差不多
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub fn entry() {
    basic_result();
    use_match_to_handling_file_result();
    handle_specific_error();
    read_file_by_closure();
    panic_simple();

    // Result 有两个参数, 是个泛型
    // 第一个是 Ok 的返回值, 第二个是 Err 的返回值
    let p = propagating_errors();
    match p {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn basic_result() {
    let mut result: Result<i32, &str> = Ok(1);
    assert!(result.is_ok());
    result = Err("error");
    assert!(result.is_err());
}

pub fn use_match_to_handling_file_result() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

pub fn handle_specific_error() {
    let r_f = File::open("hello.txt");

    match r_f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 匹配指定错误类型
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 这是一种闭包的实现方式, 更优雅
pub fn read_file_by_closure() {
    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn panic_simple() {
    let f = File::open("hello.txt");

    // unwrap 和 expect 都是返回文件句柄(成功时)或调用 panic! 宏(失败时)
    // 不同的是, unwrap 失败给的是默认 error message, expect 可以自己传一个失败时的文案
    #[allow(unused)]
    let success = f.as_ref().unwrap();
    #[allow(unused)]
    let error = f.as_ref().expect("Failed to open hello.txt");
}

// -----------------------
// 下面三种本质是一样的, 但越来越简单, 仔细体会

// 使用纯粹的 match 样板代码
#[allow(unused)]
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 运算符是一个错误处理的语法糖, 它会自动在出现错误的情况下返回 std::io::Error.
// 这样就可以在程序发生错误时自动返回错误码, 并在退出程序时打印相关的错误信息, 方便调试, 而不需要开发者手动处理错误了
pub fn propagating_errors() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 跟上面一样, 这个更简单
#[allow(unused)]
pub fn simple_propagating_errors() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// main 函数是特殊的, 其必须返回什么类型是有限制的
// Box<dyn Error> 被称为 trait 对象(rait objec)
// 目前可以理解 Box<dyn Error> 为使用 ? 时 main 允许返回的任何类型的错误
// 十七章解释道 Box 用来当有一个在编译时未知大小的类型, 而又想要在需要确切大小的上下文中使用这个类型值的时候
#[allow(unused)]
fn main() -> Result<(), Box<dyn Error>> {
    File::open("hello.txt")?;

    Ok(())
}
