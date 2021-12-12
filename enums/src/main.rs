pub mod match_control_flow_operator;

fn main() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("fe80::a8aa:ff:fe10:d81c"));
    println!("ipv4: {:#?}, ipv6: {:#?}", ipv4, ipv6);

    // impl 来为枚举定义方法
    let instance = IpAddr::V6(String::from("fe80::a8aa:ff:fe10:d81c"));
    instance.print_ip();
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举 / 结构体都可被 impl
impl IpAddr {
    fn print_ip(&self) {
        dbg!(self);
    }
}

// Option 意味着某个变量有确切的值, 或者为空为"空".
fn learn_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x = 1;

    // 😈 下面这个是错的, 简单来说 some_number 是 Option<i32> 类型, 而 x 是 i32 类型, 必然无法加和
    // let sum = x + some_number;
}

// 这种强制分头计算的思路
// 来限制空值的泛滥以增加 Rust 代码的安全性
fn plus_one(x: Option<i32>) -> Option<i32> {
    // 你可以通过 match 来分别进行计算
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
