//! 指针 pointer 是一个包含内存地址的变量的通用概念
//! 这个地址引用, 指向一些其他数据
//! Rust 中最常见的指针是第四章介绍的引用, 通过 & 符号借用了他们所指向的值
//! 智能指针(smart pointers)是一类数据结构, 他们的表现类似指针, 但是也拥有额外的元数据和功能
//! 在 Rust 中, 普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针; 相反, 在大部分情况下,智能指针拥有他们指向的数据
//!
//! 实际上本书中已经出现过一些智能指针, 比如第八章的 String 和 Vec<T>, 虽然当时我们并不这么称呼它们.
//! 当 String 或者 Vec<T> 声明的变量被销毁后, 其对应的已分配的堆内存也被销毁了, 这得怀于 Drop trait.
//! 这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们. 它们也带有元数据(比如他们的容量)和额外的功能或保证(String 的数据总是有效的 UTF-8 编码)
//!
//! 智能指针通常使用结构体实现. 只不过它的行为类似指针. 智能指针是对指针的一层封装, 提供了一些额外的功能, 比如自动释放堆内存
//! 智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait. Deref trait 允许智能指针结构体实例表现的像引用一样,
//! 这样就可以编写既用于引用, 又用于智能指针的代码. Drop trait 允许我们自定义当智能指针离开作用域时(对象即将消亡之时)运行的代码(drop 方法)
//!
//! 如下是一些常见的智能指针:
//! Box<T>, 用于在堆上分配值, 允许在编译时执行不可变或可变借用检查
//! Rc<T>, 一个引用计数类型, 允许相同数据有多个所有者, 仅允许在编译时执行不可变借用检查
//! Ref<T> 和 RefMut<T>, 通过 RefCell<T> 访问, 允许在运行时执行不可变或可变借用检查
//!

pub mod box_pointer;
pub mod cow_pointer;
pub mod deref_trait;
pub mod drop_trait;
pub mod rc_pointer;
pub mod refcell_pointer_and_interior_mutability;
pub mod reference_cycles;

fn main() {
    box_pointer::entry();
    deref_trait::entry();
    drop_trait::entry();
    rc_pointer::entry();
    refcell_pointer_and_interior_mutability::entry();
    cow_pointer::entry();
    reference_cycles::entry();
}
