//! 实现 Deref trait 允许我们重载解引用运算符(dereference operator)
//! 复习: &str

use std::ops::Deref;

pub fn entry() {
    review_dereference();

    let str = MyBox::new(String::from("hello"));
    // &MyBox<String> -> &str
    hello(&str);

    // 如果没有实现 Deref trait, 只能用下面这个
    // (*m) 将 MyBox<String> 解引用为 String. 接着 & 和 [..] 获取了
    // 整个 String 的字符串 slice 来匹配 hello 的签名
    hello(&(*str)[..]);
}

pub fn review_dereference() {
    let x = 5;
    let y = &x;
    let box_y = Box::new(x);

    assert_eq!(5, x);
    // 5 是一个 i32, 而 y 是一个 &i32, 是无法比较的
    // 因此你必须将 y 解引用城 i32 类型, 才能和 i32 类型的 5 做比较
    assert_eq!(5, *y);

    // 同理, box_y 是 Box<i32> 类型, 无法和 i32 比较
    // 需要使用解引用来把"Box"解除掉
    assert_eq!(5, *box_y);
}

// 复习下, 这叫元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // 一般都会返回 T 的引用, 因为大部分使用解引用运算符
    // 的情况下我们并不希望获取 MyBox<T> 内部值的所有权
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn custom_dereference() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // 因为一开始 MyBox 没有实现 Deref trait
    // 没有 Deref trait 的话, 编译器只会解引用 & 引用类型, 此时 *y 是报错的
    // deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值,
    // 并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力
    assert_eq!(5, *y);

    // 我们在调用 *y, 底层实际上是在调用 *(y.deref())
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// 当 T: Deref<Target=U> 时从 &T 到 &U.
// 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U.
// 当 T: Deref<Target=U> 时从 &mut T 到 &U. (可以从 mut -> 不可变, 但不能出现不可变 -> mut)
