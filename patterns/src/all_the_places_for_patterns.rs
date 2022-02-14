pub fn entry() {
    let_match(CarBrands::TOYOTA);
    while_let();
    for_pattern();
    let_pattern();
    function_params_pattern(&(3, 5));
}

#[allow(unused)]
#[derive(PartialEq)]
enum CarBrands {
    BENZ,
    BMW,
    HONDA,
    TOYOTA,
}

/// if let 条件表达式
fn let_match(brand: CarBrands) {
    // if let 可以只对某一种处理
    // if let 表达式的缺点在于其穷尽性没有为编译器所检查, 而 match 表达式则检查了.
    // 如果去掉最后的 else 块而遗漏处理一些情况, 编译器也不会警告这类可能的逻辑错误.
    if let CarBrands::BENZ = brand {
        println!("is_benz");
    } else if let CarBrands::BMW = brand {
        println!("is_bmw");
    } else if brand == CarBrands::HONDA {
        println!("is_honda");
    } else {
        println!("is_toyota");
    }
}

/// while let 条件循环
/// 它允许只要模式匹配就一直进行 while 循环
/// 这个例子会打印出 3, 2 接着是 1. pop 方法取出 vector 的最后一个元素并返回 Some(value).
/// 如果 vector 是空的, 它返回 None. while 循环只要 pop 返回 Some 就会一直运行其块中的代码.
/// 一旦其返回 None, while 循环停止. 我们可以使用 while let 来弹出栈中的每一个元素.
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// for 循环也有模式匹配
fn for_pattern() {
    let v = vec!['a', 'b', 'c'];

    // 这里使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引,
    // 他们位于一个元组中. 第一个产生的值是元组 (0, 'a').
    // 当这个值匹配模式 (index, value), index 将会是 0 而 value 将会是 'a'
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// let 语句其实也是模式匹配
fn let_pattern() {
    // let x = 5; 其实就是 let PATTERN = EXPRESSION;

    // 上面的例子可能不清晰, 下面这个例子就很清晰了
    #[allow(unused)]
    let (x, y, z) = (1, 2, 3);

    // 下面这个例子左右无法匹配, 就报错了.
    // let (x, y) = (1, 2, 3);
}

// 函数参数也是模式匹配
fn function_params_pattern(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
