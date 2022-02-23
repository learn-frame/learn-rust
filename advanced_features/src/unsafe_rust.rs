//! 以下五类是可以使用 unsafe 的
//! 解引用裸指针
//! 调用不安全的函数或方法
//! 访问或修改可变静态变量
//! 实现不安全 trait
//! 访问 union 的字段
//!
//! 要求这五类操作必须位于标记为 unsafe 的块中, 就能够知道任何与内存安全相关的错误必定位于 unsafe 块内
//! 为了尽可能隔离不安全代码, 将不安全代码封装进一个安全的抽象并提供安全 API 是一个好主意
use std::slice;

pub fn entry() {
    unsafe {
        dangerous();
    }

    let mut vec = [3; 19];
    let res = split_at_mut(&mut vec, 3);
    println!("{:?}", res);

    // use_unsafe_code(); // segmentation fault

    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", my_c_function(-3));
    // }

    visit_and_update_mut_static_varibles();

    let u = MyUnion { f1: 1 };
    unsafe { u.f1 };
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

// 复习悬垂引用 - 一个错误的例子
// pub fn dangle() -> &String {
//     // s 在 dangle 函数结束时已经被 drop 了
//     let s = String::from("hello");

//     // 😈: 因此你拿不到 s 的引用
//     &s
// }

// 复习悬垂引用 - 一个正确的例子
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
// unsafe {
//     dangerous();
// }

// split_at_mut 是标准库中的一个方法, 用户返回一个从原始 slice 的开头直到 mid 索引, 另一个从 mid 直到原 slice 的结尾的元组
// 但下面的实现中, Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分
// 它只知道我们借用了同一个 slice 两次, 本质上借用 slice 的不同部分是可以的, 因为结果两个 slice 不会重叠, 不过 Rust 还没有智能到能够理解这些
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     // Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分
//     (&mut slice[..mid], &mut slice[mid..])
// }

// 无需将 split_at_mut 函数的结果标记为 unsafe, 并可以在安全 Rust 中调用此函数
// 换句话说, 虽然用了裸指针这种不安全的做法, 但通过一系列操作, 保证了局部不安全, 整体安全
fn split_at_mut<T>(vec: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = vec.len();
    // as_mut_ptr 返回一个 *mut i32 类型的裸指针
    let ptr = vec.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice
            // 从开头截取到 mid, 因为有了 mid 必然小于等于 len 的断言, 保证不会溢出
            slice::from_raw_parts_mut(ptr, mid),
            // 从 mid 截取到最后, 保证不会溢出
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 如果不加任何限制, 使用臆测为有效的 slice 会导致未定义的行为
pub fn use_unsafe_code() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", slice);
}

// extern 用于集成其他语言的接口, 称作外部函数接口(Foreign Function Interface, FFI)
// 其他语言的接口总是不安全的, 你需要包裹在 unsafe 语句块中执行
// "C" 部分定义了外部函数所使用的应用二进制接口(application binary interface, ABI)
extern "C" {
    #[allow(unused)]
    fn my_c_function(input: i32) -> i32;
}

// 从其它语言调用 Rust 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 全局变量在 Rust 中被称为静态变量, 静态变量只能储存拥有 'static 生命周期的引用
// 这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注. 访问不可变静态变量是安全的
// 静态变量和常量的区别:
// 1. 静态变量中的值有一个固定的内存地址, 使用这个值总是会访问相同的地址
// 2. 常量则允许在任何被用到的时候复制其数据
// 3. 常量不可改变而静态变量可以改变, 但访问和修改可变静态变量都是不安全的
static STATIC_VARIABLE: &str = "Hello, world!";

// 拥有可以全局访问的可变数据, 难以保证不存在数据竞争, 尤其是在多线程,
// 这就是为何 Rust 认为可变静态变量是不安全的
static mut MUT_STATIC_VARIABLE: &str = "Hello, world!";

fn visit_and_update_mut_static_varibles() {
    println!("{}", STATIC_VARIABLE);

    unsafe {
        println!("{}", MUT_STATIC_VARIABLE);

        MUT_STATIC_VARIABLE = "Hi, world!";
    }
}

unsafe trait Foo {
    // methods go here
}

// 如果一个 trait 被标记了 unsafe, 它的实现也是 unsafe 的
unsafe impl Foo for i32 {
    // method implementations go here
}

// 联合体类型
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
