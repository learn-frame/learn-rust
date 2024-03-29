//! 所有权的存在就是为了管理堆数据
//! 因为访问堆上的数据比访问栈上的数据慢, 因为必须通过指针来访问; 并且在堆上分配大量的空间也可能消耗时间
//! 所以, 所有权的功能就是跟踪哪部分代码正在使用堆上的哪些数据, 最大限度的减少堆上的重复数据的数量, 以及清理堆上不再使用的数据确保不会耗尽空间
//!
//! 所有权的规则:
//! Rust 中的每一个值都有一个称为其所有者(owner)的变量.
//! 值在任一时刻有且只有一个所有者.
//! 当所有者(变量)离开作用域, 这个值将被丢弃.
pub mod references_and_borrowing;
pub mod return_value;
pub mod slice;

fn main() {
    // String 有两种方式
    // 一种是这种字面量形式, 它是一种硬编码的形式
    // 一是它们是不可变的, 二是并非所有字符串的值都能在编写代码时就知道(用户输入)
    let str_0 = "hello";
    println!("{}", str_0);

    // 另一种是通过 String 来创建实例
    let mut str_1 = String::new();
    // 有丰富的字符串方法
    str_1.push_str("yancey");
    println!("{}", str_1);

    // String::from 可以传入一个初始值
    let mut str_2 = String::from("intital");
    str_2.push_str(" repo");
    println!("{}", str_2);

    // 如上使用 String 创建的字符串, 是存储到堆上的, 而字符串字面量存储在栈
    // 就字符串字面值来说, 我们在编译时就知道其内容, 所以文本被直接硬编码进最终的可执行文件中
    // 对于 String 类型, 为了支持一个可变, 可增长的文本片段, 需要在堆上分配一块在编译时未知大小的内存来存放内容, 这意味着:
    // 必须在运行时向内存分配器(memory allocator)请求内存.
    // 需要一个当我们处理完 String 时将内存返回给分配器的方法.

    // 类似于垃圾回收, 内存在拥有它的变量离开作用域后就被自动释放
    // 当 s 离开作用域的时候. 当变量离开作用域, Rust 为我们调用一个特殊的函数.
    // 这个函数叫做 drop, 在这里 String 的作者可以放置释放内存的代码. Rust 在结尾的 } 处自动调用 drop
    {
        let s = String::from("hello"); // 从此处起, s 是有效的
        println!("{}", s);

        // 使用 s
    } // 此作用域已结束

    // 对于字符串字面量, x y 都是栈内存
    // x y 是独立的
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // 而对于 new 出来的, x_1 和 y_1 指向一块堆内存
    // 为了防止回收被执行两次(二次释放可能引起 bug), 所以 rust 的所有权是这样做的:
    // 既然 x_1 已经给了 y_1, 那 x_1 就废了
    // 所以你在下面不能访问到 x_1 了
    let x_1 = String::new();
    let y_1 = x_1;
    // 👿 报错, 不能访问到 x_1
    // println!("{}", x_1);
    println!("{}", y_1);

    // 如果我们确实要能访问, 可以使用 clone
    let x_3 = String::from("hello");
    let x_4 = x_3.clone();
    println!("x_3: {}, x_4: {}", x_3, x_4);

    // 所有权与函数
    let str = String::from("string...");
    foo(str);
    // 👿 报错, 上面 foo 函数用了 str 实参, 下面就不能访问了, 因为这个 str 不是 Copy 的
    // println!("{}", str);

    let str_1 = "Hello";
    bar(str_1);
    // &str 类型的字符串就可以
    println!("{}", str_1);
}

fn foo(name: String) {
    println!("{}", name);
} // name 移出作用域并调用 `drop` 方法. 占用的内存被释放

fn bar(name: &str) {
    println!("{}", name);
    // name 仅仅是移出作用域
}

pub fn func() {
    let str = String::from("ccc");
    // {:?} 用来获取引用的指针, 如果不是引用, 在编译时不会报错, 在运行时会报错
    println!("{:p}", &str);
}

#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop for {}", self.0);
    }
}
// 对于变量遮蔽, 并不是第二 x 的声明会直接把第一个 x 给 drop 掉, 具体打印如下:
// create x: S(1)
// create shadowing x: S(2)
// drop for 2
// drop for 1
#[allow(unused)]
fn varible_shadowing_drop() {
    let x = S(1);
    println!("create x: {:?}", x);
    let x = S(2);
    println!("create shadowing x: {:?}", x);
}

// Copy trait
//
// 所有整数类型, 比如 u32.
// 布尔类型, bool, 它的值是 true 和 false.
// 所有浮点数类型, 比如 f64.
// 字符类型, char.
// 元组, 当且仅当其包含的类型也都实现 Copy 的时候. 比如, (i32, i32) 实现了 Copy, 但 (i32, String) 就没有.

// 总结:
//
// 变量的所有权总是遵循相同的模式: 将值赋给另一个变量时移动它.
// 当持有堆中数据值的变量离开作用域时, 其值将通过 drop 被清理掉, 除非数据被移动为另一个变量所有.
