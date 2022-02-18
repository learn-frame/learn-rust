//! 以下五类是可以使用 unsafe 的
//! 解引用裸指针
//! 调用不安全的函数或方法
//! 访问或修改可变静态变量
//! 实现不安全 trait
//! 访问 union 的字段
//!
//! 要求这五类操作必须位于标记为 unsafe 的块中, 就能够知道任何与内存安全相关的错误必定位于 unsafe 块内
//! 为了尽可能隔离不安全代码, 将不安全代码封装进一个安全的抽象并提供安全 API 是一个好主意

pub fn entry() {
    unsafe {
        dangerous();
    }

    split_at_mut();
}

/// Rust 会在编译时检测悬垂引用, 而不安全的裸指针(raw pointer)可以跳过这个问题
/// 不可变裸指针: *const T
/// 可变裸指针: *mut T
///
/// 裸指针和普通引用, 智能指针的区别有:
/// 允许忽略借用规则, 可以同时拥有不可变和可变的指针, 或多个指向相同位置的可变指针
/// 不保证指向有效的内存
/// 允许为空
/// 不能实现任何自动清理功能
pub fn dereferencing_a_raw_pointer() {
    let mut num = 5;

    // 这里没有引入 unsafe 关键字.
    // 可以在安全代码中创建裸指针, 只是不能在不安全块之外解引用裸指针
    let r1 = &num as *const i32; // 使用 as 将不可变和可变引用强转为对应的裸指针类型
    let r2 = &mut num as *mut i32;

    // 创建一个不能确定其有效性的裸指针
    // 尝试使用任意内存是未定义行为: 此地址可能有数据也可能没有, 编译器可能会优化掉这个内存访问, 或者程序可能会出现段错误(segmentation fault)
    // 一般也不会这么写
    let address = 0x012345usize;
    let r = address as *const i32;

    // 可以在安全代码中创建裸指针, 不过不能解引用裸指针和读取其指向的数据
    // 如果确实需要解引用, 需要包在 unsafe 块中
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r is: {}", *r);
    }

    // 稍微复习下, 同时引用一个变量的可变引用和非可变引用可能会引发错误
    // 因为 Rust 的所有权规则不允许在拥有任何不可变引用的同时再创建一个可变引用
    // 但 unsafe 可以, 所以要小心潜在造成数据的竞争
    // let r3 = &num;
    // let r4 = &mut num;
    // println!("{} {}", r3, r4);
}

// pub fn dangle() -> &String {
//     // s 在 dangle 函数结束时已经被 drop 了
//     let s = String::from("hello");

//     // 😈: 因此你拿不到 s 的引用
//     &s
// }

pub fn anti_dangle() -> String {
    let s = String::from("hello");
    // 所有权被移动出去, 所以没有值被释放
    s
}

// 裸指针虽然很危险, 但它也有它的用途:
// 1. 调用 C 代码接口(C 语言: mmp!)
// 2. 构建借用检查器无法理解的安全抽象

// unsafe 的函数必须用 unsafe 代码块来调用
unsafe fn dangerous() {}

// 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的.
// 事实上, 将不安全代码封装进安全函数是一个常见的抽象.
fn split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    println!("{:?} {:?}", a, b);
}
