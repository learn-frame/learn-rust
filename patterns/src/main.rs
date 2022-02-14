//! 模式由如下一些内容组合而成
//! 字面值
//! 解构的数组, 枚举, 结构体或者元组
//! 变量
//! 通配符
//! 占位符
//! match 表达式必须是穷尽(exhaustive)的, 意为 match 表达式所有可能的值都必须被考虑到
//! 当然你可以使用 _ 来匹配所有情况

pub mod all_the_places_for_patterns;
pub mod pattern_grammer;
pub mod refutability;

fn main() {
    all_the_places_for_patterns::entry();
    refutability::entry();
    pattern_grammer::entry();
}
