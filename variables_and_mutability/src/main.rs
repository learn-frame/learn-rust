use chrono::{DateTime, Local};

fn main() {
    let num: i128 = 1;
    println!("我给长者+{0}s", num);

    // 与 js 不同, let 也不可重新赋值
    // cannot assign twice to immutable variable `num`
    // num = 2;

    // 除非使用 mut
    let mut str = "狗子你变了";
    str = "狗子你没变";
    println!("{}", str);

    // 当然你仍然不能把它重新赋值给一个其他类型
    // str = 1;

    // 而对于 const, 必须是编译时常量
    const BASE_URL: &str = "https://yanceyleo.com";
    println!("{}", BASE_URL);

    // 像 Local::now() 的就不是编译时常量, 不能使用 const, 这点跟 Dart 一样
    // const LOCAL_TIME: DateTime<Local> = Local::now();
    let LOCAL_TIME = Local::now();
}
