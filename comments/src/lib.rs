//! # 整个文档的说明
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
//! 更多关于注释/文档可查看 more_about_cargo 这一章节

/// [某个方法/功能的说明] Adds one to the number given.
///
/// # Panics
///
/// 并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数.
///
/// # Errors
///
/// 如果这个函数返回 Result, 此部分描述可能会出现何种错误以及什么情况会造成这些错误, 这有助于调用者编写代码来采用不同的方式处理不同的错误
///
/// # Safety
///
/// 如果这个函数使用 unsafe 代码, 这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件(invariants)
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = comments::add_one(arg);
///
/// // 可通过 cargo test 来测试示例代码块
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// [某个方法/功能的说明] Say hello.
pub fn hello(x: String) -> String {
    format!("Hello, {}!", x)
}
