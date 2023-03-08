mod closures;
mod iterators;

// 函数式编程风格通常包含将函数作为参数值或其他函数的返回值,
// 将函数赋值给变量以供之后执行等等
fn main() {
    iterators::entry();
    closures::entry();
}
