pub mod method_syntax;

fn main() {
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
    // 除了使用 println 的方式, 还可以用 dbg!
    // 但要注意的是, 两者都必须在 structs 上加上 trait #[derive(Debug)]
    dbg!(&user2);

    foo(
        String::from("Sayaka Yamamoto"),
        String::from("developer@yanceyleo.com"),
    );

    // 元组结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black: {:?}", black);

    // 没有任何字段的类单元结构体
    // 如果没有没有任何字段, 它们被称为类单元结构体(unit-like structs), 因为它们类似于
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用, trait 就好比 interface
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);

    let instance = method_syntax::Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(instance.area());
}

// 增加属性来派生 Debug trait, 来打印 struct
#[derive(Debug)]
struct User {
    username: String, // 这里也可以用 &str, 不过必须要增加 'a 注解, 这涉及到生命周期的概念
    email: String,
    sign_in_count: u64,
    active: bool,
    hobby: Vec<i32>,
}

fn foo(username: String, email: String) -> User {
    User {
        username, // 和 ES6 一样, 变量与字段同名时可简化 username: username 的写法
        email,
        sign_in_count: 1,
        active: true,
        hobby: [1, 2, 3].to_vec(),
    }
}
