/// Packages: Cargo 的一个功能, 它允许你构建, 测试和分享 crate.
/// Crates: 一个模块的树形结构, 它形成了库或二进制项目.
/// Modules and use: 允许你控制作用域和路径的私有性.
/// Path: 一个命名例如结构体, 函数或模块等项的方式
///
/// package 就是 packages_crate_modules 这个文件夹, 它有一个 Cargo.toml, 类似于 package.json
/// 此外, 它还有一个 src 文件夹, 里面有若干个 .rs 文件, 这些文件称为二进制 crate
/// src 有两个强制命名的文件, main.rs 和 lib.rs, 如果是业务代码, 可以用 main.rs 作为整个项目的根
/// 如果你在造轮子, 就用 lib.rs 作为整个库的根
// 引用 front_of_house 模块
mod front_of_house;

mod cooking;

mod use_struct;

// 指定使用 front_of_house 模块中的 hosting 子模块
// 😈: 一定注意, 虽然 hosting 在 front_of_house 文件中是 pub 的
// 但如果你在本文件引用时不加 pub, 它意味着在本文件中是私有的
// pub use 叫做重导出
pub use crate::front_of_house::hosting;

pub use crate::use_struct::back_of_the_house::Breakfast;

pub use crate::use_struct::back_of_the_house::Appetizer;

pub use crate::cooking::cooking_type;

pub fn eat_at_restaurant() {
    // 通过绝对路径引用模块(rust 推荐使用绝对路径来引用)
    crate::front_of_house::hosting::hello();

    // 通过相对路径引用模块
    front_of_house::hosting::hello();

    // 通过 use 来引用
    hosting::hello();
    cooking_type::炸();

    let mut instance = Breakfast::summer("Rye");
    instance.toast = String::from("fuck");

    // 只要给枚举加上 pub, 它所有的成员都是 pub 的
    let a = Appetizer::Salad;
    let b = Appetizer::Soup;
}

// 下面两个模块都有 Result, 你不能在一个文件里这样引用
// 因为重名了, 你可以使用 as 来换个名字, 这点跟 JS 一致
use std::fmt::Result;
use std::io::Result as IOResult;

// 如下三个引用
// use std::cmp::Ordering;
// use std::io;
// use std::io::Write;
// 可以缩成一个
use std::{self, cmp::Ordering, io::Write};

// 如果希望将一个路径下所有公有项引入作用域, 可以指定路径后跟 *
use std::collections::*;
