use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

fn main() {
    foo(11, "");
    bar();
    plus_one();
    generate_vec();
    multi_same_name_fn();
    exec_f();
    match_with_ref();
    swap_tuple((1, ""));

    // 泛型函数
    square(1, 4);
    // 使用 turbofish
    square::<u32>(3, 2);
    use_fn_as_params();

    let f = use_fn_as_return_val();
    println!("{:?}", f(1, 1));
}

fn multi_same_name_fn() {
    #[allow(unused)]
    fn a() {}

    // 😈: 不能在相同词法作用域下重复定义同名的函数
    // fn a() {}

    // 但在不同词法作用域可以定义同名函数
    {
        #[allow(unused)]
        fn a() {}
    }
}

// rust 不关心函数定义的位置, 定义了就行
// rust 的函数不支持默认参数和不定参数, 草, 也没箭头函数
// 必须显式指定参数
// 如果有返回值, 必须显式标明返回值的类型
fn foo(x: i32, y: &str) -> i32 {
    // 不能把 let 语句赋值给另一个变量
    // let c = (let d = 6);

    println!("Hello, {}, {}!", x, y);
    return 1;
}

// 对于没有返回值, 返回值类型可以不写, 也可以写成 -> ()
// () 叫做单元类型, 单元类型拥有唯一的值, 就是它本身
fn bar() -> () {
    let x = 5;
    println!("{}", x);

    // 这叫做表达式, 大括号之间形成了作用域
    // x + 1 相当于 return x + 1;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // 4
}

// 等价于 return 5;
// 注意 5 不能加分号, 因为下面是一个表达式, 而不能是个语句.
fn plus_one() -> i32 {
    5
}

// rust 函数不支持重载
// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }

// 函数也可以是常量
// 这叫做编译时函数执行 Compile-TimeFunction Execution, 简称 CTFE
const fn give_me_five(n: usize) -> usize {
    5 + n
}

fn generate_vec() -> [i32; 6] {
    // Rust 中固定长度数组的初始值和长度都必须在编译时是确定的
    // 如果想传入一个函数, 必须使用常量函数
    [0; give_me_five(1)]
}

// 理论上你不能用关键字来当作变量名, 除非你使用原始标识符(Raw identifiers)
pub fn r#match(needle: &str, haystack: &str) -> bool {
    let r#let = 1;
    println!("{}", r#let);

    haystack.contains(needle)
}

// Rust 编译器在解析代码的时候, 如果碰到分号, 就会继续往后面执行; 如果碰到语旬, 则执行语句; 如果碰到表达式, 则会会对表达式求值, 如果分号后面什么都没有, 就会补上单元值 ()
// 当遇到函数的时候, 会将函数体的花括号识别为块表达式(Block Expression). 块表达式是由一对花括号和一系列表达式组成的, 它总是返回块中最后一个表达式的值.

/// 函数参数模式匹配
///
/// & vs ref
/// & denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
/// ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
///
#[allow(unused)]
#[derive(Debug)]
struct S {
    i: i32,
}
fn f(ref s: S) {
    println!("{:p}", s);
}
fn f1(s: &S) {
    println!("{:p}", s);
}
fn exec_f() {
    let s = S { i: 1 };
    f(s);

    // 😈: ref 跟 & 不一样, 拿的不是引用, s 仍然会被 move 到 f1 中
    // println!("{:?}", s);

    let s = S { i: 1 };
    f1(&s);
    println!("{:?}", s);
}

fn match_with_ref() {
    let s = Some("abc".to_string());
    match s {
        Some(t) => println!("{:?}", t),
        None => (),
    }
    // s 被 move 到 match 语句中了, 下面就不能获取了
    // println!("{:?}", s);

    // 可以这样
    let s = Some("abc".to_string());
    match s {
        Some(ref t) => println!("{:?}", t),
        None => (),
    }
    println!("{:?}", s);

    // 也可以这样
    let s = Some("abc".to_string());
    match &s {
        // 因为从语法上, 不允许写 Some(&t) => (), 只能写成 Some(ref t) => ()
        // 或者干脆直接 match &s
        Some(t) => println!("{:?}", t),
        None => (),
    }
    println!("{:?}", s);
}

// 参数可以使用解构的元祖
fn swap_tuple<T, S>((x, y): (T, S)) -> (S, T) {
    (y, x)
}

// 泛型函数
fn square<T: Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

/// 高阶函数, 就是函数的参数是函数
fn math<T>(op: fn(T, T) -> T, a: T, b: T) -> T {
    op(a, b)
}

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn product<T>(a: T, b: T) -> T
where
    T: Mul<Output = T>,
{
    a * b
}

fn use_fn_as_params() {
    assert_eq!(3, math(sum, 1, 2));
    assert_eq!(2, math(product, 1, 2));
}

// function pointer 可以用 type 关键字提取出来
// 但泛型似乎做不了 function pointer, 留个 // TODO:
type Op = fn(i32, i32) -> i32;
fn use_fn_as_return_val() -> Op {
    fn math(a: i32, b: i32) -> i32 {
        a + b
    }

    math
}
