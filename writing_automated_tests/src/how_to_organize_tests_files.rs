/// 单元测试(unit tests)与集成测试(integration tests)
/// 单元测试倾向于更小而更集中, 在隔离的环境中一次测试一个模块, 或者是测试私有接口.
/// 而集成测试对于你的库来说则完全是外部的. 它们与其他外部代码一样, 通过相同的方式使用你的代码, 只测试公有接口而且每个测试都有可能会测试多个模块cargo test -- --ignored
///
/// 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码, 以便于快速而准确的某个单元的代码功能是否符合预期.
/// 单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中. 规范是在每个文件中创建包含测试函数的 tests 模块, 并使用 cfg(test) 标注模块.
/// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码, 而在运行 cargo build 时不这么做
/// cfg 属性代表 configuration, 它告诉 Rust 其之后的项只应该被包含进特定配置选项中
///
/// 因为集成测试单独存在于 tests/integration_test.rs, 所以不需要配置 #[cfg(test)]

#[allow(unused)]
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

#[allow(unused)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust 是可以测试私有函数的
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

pub mod add_group {
    #[allow(unused)]
    pub fn add_one(num: i32) -> i32 {
        num + 1
    }
}
