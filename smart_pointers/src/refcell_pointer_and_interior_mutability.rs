//! 内部可变性(Interior mutability)是 Rust 中的一个设计模式, 它允许你即使在有不可变引用时也可以改变数据,
//! 这通常是借用规则所不允许的. 为了改变数据, 该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
//!
//! Rust 中的可变或不可变主要是针对一个变量绑定而言的, 比如对于结构体来说, 可变或不可变只能对其实例进行设置, 而不能设置单个成员的可变性.
//! Rust 提供了 Cel<T> 和 RefCell<T> 来应对这种情况

use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use List::{Nil, Node};

pub fn entry() {
    learn_cell();
    learn_refcell();
    use_rc_and_refcell();
    can_not_modify_a_refrence();
}

#[allow(unused)]
#[derive(Debug)]
struct Point {
    x: Cell<i32>,
    y: Cell<i32>,
    desc: RefCell<String>,
}

/// 对于复制类型的变量, 可以使用 Cell 进行修改
///
/// 使用 Cell<T> 虽然没有运行时开销, 但是尽量不要用它包裹大的结构体
/// 因为 Cell<T> 内部每次 get/set 都会执行一次按位复制
fn learn_cell() {
    let p = Point {
        x: Cell::new(1),
        y: Cell::new(1),
        desc: RefCell::new(String::from("(1, 1)")),
    };

    assert_eq!(Cell::new(1), p.x);
    assert_eq!(1, p.x.get());

    p.x.set(2);
    assert_eq!(p.x, Cell::new(2));
}

/// 对于没有实现 Copy 的类型, 可以使用 RefCell 进行修改
///
/// 以 Point 为例, 如果 desc 是 Cell<String>, 由于 String 不是复制类型导致无法 Debug:
/// the trait bound `String: Copy` is not satisfied
/// the trait `Debug` is implemented for `Cell<T>`
/// required for `Cell<String>` to implement `Debug`
///
/// RefCell<T>需要在运行时执行借用检查, 所以有运行时开销, 一旦发现违反借用规则的情况, 则会引发线
/// 程 panic 而退出当前线程
fn learn_refcell() {
    let p = Point {
        x: Cell::new(1),
        y: Cell::new(1),
        desc: RefCell::new(String::from("(1, 1)")),
    };

    let mut desc = p.desc.borrow_mut();
    desc.clear();
    desc.push_str("(0, 0)");
    println!("{:?}", desc.borrow()); // "(0, 0)"

    //  😈: 'already borrowed: BorrowMutError'. 因为上面已经有 desc 获取过可变借用了
    // 下面再想借用就报错了(不管是 borrow 还是 borrow_mut 都不行)
    // 这个错误在编译器不会被发现, 但在运行时会有一个借用检查器来检查
    // let mut x = p.desc.borrow_mut();
    // let y = p.desc.borrow();
}

pub fn can_not_modify_a_refrence() {
    // let x = 5;
    // 显然是不对的, 你借了人家 x, 是要原封不动的还的, 因此不能修改
    // let y = &mut x;

    let mut x = 5;
    let y = &mut x;
    println!("{}", y);

    let a = RefCell::new(5);
    let b = a.borrow_mut();
    println!("{}", b);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/// 使用 RefCell<T> 能够在外部值被认为是不可变的情况下修改内部值
/// 当创建不可变和可变引用时, 我们分别使用 & 和 &mut 语法.
/// 对于 RefCell<T> 来说, 则是 borrow 和 borrow_mut 方法
/// borrow 方法返回 Ref<T> 类型的智能指针, borrow_mut 方法返回 RefMut 类型的智能指针.
/// 这两个类型都实现了 Deref, 所以可以当作常规引用对待.
///
/// 原则:
/// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针. 每次调用 borrow, RefCell<T> 将活动的不可变借用计数加一.
/// 当 Ref<T> 值离开作用域时, 不可变借用计数减一. 就像编译时借用规则一样, RefCell<T> 在任何时候只允许有**多个不可变借用或一个可变借用**.
#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger {
            //     sent_messages: vec![],
            // }
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // 😈: 不能相同作用域创建两个可变借用, 在编译时不会报错, 在运行时会报错
    // panicked at 'already borrowed: BorrowMutError'
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();

    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}

// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
// 如果有一个储存了 RefCell<T> 的 Rc<T> 的话, 就可以得到有多个所有者并且可以修改的值了
#[derive(Debug)]
pub enum List {
    Node(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn use_rc_and_refcell() {
    // 里创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问
    let value = Rc::new(RefCell::new(5));

    // 接着在 a 中用包含 value 的 Node 成员创建了一个 List. 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权,
    // 而不是将所有权从 value 移动到 a 或者让 a 借用 value
    let a = Rc::new(Node(Rc::clone(&value), Rc::new(Nil)));
    let b = Node(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Node(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 可以修改值
    // 通过使用 RefCell<T>, 我们可以拥有一个表面上不可变的 List,
    // 不过可以使用 RefCell<T> 中提供内部可变性的方法来在需要时修改数据
    // RefCell<T> 的运行时借用规则检查也确实保护我们免于出现数据竞争
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a); // a after = Node(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b); // b after = Node(RefCell { value: 6 }, Node(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c); // c after = Node(RefCell { value: 10 }, Node(RefCell { value: 15 }, Nil))
}
