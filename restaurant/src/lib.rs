/// Packages: Cargo 的一个功能, 它允许你构建, 测试和分享 crate.
/// Crates: 一个模块的树形结构, 它形成了库或二进制项目.
/// Modules and use: 允许你控制作用域和路径的私有性.
/// Path: 一个命名例如结构体, 函数或模块等项的方式
///
/// package 就是 packages_crate_modules 这个文件夹, 它有一个 Cargo.toml, 类似于 package.json
/// 此外, 它还有一个 src 文件夹, 里面有若干个 .rs 文件, 这些文件称为二进制 crate
/// src 有两个强制命名的文件, main.rs 和 lib.rs, 如果是业务代码, 可以用 main.rs 作为整个项目的根
/// 如果你在造轮子, 就用 lib.rs 作为整个库的根
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
