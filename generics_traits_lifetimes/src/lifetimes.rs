use std::fmt::Display;

pub fn entry() {
    bar();
    println!("{}", first_word("Yancey Leo"));
    println!("{}", longest_with_an_announcement("hi", "world", 1));
}

// 如下这个写法是错误的, 因为返回值是 s 的引用
// 但 s 在函数之后就被销毁, 所以外面是不可能拿到的
// pub fn say_hi() -> &i32 {
//     let s = 1;

//     &s
// }
// ^ `x` dropped here while still borrowed

// Rust 编译器有一个借用检查器(borrow checker), 它比较作用域来确保所有的借用都是有效的.

// r 一开始的生命周期是 'a 这个区域, 但因为后面引用了 x,
// 所以生命周期缩到 'b 这个区域, 因此最后打印 r 时,
// 超出了 'b 这个区域, 故报错
// pub fn say_hello() {
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }

// r 引用了 x, 也就拥有了 x 的生命周期 'b
// 就可以正常打印出 r 来了
// pub fn say_hello() {
//     {
//         let x = 5;            // ----------+-- 'b
//                               //           |
//         let r = &x;           // --+-- 'a  |
//                               //   |       |
//         println!("r: {}", r); //   |       |
//                               // --+       |
//     }                         // ----------+
// }

pub fn bar() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); // The longest string is abcd
}

// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// 😈 如下语法是错的, 看似人畜无害, 其实暗藏玄机
// 因为函数可能返回 x 的引用, 也可能返回 y 的引用, 所以它不知道最终采用的是 x 还是 y 的生命周期
// fn longest(x: &str, y: &str) -> &str {

// 因此你需要加上生命周期注解, 这两个生命周期注解意味着
// 引用 first 和 second 必须与这泛型生命周期存在得一样久
// 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
// 这就是我们告诉 Rust 需要其保证的约束条件
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// pub fn foo() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         // 此时 string2 跟 string1 不在同一级生命周期上, 就报错了
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// 更蛋疼的是, 这样也是错的
// 因为返回值不是属于 'a 泛型的, 严谨, 👍
// fn longest1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 结构体的生命周期
// 上回书说道, 我们在结构体中定义字符串的时候, 只用了 String, 而没有使用 &str
// 如果你想用 string slice, 就必须得使用 'a
#[allow(unused)]
pub struct ImportantExcerpt<'a, T> {
    part: &'a str,
    age: T,
}

#[allow(unused)]
pub fn foo() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
        age: 18,
    };
    println!("{}", i.part);
}

// 现在我们已经知道了每一个引用都有一个生命周期,
// 而且我们需要为那些使用了引用的函数或结构体指定生命周期
// 但下面这个例子定义了一个没有使用生命周期注解的函数, 即便其参数和返回值都是引用
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 这是因为上面这个例子就涉及到了生命周期省略(Lifetime Elision)
// 被编码进 Rust 引用分析的模式被称为生命周期省略规则(lifetime elision rules),
// 函数或方法的参数的生命周期被称为 输入生命周期(input lifetimes), 而返回值的生命周期被称为输出生命周期(output lifetimes)
// 编译器共有三个规则:
// 1. 每一个是引用的参数都有它自己的生命周期参数
// 2. 如果只有一个输入生命周期参数, 那么它被赋予所有输出生命周期参数
// 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self, 说明是个对象的方法, 那么所有输出生命周期参数被赋予 self 的生命周期

// 以 fn first_word(s: &str) -> &str {} 为例
// 应用第一条规则, 变成了:
// fn first_word<'a>(s: &'a str) -> &str {}
// 再应用第二条规则, 变成了:
// fn first_word<'a>(s: &'a str) -> &'a str {}

// 再看另一个例子: fn longest(x: &str, y: &str) -> &str {}
// 应用第一条规则, 变成了:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// 然后它既不适合第二条规则, 也不适合第三条规则, 编译器就无法明确生命周期, 你就必须自己声明泛型注解了

// 如上所述, 因为 ImportantExcerpt 上有个 string slice
// 所以你必然需要加上生命周期注解了
impl<'a, T> ImportantExcerpt<'a, T> {
    #[allow(unused)]
    fn level(&self) -> i32 {
        3
    }

    #[allow(unused)]
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(unused)]
fn bar1() {
    // 'static 的生命周期能够存活于整个程序期间. 所有的字符串字面值都拥有 'static 生命周期
    #[allow(unused)]
    let s: &'static str = "I have a static lifetime.";
}

// ann 的类型是泛型 T, 它可以被放入任何实现了 where 从句中指定的 Display trait 的类型
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
