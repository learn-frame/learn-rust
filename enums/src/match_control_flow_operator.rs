/// match 是 Rust 内置的一个控制流运算符, 它允许我们将一个值与一系列的模式相比较
/// 并根据相匹配的模式执行相应代码
///
fn main() {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 以硬币为例,
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
    }
}
