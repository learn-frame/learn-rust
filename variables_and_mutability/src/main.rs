fn main() {
    // rust 中的 let 变量允许用一个新值来隐藏(shadow) 旧值, 并且可以改变其数据类型
    let x = 0;
    let x = "";

    // 👿 然而变量默认是不可变的, 因此你不能重新赋值
    let y = 0;
    // y = 1;

    // 除非你加上 mut, 表示这个变量的值是可变的
    let mut z = 1.0;
    let mut z = z + 2.0;
    z = 3.0;
    // 👿 需要注意, 虽然值是可变的, 但它的类型是不可变的, 如不能从 float64 变成字符串
    // z = "";

    // 你必须为常量提供它的类型
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // 👿 常量不具备 shadow 效果
    // const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;

    const BASE_URL: &str = "https://api.yanceyleo.com";
}

// 总结:
// let 可改变数据类型
// let mut 不可改变数据类型
// let 和 let mut 都可以 shadow
// const 必须显式标明数据类型
