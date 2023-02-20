//! 从语义上来说, trait是在行为上对类型的约束, 这种约束可以让 trait 有如下 4 种用法:
//! - 接口抽象. 接口是对类型行为的统一约束.
//! - 泛型约束. 泛型的行为被 trait 限定在更有限的范围内.
//! - 抽象类型. 在运行时作为一种间接的抽象类型去使用, 动态地分发给具体的类型.有 trait 对象和 impl Trait 两种 
//! - 标签 trait. 对类型的约束, 可以直接作为一种标签使用.

// impl Trait for Type
// 为 Type 实现 Trait 接口
use std::fmt::{Display, Formatter, Result};
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

pub trait Debug {
    fn show_bug(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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
fn some_function<T: Displayed + Clone, U: Clone + Debug>(t: T, u: U) {}

// 可以用 where 语法
#[allow(unused)]
fn some_function_1<T, U>(t: T, u: U)
where
    T: Displayed + Clone,
    U: Clone + Debug,
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
    fn set_page(&self) -> () {
        todo!()
    }
}
impl PerPage for MyPaginate {
    fn set_per_page(&self) -> () {
        todo!()
    }
}

// 继承 trait
trait Paginate: Page + PerPage {
    fn set_skip_page(&self) -> ();
}

// 实现 trait
impl<T: Page + PerPage> Paginate for T {
    fn set_skip_page(&self) -> () {
        todo!()
    }
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
fn some_function_2<T: Displayed + Clone, U: Clone + Debug>(t: T, u: U) {}

// 可以用 where 语法
#[allow(unused)]
fn some_function_3<T, U>(t: T, u: U)
where
    T: Displayed + Clone,
    U: Clone + Debug,
{
}
