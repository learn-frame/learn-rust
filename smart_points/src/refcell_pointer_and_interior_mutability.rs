//! 内部可变性(Interior mutability)是 Rust 中的一个设计模式, 它允许你即使在有不可变引用时也可以改变数据,
//! 这通常是借用规则所不允许的. 为了改变数据, 该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
//! TODO: 第十九章会讲到不安全代码
//!
//! 不同于 Rc<T>, RefCell<T> 代表其数据的唯一的所有权
//!
//! 回顾一下借用规则:
//! 1. 在任意给定时刻, 只能拥有一个可变引用或任意数量的不可变引用之一(而不是两者)
//! 2. 引用必须总是有效的.
//!
//! 对于引用和 Box<T>, 借用规则的不可变性作用于编译时. 对于 RefCell<T>, 这些不可变性作用于运行时.
//! 对于引用, 如果违反这些规则, 会得到一个编译错误. 而对于 RefCell<T>, 如果违反这些规则程序会 panic 并退出
//!
//! RefCell<T> 正是用于当你确信代码遵守借用规则, 而编译器不能理解和确定的时候.
//!
//! 类似于 Rc<T>, RefCell<T> 只能用于单线程场景.
//! 如果尝试在多线程上下文中使用 RefCell<T>, 会得到一个编译错误. TODO: 第十六章会介绍如何在多线程程序中使用 RefCell<T> 的功能.
//!
//! Rc<T> 允许相同数据有多个所有者; Box<T> 和 RefCell<T> 有单一所有者.
//! Box<T> 允许在编译时执行不可变或可变借用检查; Rc<T>仅允许在编译时执行不可变借用检查; RefCell<T> 允许在运行时执行不可变或可变借用检查.
//! 因为 RefCell<T> 允许在运行时执行可变借用检查, 所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值. 在不可变值内部改变值就是"内部可变性"模式

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

pub fn entry() {
    use_rc_and_refcell();
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
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn use_rc_and_refcell() {
    // 里创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问
    let value = Rc::new(RefCell::new(5));

    // 接着在 a 中用包含 value 的 Cons 成员创建了一个 List. 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权,
    // 而不是将所有权从 value 移动到 a 或者让 a 借用 value
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 可以修改值
    // 通过使用 RefCell<T>, 我们可以拥有一个表面上不可变的 List,
    // 不过可以使用 RefCell<T> 中提供内部可变性的方法来在需要时修改数据
    // RefCell<T> 的运行时借用规则检查也确实保护我们免于出现数据竞争
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a); // a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b); // b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c); // c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}
