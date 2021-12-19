/// match 是 Rust 内置的一个控制流运算符, 它允许我们将一个值与一系列的模式相比较
/// 并根据相匹配的模式执行相应代码

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Other,
}

// 以硬币为例
pub fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        // match 其实就类似于 switch...case
        // _ 就类似于 default
        // 就酱
        _ => 0,
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// ----- if let -----

// 如果使用 match, 需要把所有的情况都列举处理
pub fn foo() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

// 使用 if let 可以只对某一种处理
pub fn bar() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// 枚举也可以使用
pub fn bars() {
    let mut count = 0;
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("count: {}!", count);
    }
}
