fn main() {
    println!("Hello, world!");
    foo(11, "");
    bar();
    println!("{}", plus_one());

    println!("{:?}", generate_vec());
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

// 对于没有返回值, 返回值类型可以不写, 也可以写成 -> ()
// () 叫做单元类型, 单元类型拥有唯一的值, 就是它本身
fn bar() -> () {
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

// 函数也可以是常量
// 这叫做编译时函数执行 Compile-TimeFunction Execution, 简称 CTFE
const fn give_me_five(n: usize) -> usize {
    5 + n
}

fn generate_vec() -> [i32; 6] {
    // Rust 中固定长度数组的初始值和长度都必须在编译时是确定的
    // 如果想传入一个函数, 必须使用常量函数
    [0; give_me_five(1)]
}

// 理论上你不能用关键字来当作变量名, 除非你使用原始标识符(Raw identifiers)
pub fn r#match(needle: &str, haystack: &str) -> bool {
    let r#let = 1;
    println!("{}", r#let);
    haystack.contains(needle)
}

// Rust 编译器在解析代码的时候, 如果碰到分号, 就会继续往后面执行; 如果碰到语旬, 则执行语句; 如果碰到表达式, 则会会对表达式求值, 如果分号后面什么都没有, 就会补上单元值 ()
// 当遇到函数的时候, 会将函数体的花括号识别为块表达式(Block Expression). 块表达式是由一对花括号和一系列表达式组成的, 它总是返回块中最后一个表达式的值.
