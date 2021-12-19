fn main() {
    println!("Hello, world!");
    foo(11, "");
    bar();
    println!("{}", plus_one());
}

// rust 不关心函数定义的位置, 定义了就行
// rust 的函数不支持默认参数和不定参数, 草, 也没箭头函数
// 必须显式指定参数
// 如果有返回值, 必须显式标明返回值的类型
fn foo(x: i32, y: &str) -> i32 {
    // 不能把 let 语句赋值给另一个变量
    // let c = (let d = 6);

    println!("Hello, {}, {}!", x, y);
    return 1;
}

fn bar() {
    let x = 5;
    println!("{}", x);

    // 这叫做表达式, 大括号之间形成了作用域
    // x + 1 相当于 return x + 1;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // 4
}

// 等价于 return 5;
// 注意 5 不能加分号, 因为下面是一个表达式, 而不能是个语句.
fn plus_one() -> i32 {
    5
}

// rust 函数不支持重载
// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }
