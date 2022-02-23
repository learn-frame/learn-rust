//! 宏(Macro)指的是 Rust 中一系列的功能
//! 1. 使用 macro_rules! 的声明宏(declarative macros, 有时也叫 macros by example).
//! 2. 三种 过程宏(procedural macros):
//!  - 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
//!  - 类属性(Attribute-like)宏定义可用于任意项的自定义属性
//!  - 类函数宏看起来像函数不过作用于作为参数传递的 token
//!
//! 宏是一种为写其他代码而写代码的方式, 即所谓的元编程(metaprogramming)
//! 我们曾经用过 println! 宏和 vec! 宏, 这些宏以**展开**的方式来生成比你所手写出的更多的代码
//!
//! Rust 中的函数不支持扩展参数, 但宏能够接受不同数量的参数
//! 比如 println!("hello"); println!("{}", "hello");
//!
//! 宏可以在编译器翻译代码前展开, 例如, 宏可以在一个给定类型上实现 trait.
//! 而函数则不行, 因为函数是在运行时被调用, 同时 trait 需要在编译时实现.
//!
//! 在一个文件里调用宏之前必须定义它(可以类比 ES6 的临时死区), 或将其引入作用域, 而函数则可以在任何地方定义和调用

use hello_macro::HelloMacro;
use hello_macro_derive::{make_answer, route, HelloMacro};

/* 声明宏 */
// 不过后面 macro_rules! 这种模式就被淘汰了, 艹
#[macro_export]
macro_rules! my_vec {
    // $x:expr 匹配 Rust 的任意表达式, 并将该表达式记作 $x
    // ( $x:expr ) 后面跟着的逗号意味着一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后
    ( $( $x:expr ),* ) => {
        {
            // 创建一个空 Vector
            let mut temp_vec = Vec::new();
            $(
                // 每传一次值, 就往这个临时 vector push 一次
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/* 用于从属性生成代码的过程宏 */

/// 1. 自定义 #[derive] 宏
#[derive(HelloMacro)]
struct Pancakes;

/// 2. 类属性宏

// enum Method {
//     GET,
//     POST,
// }

#[route(Method::GET, "/api/users")]
fn get_users() {}

pub fn entry() {
    my_vec![1, 2, 3];
    Pancakes::hello_macro(); // Hello, Macro! My name is Pancakes!
    get_users();
    make_answer!(); // 3. 类函数宏
}
