pub fn entry() {
    match_literals();
    match_named_variables();
    multiple_patterns();
    range_patterns();
    destructure_struct();
    destructure_enum();
    destructure_struct_and_enum();
    foo(3, 4);
    ignore_nested();
    ignore_multi();
    ignore_variable();
    bar();
    double_dot_ignore();
    match_guard();
    match_guard_resolve_covered_variable();
    at_operator_binding();
    while_let();
}

// 匹配字面量
fn match_literals() {
    let x = 9;

    match x {
        1..=3 => println!("in 1 - 3"),
        6 => println!("6"),
        5 | 7 | 9 => println!("could be 5, 7, 9"),
        // @ 符号把 23 赋值给 n, 使用操作符 @ 可以将模式中的值绑定给一个变量, 供分支右侧的代码使用, 这类匹配叫绑定模式(Binding Mode).
        n @ 23 => println!("{}", n),
        _ => println!("anything"),
    }
}

// 匹配命名变量
fn match_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 第二个匹配分支中的模式引入了一个新变量 y, 它会匹配任何 Some 中的值.
        // 因为我们在 match 表达式的新作用域中, 这是一个新变量, 而不是开头声明为值 10 的那个 y.
        // 这个新的 y 绑定会匹配任何 Some 中的值, 在这里是 x 中的值.因此这个 y 绑定了 x 中 Some 内部的值.
        // 这个值是 5, 所以这个分支的表达式将会执行并打印出 Matched, y = 5.
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

// 多个模式
fn multiple_patterns() {
    let x = 1;

    match x {
        // 可以使用 | 语法匹配多个模式, 它代表或的意思
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 范围匹配
fn range_patterns() {
    let x = 5;

    match x {
        // 在 [1, 5] b闭区间匹配
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // char 类型也可以有范围
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 复习下 range, 如果前者比后者大, 将返回 early ASCII letter
    // range 不仅可以是数字, 也可以是 char
    let a = -1..=1;
    for i in a {
        println!("{}", i); // early ASCII letter
    }
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn destructure_struct() {
    let p = Point { x: 0, y: 7, z: 2 };
    let Point { x: a, y: b, z: c } = p;
    println!("x: {} y: {} z: {}", a, b, c);

    let Point { x, y, z } = p;
    println!("x: {} y: {} z: {}", x, y, z);

    match p {
        Point { x, y: 0, z } => println!("On the x axis at {}, z: {}", x, z),
        Point { x: 0, y, z } => println!("On the y axis at {}, z: {}", y, z),
        Point { x, y, z } => println!("On neither axis: ({}, {}), z: {}", x, y, z),
    }
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeStandardColor(Color),
}

#[allow(unused)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn destructure_enum() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        _ => (),
    }

    // 嵌套枚举
    let msg1 = Message::ChangeStandardColor(Color::Hsv(0, 160, 255));
    match msg1 {
        Message::ChangeStandardColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeStandardColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn destructure_struct_and_enum() {
    let ((feet, inches), Point { x, y, z }) = ((3, 10), Point { x: 3, y: -10, z: 2 });
    println!("{} {} {} {} {}", feet, inches, x, y, z);
}

// 函数也可以通过下划线来忽略参数
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_nested() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn ignore_multi() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// 通过在变量名前加一个下划线来忽略未使用的变量, 这样 lint 就不会报 unused variable 的 warning 了
fn ignore_variable() {
    let _x = 5;
    let _y = 10;
}

fn bar() {
    let a = Some(String::from("Hello!"));
    let b = Some(String::from("Hello!"));
    let c = Some(String::from("Hello!"));

    // if let 可以代替冗长的 match
    if let Some(_s) = a {
        println!("found a string");
    }

    // 需要注意的是, 虽然是 Some(_s), 但 a 的所有权仍然被移动到了 _s, 所以下面就拿不到 a 了
    // println!("{:?}", a);

    // 一种方式是通过完全忽略来避免
    if let Some(_) = b {
        println!("found a string");
    }
    println!("{:?}", b);

    // 第二种就是通过借用啦
    if let Some(_s) = &c {
        println!("found a string");
    }
    println!("{:?}", c);
}

fn while_let() {
    let mut v: Vec<i32> = (0..10).collect();

    // while let 循环
    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    // 不用 while let
    while v.len() != 0 {
        let x = v.pop();
        println!("{:?}", x);
    }

    // 用 loop 的方式
    loop {
        match v.pop() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
}

fn double_dot_ignore() {
    let origin = Point { x: 0, y: 0, z: 0 };

    // 如果只取某个, 而忽略剩下的, 用 .. 会更优雅
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 否则需要把剩下的所有都列出来, 然后用 _ 忽略
    match origin {
        Point { x, y: _, z: _ } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 对于元组也可以这么用
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // match numbers {
    // // 当然你不能这么做
    // // Rust 不可能决定在元组中匹配 second 值之前应该忽略多少个值, 以及在之后忽略多少个值
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
}

// 匹配守卫(match guard)是一个指定于 match 分支模式之后的额外 if 条件, 它也必须被满足才能选择此分支
fn match_guard() {
    let num = Some(4);

    match num {
        // num 即得是 Some 类型, num 的数值还得小于 5 才会命中下面这条
        // 既当婊子又立牌坊
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;

    // 匹配守卫和 multi pattern 的结合
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn match_guard_resolve_covered_variable() {
    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

enum Messages {
    Hello { id: i32 },
}

fn at_operator_binding() {
    let msg = Messages::Hello { id: 5 };

    match msg {
        Messages::Hello {
            // @ 符号没啥可说的, 在本例中就是
            // 5 是否在 [3, 7] 闭区间之间
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Messages::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Messages::Hello { id } => println!("Found some other id: {}", id),
    }
}
