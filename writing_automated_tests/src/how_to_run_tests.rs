/// cargo test 生成的二进制文件的默认行为是并行的运行所有测试
/// 比如 cargo test --help, --help 是编译前的参数
/// cargo test -- --help, --help 是传递给生成的测试二进制文件
///
/// 如果你不希望测试并行运行, 可使用 cargo test -- --test-threads=1
/// 如果你的函数中有 println, 默认执行 cargo test 是不会打印出来的, 你可以使用 cargo test -- --nocapture
/// 可用 cargo test + 函数名来运行单个测试, 比如 cargo test this_test_will_pass
/// 可用 cargo test + 函数名的一部分来过滤部分函数, 比如 cargo test this 会匹配 this_test_will_pass 和 this_test_will_fail
/// 如果某个函数加了 #[ignore] 注解, 可通过 cargo test -- --ignored 来只执行这些被忽略的

#[allow(unused)]
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn say_panic() {
        panic!("error:");
    }
}
