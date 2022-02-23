pub fn entry() {
    do_twice(add_one, 6);
    enum_tuple();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

/// 函数指针, 普通函数也可以被当作另一个函数的参数使用
/// fn 被称为函数指针(function pointer)
/// 函数指针实现了所有三个闭包 trait(Fn, FnMut 和 FnOnce)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn closure_and_fn_pointer() {
    let list_of_numbers = vec![1, 2, 3];

    // 这个使用闭包, 没什么好说的
    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // 这个则使用 to_string 这个函数, 注意要使用完全限定语法, 即 ToString::to_string,
    // 因为存在多个叫做 to_string 的函数
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?} {:?}", list_of_strings1, list_of_strings2);

    // 这种 JavaScript 中也很常见
    //
    // ```ts
    // const arr = [0,1,2,3]
    // arr.filter(v => !!v)
    // arr.filter(Boolean)
    // ```
}

// 枚举值也可以当作 map 函数中传入的函数
pub fn enum_tuple() {
    #[derive(Debug)]
    #[allow(unused)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (1u32..10).map(Status::Value).collect();
    // [Value(1), Value(2), Value(3), Value(4), Value(5), Value(6), Value(7), Value(8), Value(9)]
    println!("{:?}", list_of_statuses);
}

// 闭包表现为 trait, 这意味着不能直接返回闭包
// 不过话说为什么要返回闭包?
pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}