//! Rust 中的值默认被分配到栈内存, 可以通过 Box<T> 将值装箱(在堆内存中分配)
//! Box<T> 是指向类型为T的堆内存分配值的智能指针
//! 当 Box<T> 超出作用域范围时, 将调用其析构函数, 销毁内部对象, 并自动动释放堆中的内存
//! 可以通过解引用操作符来获取 Box<T> 中的 T
//! 看得出来, Box<T>的行为像引用, 并且可以自动释放内存, 所以我们称其为智能指针 

//! Box<T> 的特点:
//! Box 允许你将一个值放在堆上而不是栈上
//! 当有一个在编译时未知大小的类型, 而又想要在需要确切大小的上下文中使用这个类型值的时候
//! 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
//! 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
//!
use List::{Node, Nil};

pub fn entry() {
    // 在堆上存储一个 i32
    let b = Box::new(1);
    // 可以直接通过 * 拿到原始值
    println!("{}", *b);

    make_list();
}

// Rust 需要在编译时知道类型占用多少空间
// 一种无法在编译时知道大小的类型是递归类型
// smart_points::box_pointer
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

/// 因为这是一个递归枚举, 所以 Rust 会认为它"有无限的大小"
/// 其原因是 List 的一个成员被定义为是递归的: 它直接存放了另一个相同类型的值.
/// 这意味着 Rust 无法计算为了存放 List 值到底需要多少空间
/// ![box_pointer_1](https://edge.yancey.app/beg/ykt4jzph-1642414384386.png)
/// 因此你需要使用 Box<T> 给递归类型一个已知的大小
/// 因为 Box<T> 是一个指针, 我们总是知道它需要多少空间: 指针的大小并不会根据其指向的数据量而改变
/// ![box_pointer_2](https://edge.yancey.app/beg/5i3qqkhx-1642414392475.png)
#[derive(Debug)]
#[allow(unused)]
pub enum List {
    Node(i32, Box<List>),
    Nil,
}

fn make_list() {
    let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));
    println!("{:?}", list);
}
