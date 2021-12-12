/// match 是 Rust 内置的一个控制流运算符, 它允许我们将一个值与一系列的模式相比较
/// 并根据相匹配的模式执行相应代码
///
fn main() {
    let x = plus_one(Some(5));
    let y = plus_one(None);
}

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
fn value_in_cents(coin: Coin) -> i8 {
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
