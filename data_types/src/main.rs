// Rust 是静态类型(statically typed)语言, 也就是说在编译时就必须知道所有变量的类型
// use std::num::Wrapping;

use std::{
    f32::{INFINITY, NEG_INFINITY},
    i32::{MAX, MIN},
    ops::{Bound::*, RangeBounds},
};

fn main() {
    // 当多种类型均有可能时, 必须增加类型注解
    // 必须显式指定变量的数据类型
    let number_from_string: u32 = "42".parse().expect("必须是个数字字符串啊喂");
    println!("{}", number_from_string);

    // 标量类型
    // Rust 有四种基本的标量类型: 整型, 浮点型, 布尔类型和字符类型(characters)

    // 整型
    // Length	 Signed	                                    Unsigned                                备注
    //
    // 8-bit	 i8(-2^⁷ - 2^⁷-1)	                        u8(0 - 2^⁸-1)                           占用 1 个字节, u8 类型通常在 Rust 中表示字节序列, 在文件 I/O 或网络 I/O 中读取数据流时需要使用 u8.
    // 16-bit	 i16(-2^¹⁵ - 2^¹⁵-1)	  	                u16(0 - 2^¹⁶-1)                         占用 2 个字节
    // 32-bit	 i32(-2^³¹ - 2^³¹-1)	  	                u32(0 - 2^³²-1)                         占用 4 个字节
    // 64-bit	 i64(-2^⁶³ - 2^⁶³-1)	  	                u64(0 - 2^⁶⁴-1)                         占用 8 个字节
    // 128-bit	 i128(-2^¹²⁷ - 2^¹²⁷-1)	  	                u128(0 - 2^¹²⁸-1)                       占用 16 个字节
    // arch	     isize(-2^³¹ - 2^³¹-1 或 -2^⁶³ - 2^⁶³-1)  	usize(0 - 2^³²-1 或 0 - 2^⁶⁴-1)         占用 4 或 8 个字节, 具体取决于机器的字长, 字长可以理解为 32位系统 / 64 位系统
    
    // 有符号数: 可以为正数, 也可以为负数, 和 0; 有符号数以补码形式(two's complement representation) 存储; 每一个有符号的变体可以储存包含从 -(2^n - 1) 到 2^(n-1) - 1 在内的数字
    // 无符号数: 只能为正整数和 0; 无符号的变体可以储存从 0 到 2^(n-1) 的数字
    // isize 和 usize 类型依赖运行程序的计算机架构:64 位架构上它们是 64 位的, 32 位架构上它们是 32 位的

    // 数字字面值	                 例子
    // Decimal (十进制)	             98_222
    // Hex (十六进制)	             0xff
    // Octal (八进制)	             0o77
    // Binary (二进制)	             0b1111_0000
    // Byte (单字节字符)(仅限于u8)	  b'A'

    // 整型支持二, 八, 十, 十六进制和单字节字符
    // rust 允许使用 _ 做为分隔符以方便读数, 如 1_000_000 等价于 1000000

    // 整型溢出, 因为 i8 最大到 255
    // 这句话会在编译阶段报错, 但在 release 阶段, 会将 256 变成 0, 257 变成 1, 依此类推. 这叫做二进制补码包装(two's complement wrapping).
    // 标准库中有一个 Wrapping 来显式处理溢出: https://doc.rust-lang.org/std/num/struct.Wrapping.html
    // 不过 Wrapping 大多都属于实验中的
    // let num: i8 = 256;

    // 浮点型
    // 有 f32 和 f64 两种, 分别占 32 位和 64 位, 默认是 f64
    // 浮点数采用 IEEE-754 标准表示. f32 是单精度浮点数, f64 是双精度浮点数
    println!("{}", 0.1 + 0.2); // 0.30000000000000004 哈哈哈我就知道

    // 数值运算
    // 所有运算符列表: https://doc.rust-lang.org/book/appendix-02-operators.html
    println!("{}", 4.0 / 3.0); // 1.3333333333333333, 如果是 float 运算产生小数, 会带小数
    println!("{}", 4 / 3); // 1, 如果两个整型产生小数, 会把小数抹掉, 这在算法题大数加法相关题目中求商很有用.

    let a: i8 = 4;
    let b: i8 = 0b1111;
    let c: f32 = 1.0;
    let d: i32 = 4;
    let e: f64 = 2.0;
    let f = 4u32; // 可以通过直接使用类型后缀来告知编译器这个整型数字是个 u32 的, 否则默认被推算成 i32
    let g = MAX;
    let i = MIN;
    let j = INFINITY;
    let k = NEG_INFINITY;
    assert_eq!(b'A', 65u8); // 二进制可以用 b 前缀

    // 👿 必须是相同类型才能做加减乘除取余
    println!("{}", a + b);
    println!("{} {} {} {}", c, d, e, f);
    println!("{} {} {} {}", g, i, j, k);

    // 👿 i8 和 i32 无法进行运算
    // println!("{}", a + d);

    // 👿 int 类型和 float 类型不能进行运算, 诸如此类, 云云.
    // println!("{}", c * a);

    // 布尔型
    let t = true;
    let f: bool = false; // 显式指定类型注解
    let truely = t as i32; // 可以通过 as 将 bool -> i32, u32, i8, usize... 但不可以转成其他类型, 比如 float, str; 此外只能 bool -> int, 反过来不行
    let falsy = f as i32;
    println!("{} {} {} {}", t, f, truely, falsy);

    // 字符类型
    // 字符类型代表的是一个 Unicode 标量值.
    // 字符类型是语言中最原生的字母类型, 它必须是单个字符, 比如 'a', 'b', 但可以是 '𝌆' '😻' 等四字符.
    // 注意要跟字符串区分, 字符类型用的是单引号, 字符串是双引号
    // 具体区分: https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
    // Rust 的 char 类型的大小为四个字节, 并代表了一个 Unicode 标量值(Unicode Scalar Value)
    let f = 'z';
    let g = 'ℤ';
    let h = '😻';
    let i = '\x2A';
    let j = '\u{CA0}';

    // 使用 as 操作符将字符转为数字类型, 比如 % 的十进制 ASCII 是 37, 那结果就是 37
    // ಠ 是 3232, 如果强制转 i8 的话, 高位会被截取, 变成 -96
    let k = '%' as i8; // 37
    let l = 'ಠ' as i8; // 96
    println!("{} {} {} {} {} {} {}", f, g, h, i, j, k, l);

    //: 复合类型
    //: Rust 有两种基本的复合类型: 元组, 数组

    /* 元组 */
    // let a = [3; 5];
    let tup = (500, 6.4, "str");
    // 根据下标获取元组元素
    println!("{}, {}", tup.0, tup.1);
    // 解构元组, 因为 let 支持模式匹配, 所以可以用来解构元组.
    // 如果使用元组解构, 你必须把所有元素都解构出来, 否则报错...
    let (i, j, k) = tup;
    println!("The value of i is: {}", i);
    println!("{} {}", j, k);
    // 枚举元组中的所有值
    println!("{:?}", tup);
    // 很长的元组无法打印, 这么操蛋的吗...
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 没有任何值的元组 () 是一种特殊的类型, 只有一个值, 也写成 (). 该类型被称为 单元类型(unit type),
    // 而该值被称为 单元值(unit value). 如果表达式不返回任何其他值, 则会隐式返回单元值
    let l = ();
    println!("{:?}", l);
    let tup = (0,); // 如果元组只有一个元素, 必须加一个逗号
    println!("{:?}", tup);

    /* 数组 [T; N], 必须是一致的类型, 且长度必须为编译时常量, 默认不可变 */
    // 对于原始固定长度数组, 只有实现 Copy trait 的类型才能作为其元素, 也就是说, 只有可以在枝上存放的元素才可以存放在该类型的数组中

    // let m = [1, 2, 3, 4, 5, ""]; // 👿 报错, 必须是一致的类型
    let mut n = ["Yancey", "Sayaka"];

    // 数组一旦创建就不可修改长度了, 它没有 push pop 等方法
    // 此外和其他数据类型一样默认是不可变的, 除非你加上 mut, 但即便加上了 mut, 也只能修改己存在于索号位上的元素
    n[1] = "Yancey"; // 可以
                     // n[5] = "hello"; // 👿 运行时出错, 编译时是无法检测数组访问元素溢出的

    // for...in
    for val in n.iter() {
        println!("val is :{}", val);
    }

    // 对于长度固定的, 用数组比较好, 可变的建议用 vector
    let o = vec![1, 2, 3];
    println!("{:?}", o);
    // println!("{:?}", o[4]);  // 👿 也运行时出错, 编译时是无法检测数组访问元素溢出的

    // 这个就比较骚, 等价于 [3, 3, 3, 3, 3]
    let p = [3; 5];
    println!("{:?}", p);

    // 我爆哭 qwq, 用 js 刷动态规划的 leetcode 题, 初始化二维数组贼麻烦.
    let q = [[""; 5]; 5];

    // 对于数组的打印, 和元组类似, 都得用 {:?}
    println!("{:?}", q);
    let u = n[0];
    println!("{:?}", u);

    /* 范围类型 */
    assert_eq!(std::ops::Range { start: 1, end: 5 }, (1..5)); // 前闭后开
    assert_eq!(std::ops::RangeInclusive::new(1, 5), (1..=5)); // 前闭后闭
    assert_eq!(1 + 2 + 3, (1..=3).sum()); // 范围类型自带一些方法, 比如 sum
    assert!((1..=3).contains(&2)); // 范围类型自带一些方法, 比如 contains
    assert_eq!((1..).end_bound(), Unbounded); // 范围类型自带一些方法, 比如 end_bound
    assert_eq!((..1).start_bound(), Unbounded); // 范围类型自带一些方法, 比如 start_bound
    assert_eq!((1..3).end_bound(), Excluded(&3)); // 范围类型自带一些方法, 比如 end_bound
    assert_eq!((1..=3).end_bound(), Included(&3)); // 范围类型自带一些方法, 比如 end_bound
    for _ in 1..=3 {} // 范围类型自带迭代器, 可以直接使用 for...in

    /* 切片类型 */
    // 切片(Slice)类型是对一个数组(包括固定大小数组和动态数组)的引用片段, 签名为 &[T] 和 &mut [T].
    let arr = [1, 2, 3, 4];
    let vec = vec![1, 2, 3, 4];
    // let arr_2 = arr[1..]; 👿 不能对数组/可变数组这样切
    assert_eq!(&arr[1..], [2, 3, 4]); // 只能使用切片
    assert_eq!(&vec[1..], [2, 3, 4]); // 只能使用切片
    assert_eq!(&vec[1..].len(), &3); // len 方法, 是 const fn 方法
    assert!(&vec[4..].is_empty()); // is_empty 方法, 是 const fn 方法

    /* str 字符串类型 */
    let str: &'static str = "Hello, world"; // 'static 的生命周期能够存活于整个程序期间. 所有的字符串字面值都拥有 'static 生命周期
    let ptr = str.as_ptr();
    let len = str.len();
    let str_1 = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(str_1, Ok(str));

    /* 原生指针 */
    // 表示内存地址的类型称为指针, Rust 提供了多种指针类型:
    // 引用(Reference)
    // 原生指针(Raw Pointer)
    // 函数指针(fn Pointer)
    // 智能指针(Smart Pointer)

    // Rust 分为 Safe Rust 和 Unsafe Rust
    // 引用是一种非空指针, 在 Safe Rust 中, 编译器会对引用进行借用检查, 以保证内存安全和类型安全
    // 而原生指针是不安全的, 比如原生指针可能指向 Null, 或者一个已经被释放的内存区域(悬垂引用)
    // 因为使用原生指针的地方不在 Safe Rust 的可控范围内，所以需要程序员自己保证安全.
    // Rust 支持两种原生指针:
    // 不可变原生指针 *const T
    // 可变原生指针 *mut T

    {
        let mut x = 10;
        let ptr_x = &mut x as *mut i32; // 通过 as 将 &mut x 可变引用转换为 *mut i32 可变原生指针 ptr_x
        let y = Box::new(20); // 通过 Box 将数字 20 存到堆内存上
        let ptr_y = &*y as *const i32; // 将 y 转换成原生指针 ptr_y

        unsafe {
            *ptr_x += *ptr_y; // 两个原生指针相加
        }
        assert_eq!(x, 30);
    }

    /* 零大小类型(ZST) */
    // 单元类型和单元结构体大小为零, 单元类型组成的数组大小也为零
    // ZST 类型的特点是, 它们的值就是其本身, 运行时并不占用内存空间. ZST 类型代表的意义正是空
    // 比较有趣的实现就是 HashSet<T> 实际上是由 HashMap<K, ()> 发展来了.
    enum Void {}
    struct Foo;
    #[allow(unused)]
    struct Baz {
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    // 可通过 std::mem::size_of() 来查找一个表达式占用内存的字节大小
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);

    let vec = vec![(); 10];
    // 使用这种方式要比 0..10 获得较高的性能, 不过也无所谓吧
    for i in vec {
        println!("{:?}", i);
    }

    for i in 0..10 {
        println!("{:?}", i);
    }

    /* never 类型 */
    // nerver 类型(!)是一个没有值的类型, 表示永远不会完成计算的结果. 由于 Rust 中的底类型用叹号(!) 表示, 此类型也被称为 Bang Type
    // nerver 类型系统又叫底类型(Bottom Type), 底类型源自类型理论的术语, 它的特点是没有值, 其次是是其他任意类型的子类型.
    // 如果说 ZST 类型表示空的话，那么底类型就表示无. 底类型无值, 而且它可以等价于任意类型.
    // Rust 中的底类型用叹号表示, 此类型也被称为 BangType, Rust 中有很多种情况确实没有值, 但为了类型安全, 必须把这些情况纳入类型系统进行统一处理, 这些情况包括:
    // - 发散函数(Diverging Function), 指会导致线程崩溃的 panic!(), 或者用于退出函数的 std::process::exit
    // - continue 和 break 关键字, 它们只是表示流程的跳转, 并不会返回什么
    // - loop 循环, loop 循环虽然可以返回某个值, 但也有需要无限循环的时候.
    // - 空枚举, 比如 enum Void {}, 它完全没有任何成员, 因而无法对其进行变量绑定, 不知道如何初始化并使用它, 所以它也是底类型.

    // ! 的类型表达式可以强转为任何其他类型. 目前 ! 只在 nightly 版本, 还不稳定.
    // let x: ! = panic!();
    // let y: u32 = x;
    // 返回 ! 类型

    fn foo() -> ! {
        loop {
            println!("nerver");
        }
    }

    #[allow(unused)]
    let i = if false {
        foo();
    } else {
        100
    };

    /* 类型推导与 Turbofish 操作符 */
    // Rust 的类型推导并不强大, 只能在局部范围内进行类型推导
    // 当 Rust 无法从上下文中自动推导出类型的时候，编译器会通过错误信息提示你
    let x = "1";
    // println!("{:?}", x.parse().unwrap()); // 😈 can't call method `parse` on ambiguous numeric type `{integer}`, 因为 Rust 中的数字类型很多,  parse 不知道你想让 x 转成 i32 还是其他, 因此你需要自己指明
    // 因此你可以给表达式赋值给某个变量, 然后给这个变量标注明确的值
    let int_x: i32 = x.parse().unwrap(); // 这样是可以的
    println!("{:?}", int_x);
    // Rust 还提供了一种标注类型的方法, 用于方便地在值表达式中直接标注类型
    // 下面这个代码使用了 parse::<i32>() 这样的形式为泛型函数标注类型, 就避免了新建一个变量, 这种标注类型 ::<> 的形式就叫作 turbofish 操作符
    println!("{:?}", x.parse::<i32>().unwrap());
    // Rust 的类型推导还是不够强大, 下面这个居然出错了.
    // let b = 1.is_positive(); // 😈 you must specify a type for this binding, like `i32`: `x: i32`
    // 你必须给它明确的变量
    assert!((-1 as i32).is_negative());
}

// 类型系统与多态性
// 如果一个类型系统允许一段代码在不同的上下文中具有不同的类型, 这样的类型系统就叫作多态类型系统. 对于静态类型的语言来说, 多态性的好处是可以在不影响类型丰富的前提下, 为不同的类型编写通用的代码.
// 现代编程语言包含了三种多态形式: 参数化多态(Parametric polymorphism), Ad-hoc 多态(Ad-hoc polymorphism)和子类型多态(Subtype polymorphism).
// 如果按照多态发生时间来划分, 又可分为静多态(Static polymorphism)和动多态(Dynamic polymorphism). 静多态发生在编译期, 动多态发生在运行时.
// 参数化多态和 Ad-hoc 多态一般是静多态, 子类型多态一般是动多态. 静多态牺牲灵活性获取性能, 动多态牺牲性能获取灵活性. 动多态在运行时需要查表, 占用较多空间, 所以一般情况下都使用静多态. Rust 语言同时支持静多态和动多态, 静多态就是一种零成本抽象.
// 参数化多态实际就是指泛型, Ad-hoc 多态也叫特定多态, Rust 中的类型系统目前只支持参数化多态和 Ad-hoc 多态, 也就是泛型和 trait
