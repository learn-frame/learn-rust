//! 大部分情况下所有权是非常明确的: 可以准确地知道哪个变量拥有某个值.
//! 然而, 有些情况单个值可能会有多个所有者. 例如, 在图数据结构中, 多个边可能指向相同的节点
//! 而这个节点从概念上讲为所有指向它的边所拥有. 节点直到没有任何边指向它之前都不应该被清理
//!
//! 为了启用多所有权, Rust 有一个叫做 Rc<T> 的类型. 其名称为引用计数(reference counting)的缩写.
//! 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用. 如果某个值有零个引用, 就代表没有任何有效引用并可以被清理.
//!
//! Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取, 而且无法在编译时确定程序的哪一部分会最后结束使用它的时候.
//! 如果确实知道哪部分是最后一个结束使用的话, 就可以令其成为数据的所有者, 正常的所有权规则就可以在编译时生效.
//! TODO: Rc<T> 只能用在单线程, 十六章会讲到多线程的

use std::rc::Rc;
use List::{Nil, Node};

#[derive(Debug)]
#[allow(unused)]
enum List {
    Node(i32, Rc<List>),
    Nil,
}

pub fn entry() {
    concept_of_rc();
    make_rc_list();
    increase_count();
}

/// Example
///
/// ```
/// b → 3
///        ↘
///     a → 5 → 10 → Nil
///        ↗
/// c → 4
/// ```
///
fn make_rc_list() {
    // Node 成员拥有其储存的数据, 所以当创建 b 列表时, a 被移动进了 b 这样 b 就拥有了 a.
    // 接着当再次尝试使用 a 创建 c 时, 这不被允许, 因为 a 的所有权已经被移动.
    // let a = BoxCons(5, Box::new(BoxCons(10, Box::new(BoxNil))));
    // let b = BoxCons(3, Box::new(a));
    // let c = BoxCons(4, Box::new(a));

    // 当创建 b 时, 不同于获取 a 的所有权, 这里会克隆 a 所包含的 Rc<List>,
    // 这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权.
    // 创建 c 时也会克隆 a, 这会将引用计数从 2 增加为 3. 每次调用 Rc::clone, Rc<List>
    // 中数据的引用计数都会增加, 直到有零个引用之前其数据都不会被清理
    // Rc::clone 只会增加引用计数, 比起会做 clone 的深拷贝, 这并不会花费多少时间
    let a = Rc::new(Node(5, Rc::new(Node(10, Rc::new(Nil)))));
    let b = Node(3, Rc::clone(&a));
    let c = Node(4, Rc::clone(&a));
    println!("{:?} {:?}", b, c);
}

// 克隆 Rc<T> 会增加引用计数
fn increase_count() {
    let a = Rc::new(Node(5, Rc::new(Node(10, Rc::new(Nil)))));
    // Rc::string_count 也有 Rc::weak_count
    println!("count after creating a = {}", Rc::strong_count(&a)); // count after creating a = 1
    #[allow(unused)]
    let b = Node(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // count after creating b = 2
    {
        #[allow(unused)]
        let c = Node(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // count after creating c = 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // count after c goes out of scope = 2
}
// 结尾当 b 然后是 a 离开作用域时, 此处计数会是 0, 同时 Rc<List> 被完全清理.
// 使用 Rc<T> 允许一个值有多个所有者, 引用计数则确保只要任何所有者依然存在其值也保持有效

// 我们知道 Rust 中只能一个变量(引用)指向一个堆, 不能存在多个变量指向同一个堆
// 通过
fn concept_of_rc() {
    let a = Rc::new(1);
    #[allow(unused)]
    let b = a.clone();
    #[allow(unused)]
    let c = a.clone();
    assert_eq!(3, Rc::strong_count(&a)); // Rc::new(1) 共有 3 个强引用变量, 换句话说, 有三个变量对 Rc::new(1) 拥有所有权

    #[allow(unused)]
    let d = Rc::downgrade(&a);
    #[allow(unused)]
    let e = Rc::downgrade(&a);
    #[allow(unused)]
    let f = &*a;
    assert_eq!(2, Rc::weak_count(&a)); // Rc::new(1) 共有 2 个弱引用变量, 弱引用没有有权, 只保留对 Rc<T> 中值的引用

}
