// 好家伙, trait 就类似于 interface
// TypeScript 的 interface 可以有方法, 也可以有对象属性
// 而 rust 中的 interface 只承载方法, 常量
// 对象属性是要放在 struct 中的
use std::fmt::Display;

pub fn entry() {
    let tweet = Tweet {
        username: String::from("YanceyOfficial"),
        content: String::from("content content content"),
        reply: true,
        retweet: false,
    };

    println!("{}", tweet.summarize());
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
// TODO: 十七章会讲到如何返回多种类型的 trait
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

// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations, 他们被广泛的用于 Rust 标准库中
// 例如, 标准库为任何实现了 Display trait 的类型实现了 ToString trait
// 我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法
// impl<T: Display> ToString for T {}
// let s = 3.to_string();
