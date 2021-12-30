pub fn entry() {
    bar();
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

pub struct ImportantExcerpt<'a, T> {
    part: &'a str,
    age: T,
}

pub fn foo() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
        age: 18,
    };
    println!("{}", i.part);
}
