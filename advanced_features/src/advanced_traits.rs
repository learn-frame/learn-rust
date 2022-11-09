use std::fmt;
use std::ops::Add;

pub fn entry() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    use_trait_name_to_call_method();
    fully_qualified_syntax();

    OutlinePrint::outline_print(&Points { x: 1, y: 2 });

    use_new_type();
}

#[allow(unused)]
struct Counter {
    count: u32,
}

#[allow(unused)]
struct Node<T> {
    val: T,
    left: Box<Node<T>>,
    right: Box<Node<T>>,
}

pub trait Iterator {
    // 关联类型, 类似于泛型
    // 比起泛型, 它的优点是更加内聚
    // 差距可以跟 IteratorByGenerics 的写法来对比
    type Item;

    // Self 是每个 trait 都带有的隐式类型参数, 代表实现当前 trait 的具体类型.
    fn next(&mut self) -> Option<Self::Item>;

    fn before(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    // 实现某个 trait 时, 关联类型必须指定类型
    type Item = Node<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn before(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait IteratorByGenerics<T> {
    fn next(&mut self) -> Option<T>;

    fn before(&mut self) -> Option<T>;
}

// 如果用泛型, 只要是 <T> 的, 你都得改成指定的类型
impl IteratorByGenerics<u32> for Counter {
    // 手写类型
    fn next(&mut self) -> Option<u32> {
        // 手写类型
        todo!()
    }

    fn before(&mut self) -> Option<u32> {
        // 手写类型
        todo!()
    }
}

/// 默认泛型类型参数和运算符重载
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 重载 Add, 可以让两个非数值类型相加
// Point { x: 1, y: 0 } + Point { x: 2, y: 3 }
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add 的泛型部分叫做默认类型参数(default type parameters)
// 如果实现 Add trait 时不指定 Rhs 的具体类型, Rhs 的类型将是默认的 Self 类型
// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

struct Millimeters(u32);
struct Meters(u32);

// 默认参数类型主要用于如下两个方面:
// 1. 扩展类型而不破坏现有代码.
// 2. 在大部分用户都不需要的特定情况进行自定义. 比如这个例子默认是 Point + Point 两种相同的类型相加, 但通过默认参数类型你可以使两个不同类型 Millimeters + Meters 相加
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// let x = Millimeters(2) + Meters(1); // 比较操蛋的事只能 Millimeters + Meters
// let x = Meters(2) + Millimeters(1); // 😈: Meters + Millimeters 就报错

/// 完全限定语法与消歧义: 调用相同名称的方法
/// 不同的 trait 可以有相同的方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn new() -> Human {
        Human
    }

    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn use_trait_name_to_call_method() {
    let human = Human::new();
    Pilot::fly(&human);
    Wizard::fly(&human);
    human.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn fully_qualified_syntax() {
    // A baby dog is called a "Spot"
    println!("A baby dog is called a \"{}\"", Dog::baby_name());

    // 😈: 但这样是错误的, 因为 Animal::baby_name 是关联函数而不是方法,
    // 因此它没有 self 参数，Rust 无法计算出所需的是哪一个 Animal::baby_name 实现
    // println!("A baby dog is called a \"{}\"", Animal::baby_name());

    // 因此你可以使用完全限定语法, 这是调用函数时最为明确的方式
    println!(
        "A baby dog is called a \"{}\"",
        <Dog as Animal>::baby_name()
    );
}

/// 父 trait 用于在另一个 trait 中使用某 trait 的功能
// OutlinePrint 需要 fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Points {
    x: i32,
    y: i32,
}

// 那 Points 必须要实现 fmt::Display
impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Points 需要实现 OutlinePrint
impl OutlinePrint for Points {}

/// 孤儿规则: 该 trait 和要实现该 trait 的那个类型至少有一个要在当前 crate 中定义, 才能实现某个 trait
/// 比如不能在当前这个 crate 中为 Vec<T> 实现 Display trait. 这是因为 Display 和 Vec<T> 都定义于标准库中, 因为如果你直接改了 std 的方法, 那就出大事了
/// 因此你可以通过 `impl 原生 for 自定义`

struct Wrapper(Vec<String>);

// impl 原生 for 自定义
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display 的实现使用 self.0 来访问其内部的 Vec<T>
        // 因为 Wrapper 是元组结构体而 Vec<T> 是结构体总位于索引 0 的项
        // 接着就可以使用 Wrapper 中 Display 的功能了
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn use_new_type() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
