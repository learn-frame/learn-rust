/// 错误处理指导原则
/// 
/// 以下应该使用 panic!
/// 有害状态并不包含预期会偶尔发生的错误(比如造的轮子, 使用者错误用法应该返回 panic!)
/// 在此之后代码的运行依赖于不处于这种有害状态
/// 当没有可行的手段来将有害状态信息编码进所使用的类型中的情况
/// 
/// 然而当错误预期会出现时, 返回 Result 会更优雅


mod recoverable_errors_with_result;

fn main() {
    recoverable_errors_with_result::entry();
}
