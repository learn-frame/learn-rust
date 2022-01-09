// 集成测试中可能用到的通用方法
// 如果不按照 tests/common/mod.rs 的路径, 而是 tests/common.rs
// common.rs 就会被当作测试文件执行, 这是没有必要的

// 此外, 如果是 src/main.rs, 就不能执行集成测试
pub fn setup() {}
