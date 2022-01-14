//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

// //! 开头的注释用于做综述, 来描述整个文档的功能
// /// 开头的注释叫做文档注释, 里面可以手写 markdown 语法
// 通过 cargo doc --open 生成文档

// 通过 cargo login CARGO_REGISTRY_TOKEN 将 token 存到 ~/.cargo/credentials
// 通过 cargo publish 就可以发包了
// 通过手动修改版本号, 再使用 cargo publish 更新发版
// 通过 cargo yank --vers 1.0.1 撤回一个版本
// 通过 cargo yank --vers 1.0.1 --undo 将撤回的版本再发布上去
// 通过 cargo install ripgrep 安装包

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = more_about_cargo::add_one(arg);
///
/// // 文档注释中的测试语句是可以被 cargo test 测到的.
/// assert_eq!(6, answer);
/// ```
///
/// # Panic
/// 这个函数可能会 panic! 的场景. 并不希望程序崩溃的函数调用者应该确保他们不
/// 会在这些情况下调用此函数.
///
/// # Errors
/// 如果这个函数返回 Result, 此部分描述可能会出现何种错误以及什么情况会造成
/// 这些错误, 这有助于调用者编写代码来采用不同的方式处理不同的错误.
///
/// # Safety
/// 如果这个函数使用 unsafe 代码(这会在第十九章讨论), 这一部分应该会涉及到期
/// 望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件(invariants).
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = more_about_cargo::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

// 使用 pub use 导出合适的公有 API
pub mod kinds {
    /// The primary colors according to the RYB color model.

    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!("{:?} {:?}", c1, c2);
        SecondaryColor::Green
    }
}

// 其他用户引用了你的包, 就可以这么用了
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// }

// 当然上面这种结构仍然很复杂, 你可以重导出
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

// 那用户用起来就简单多了
// use art::PrimaryColor;
// use art::mix;

// fn main() {
//     // --snip--
// }
