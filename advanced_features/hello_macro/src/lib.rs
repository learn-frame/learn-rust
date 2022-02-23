pub trait HelloMacro {
    fn hello_macro();
}

// 我们想实现一个通用的, 如下的功能
// 即给一个的指定 struct, 能够给它自动包装上 HelloMacro 的实现
// 说白了就是给 struct 加个通用注解, 以实现 HelloMacro 这个 trait
// 
// use hello_macro::HelloMacro;
//
// struct Pancakes;
//
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }
//
// fn main() {
//     Pancakes::hello_macro();
// }