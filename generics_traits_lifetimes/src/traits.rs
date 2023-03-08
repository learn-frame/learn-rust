//! 从语义上来说, trait是在行为上对类型的约束, 这种约束可以让 trait 有如下 4 种用法:
//! - 接口抽象. 接口是对类型行为的统一约束.
//! - 泛型约束. 泛型的行为被 trait 限定在更有限的范围内.
//! - 抽象类型. 在运行时作为一种间接的抽象类型去使用, 动态地分发给具体的类型.有 trait 对象和 impl Trait 两种
//! - 标签 trait. 对类型的约束, 可以直接作为一种标签使用.

// impl Trait for Type
// 为 Type 实现 Trait 接口
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Add;

pub fn entry() {
    let tweet = Tweet {
        username: String::from("YanceyOfficial"),
        content: String::from("content content content"),
        reply: true,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    // 继承
    let p = MyPaginate {};
    p.set_page();
    p.set_per_page();
    p.set_skip_page();

    // 泛型约束
    add(1, 1);

    // trait 在编译时是无法确定大小的一种类型, 而静态要比动态性能更好
    //
    // 静态
    fn on_page_static(s: impl Page + PerPage) {
        s.set_page()
    }
    // 动态
    fn on_page_dynamic(s: &MyPaginate) {
        s.set_page()
    }
    let p1 = MyPaginate {};
    on_page_static(p1);
    let p2 = MyPaginate {};
    on_page_dynamic(&p2);

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    use_trait_name_to_call_method();
    fully_qualified_syntax();

    OutlinePrint::outline_print(&Points { x: 1, y: 2 });

    use_new_type();

    higher_ranked_trait_bounds(1_usize);
}

pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn say_hi() -> String {
        String::from("Hello, world")
    }
}

pub trait Displayed {
    fn show_modal(&self) -> String;
}

pub trait Clone {
    fn copy(&self) -> String;
}

pub trait CustomDebug {
    fn show_bug(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// self 为结构体 NewsArticle 的任意实例, &self 则为实例的引用
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // 你可以选择重载默认方法
    fn say_hi() -> String {
        String::from("Hi, world")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[allow(unused)]
pub fn use_trait_as_params(item: impl Summary) {}

#[allow(unused)]
pub fn use_trait_as_params_1<T: Summary + Displayed>(item: T) {}

#[allow(unused)]
pub fn use_trait_as_params_2(item: impl Summary + Displayed) {}

#[allow(unused)]
pub fn use_trait_bound_as_params<T: Summary>(item: T) {}

#[allow(unused)]
pub fn use_trait_bound_as_params_1<T: Summary>(item: T, item1: T) {}

#[allow(unused)]
pub fn use_trait_bound_as_params_2<T: Summary + Displayed>(item: T) {}

// 如果像这种很复杂的
#[allow(unused)]
fn some_function<T: Displayed + Clone, U: Clone + CustomDebug>(t: T, u: U) {}

// 可以用 where 语法
#[allow(unused)]
fn some_function_1<T, U>(t: T, u: U)
where
    T: Displayed + Clone,
    U: Clone + CustomDebug,
{
}

// 返回值也可以是 traits
#[allow(unused)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[allow(unused)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(unused)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 类型 Pair<T> 总是实现了 new 方法,
// 不过只有那些为 T 类型实现了 PartialOrd trait (来允许比较)
// 和 Display trait (来启用打印) 的 Pair<T> 才会实现 cmp_display 方法
#[allow(unused)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 对原生的, 直接导入 Debug 即可
#[derive(Debug)]
struct Foo; // our custom type

// 也可以手动为 Foo 实现 Display trait
impl Display for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "testing, testing")
    }
}

// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations, 他们被广泛的用于 Rust 标准库中
// 例如, 标准库为任何实现了 Display trait 的类型实现了 ToString trait
// 我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法
// impl<T: Display> ToString for T {}
// let s = 3.to_string();

/// 继承
trait Page {
    fn set_page(&self) -> ();
}

trait PerPage {
    fn set_per_page(&self) -> ();
}

struct MyPaginate;

impl Page for MyPaginate {
    fn set_page(&self) -> () {}
}
impl PerPage for MyPaginate {
    fn set_per_page(&self) -> () {}
}

// 继承 trait
trait Paginate: Page + PerPage {
    fn set_skip_page(&self) -> ();
}

// 实现 trait
impl<T: Page + PerPage> Paginate for T {
    fn set_skip_page(&self) -> () {}
}

/// 泛型约束
// 😈 下面这段代码是错的, 因为 T 是泛型, 理论设置什么类型都可以,
// 但加法却只能接受数字和字符串, 这就会出错
// fn add<T>(a: T, b: T) -> T {
//     a + b
// }
// 因此它要求你有如下写法, 即表示 sum 函数的参数必须实现 Add trait
pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 使用 trait 对泛型进行约束, 叫作 trait 限定(trait Bound). 格式如下:
// fn generic<T : MyTrait + MyOtherTrait + SomeStandardTrait> (t : T) {}
// 该泛型函数签名要表达的意思是: 需要一个类型 T, 并且该类型 T 必须实现 MyTrait,
// MyOtherTrait 和 SomeStandardTrait 中定义的全部方法, 才能使用该泛型函数.
// 从数学角度分析比较好理解, 如 impl<T: A + B> C for T, 冒号意味着包含于, 加号代表着交集, 即为所有 T ⊂ (A ∩ B) 实现 Trait C

// 如果像这种很复杂的
#[allow(unused)]
fn some_function_2<T: Displayed + Clone, U: Clone + CustomDebug>(t: T, u: U) {}

// 可以用 where 语法
#[allow(unused)]
fn some_function_3<T, U>(t: T, u: U)
where
    T: Displayed + Clone,
    U: Clone + CustomDebug,
{
}

/// 标签 trait
/// - Sized trait, 用来标识编译期可确定大小的类型.
/// - Unsize trait, 目前该 trait 为实验特性, 用于标识动态大小类型(DST).
/// - Copy trait, 用来标识可以按位复制其值的类型.
/// - Send trait,  用来标识可以跨线程安全通信的类型, 可以安全地在线程间传递值, 也就是说可以跨线程传递所有权.
/// - Sync trait, 用来标识可以在线程间安全共享引用的类型, 可以跨线程安全地传递共享(不可变)引用.
/// 有了 Send trait 和 Sync trait, 就可以把 Rust 中所有的类型归为两类: 可以安全跨线程传递的值和引用, 以及不可以跨线程传递的值和引用
#[allow(unused)]
struct Foo1<T>(T); // 等价于 Foo<T: Sized>
#[allow(unused)]
struct Bar1<T: ?Sized>(T);

// Trait 的三个问题
// - 孤儿规则, #[fundamental]
// - 重叠问题. #[feature(specialization)]
// - 关联类型不支持泛型. 泛型关联类型(Generic Associated Type, GAT)

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
trait OutlinePrint: Display {
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
impl Display for Points {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Points 需要实现 OutlinePrint
impl OutlinePrint for Points {}

/// 孤儿规则: 该 trait 和要实现该 trait 的那个类型至少有一个要在当前 crate 中定义, 才能实现某个 trait
/// 比如不能在当前这个 crate 中为 Vec<T> 实现 Display trait. 这是因为 Display 和 Vec<T> 都定义于标准库中, 因为如果你直接改了 std 的方法, 那就出大事了
/// 因此你可以通过 `impl 原生 for 自定义`; 或者把原生 trait 抄到当前的 crate 中, 这样你就可以对当前 crate 中的 trait 进行修改, 而非改的原生的

struct Wrapper(Vec<String>);

// impl 原生 for 自定义
impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

// 高阶生命周期
trait DoSomething<T>
where
    T: Debug,
{
    fn do_something(&self, any_val: T) -> ();
}

impl<T> DoSomething<T> for usize
where
    T: Debug,
{
    fn do_something(&self, any_val: T) -> () {
        println!("{:?}", any_val);
    }
}

/// 😈: s 在 higher_ranked_trait_bounds 函数调用结束后被析构了
/// 但 s 的引用却被 d 的某个方法引用着, 就不符合生命周期规则了
/// ```
/// fn higher_ranked_trait_bounds<'a>(d: impl DoSomething<&'a usize>) {
///     let s: usize = 1;
///     d.do_something(&s);
/// }
/// ```
///
/// 其实明眼人能看出来, 上面的 'a, 是把 higher_ranked_trait_bounds 函数跟内部 s 变量的生命周期绑定在一起了
/// 但其实应该把 s 跟 DoSomething Trait 中的方法们绑定起来才行
fn higher_ranked_trait_bounds(d: impl for<'a> DoSomething<&'a usize>) {
    let s: usize = 1;
    d.do_something(&s);
}
