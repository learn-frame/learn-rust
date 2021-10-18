fn main() {
    // rust 中的 let 变量允许用一个新值来隐藏(shadow) 旧值
    // 这跟 JavaScript 是不同的
    let x = 0;
    let x = "";

    // 然而变量默认是不可变的, 因此你不能重新赋值
    let y = 0;
    y = 1;
}
