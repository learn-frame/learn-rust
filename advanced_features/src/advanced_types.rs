use std::fmt;

pub fn entry() {
    synonym();
}

/// 类型别名用来创建类型同义词(synonym)
/// 类型别名(type alias)的能力, 使用 type 关键字来给予现有类型另一个名字
pub fn synonym() {
    // 类型别名

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    // 因为 Kilometers 是 i32 的别名, 他们是同一类型
    // 可以将 i32 与 Kilometers 相加, 也可以将 Kilometers 传递给获取 i32 参数的函数
    println!("x + y = {}", x + y);
}

/// 类型别名的主要用途是减少重复, 比如有一个很长的类型
/// Box<dyn Fn() + Send + 'static>
/// 你可以用 type Thunk = Box<dyn Fn() + Send + 'static> 来代替.
///
/// 它还可以用来简化 Result<T, Error>
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait WriteSimple {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;

    // 否则你得写成这样的
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // fn flush(&mut self) -> Result<(), Error>;
    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

/// ! 叫做 never type, 也叫做 empty type
/// 用于在函数从不返回的时候充当返回值, 它广泛应用于 match 语句
/// 从不返回的函数被称为发散函数(diverging functions)
pub fn noop() -> ! {
    // --snip--
    todo!();
}

#[allow(unused)]
pub fn match_with_never_type() {
    let mut guess = "";

    loop {
        // 😈: 这个例子是错误的, 因为 Ok 返回的是 u32, 而 Err 返回的是 &str
        // Rust 不允许通过编译
        // let res1: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => "abc",
        // };

        // 而下面这个例子 continue 正是 never type
        let res: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型
            // 允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值
            // 相反它把控制权交回上层循环, 所以在 Err 的情况, 事实上并未对 guess 赋值.
            Err(_) => continue,
        };
    }
}

// ! 另外一个用途是 panic!
// 因为孤儿规则(Orphan Rule)这里没法直接实现 Option, 这里就不展示了
// val 是 T 类型, panic! 是 ! 类型, 但这能工作是因为 panic! 并不产生一个值, 它会终止程序
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

/// 动态大小类型和 Sized trait
/// slice 数据结构, 也就是 &str, 储存了开始位置和 slice 的长度
/// 所以虽然 &T 是一个储存了 T 所在的内存位置的单个值, 但 &str 则是两个值: str 的地址和其长度
/// 因此, &str 就有了一个在编译时可以知道的大小: 它是 usize 长度的两倍
///
/// 而动态大小类型(dynamically sized types, 也叫 DST 或者 unsized types)
/// 这些类型允许我们处理只有在运行时才知道大小的类型
///
/// Sized trait 自动为编译器在编译时就知道大小的类型实现, Rust 隐式的为每一个泛型函数增加了 Sized bound
pub fn generic<T: fmt::Debug>(t: T) {
    println!("{:?}", t);
}

// 等价于
pub fn generic1<T>(t: T)
where
    T: Sized + fmt::Debug,
{
    println!("{:?}", t);
}

/// ?Sized 上的 trait bound 意味着 “T 可能是也可能不是 Sized
/// 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则.
/// 这种意义的 ?Trait 语法只能用于 Sized, 而不能用于任何其他 trait.
/// 此外由于 T 可能不是 Sized, 后面使用了 &T
pub fn generic2<T: ?Sized + fmt::Debug>(t: &T) {
    println!("{:?}", t);
}
