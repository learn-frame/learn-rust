pub mod method_syntax;

fn main() {
    #[derive(Debug)]
    struct User {
        #[allow(unused)]
        username: String, // 这里也可以用 &str, 不过必须要增加 'a 注解, 这涉及到生命周期的概念
        #[allow(unused)]
        email: String,
        #[allow(unused)]
        sign_in_count: u64,
        active: bool,
        #[allow(unused)]
        hobby: Vec<i32>,
    }

    let mut user1 = User {
        username: String::from("Yancey Leo"),
        email: String::from("yanceyofficial@gmail.com"),
        sign_in_count: 1,
        active: false,
        hobby: vec![1, 2, 3],
    };

    // 注意和 JavaScript 不同, Rust 中修改"对象"的某属性的值, 也需要保证这个变量是 mut 的.
    user1.active = true;

    // user2 中, 除了 sign_in_count 和 hobby, 其他字段都可复用 user1 里面的属性
    let user2 = User {
        sign_in_count: 2,
        hobby: dbg!([2, 3, 4].to_vec()),
        ..user1
    };

    // println!("user2: {:#?}", user2);
    // 除了使用 println! 的方式, 还可以用 dbg!
    // 但要注意的是, 两者都必须在 structs 上加上 trait #[derive(Debug)]
    dbg!(&user2);

    // 元组结构体
    // 特点是字段只有类型没有名称
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);

    // 当元组结构体只有一个元素时称为 New Type 模式
    struct Interger(u32);
    // 也可以使用 type 关键字为一个类型创建别名
    type Int = u32;
    let interger = Interger(10);
    let int: Int = 10;
    assert_eq!(interger.0, 10);
    assert_eq!(int, 10);

    // 没有任何字段的类单元结构体
    // 如果没有没有任何字段, 它们被称为类单元结构体(unit-like structs), 因为它们类似于
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);
    // 比如标准库的 RangeFull 就是单元结构体
    assert_eq!((..), std::ops::RangeFull);

    let instance = method_syntax::Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(instance.area());

    // 联合体和结构体
    #[allow(unused)]
    union U {
        f1: i32,
        f2: u64,
        f3: u32,
    }

    #[allow(unused)]
    struct S {
        f1: i32,
        f2: u64,
        f3: u32,
    }

    println!(
        "{:?} {:?}",
        // 联合体区最大的内存占用, 即 f2 占 8 字节, 那就是 8
        std::mem::size_of::<U>(), // 8
        // 结构体是所有属性的累加(f1: 4 + f2: 8 + f3: 4)
        std::mem::size_of::<S>() // 16
    )
}
