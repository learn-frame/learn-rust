/// 变量有两种: 全局变量和局部变量. 全局变量分为常量变量和静态变量. 局部变量是指在函数中定义的变量.
fn main() {
    // rust 中的 let 变量允许用一个新值来隐藏(shadow) 旧值, 并且可以改变其数据类型
    #[allow(unused)]
    let x = 0;
    #[allow(unused)]
    let x = "";

    // 👿 然而变量默认是不可变的, 因此你不能重新赋值
    #[allow(unused)]
    let y = 0;
    // y = 1;

    // 除非你加上 mut, 表示这个变量的值是可变的
    #[allow(unused)]
    let mut z = 1.0;
    #[allow(unused)]
    let mut z = z + 2.0;
    z = 3.0;
    println!("{}", z);
    // 👿 需要注意, 虽然值是可变的, 但它的类型是不可变的, 如不能从 float64 变成字符串
    // z = "";

    // 常量没有固定的内存地址, 因为其生命周期是全局的, 随着程序消亡而消亡, 并且会被编译器有效地内联到每个使用到它的地方.
    // 你必须为常量提供它的类型
    #[allow(unused)]
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // 👿 常量不具备 shadow 效果
    // const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;

    // 静态变量的生命周期也是全局的, 但它并不会被内联, 每个静态变量都有一个固定的内存地址
    // 静态变量并非被分配到栈中, 也不是在堆中, 而是和程序代码一起被存储于静态存储区中.
    // 静态存储区是伴随着程序的二进制文件的生成(编译时)被分配的, 并且在程序的整个运行期都会存在. Rust 中的字街串字面量同样是存储于静态内存中的.
    #[allow(unused)]
    static A: i32 = 1 * 2;

    // 🦀️ shadowing 是可以改变数据类型的
    // mut 是不可以改变数据类型的, 只能改变值
    #[allow(unused)]
    let space = "";

    #[allow(unused)]
    let space = 1;

    {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x); // 12
        }

        println!("The value of x is: {}", x); // 6
    }

    // rust 必须给变量赋初值, 如下代码就是错的
    // let y: i32;
    // println!("{}", y);

    // 除非你这么写, 注意你要是把 else 去掉, 也不行
    let y: i32;
    if true {
        y = 1;
    } else {
        y = 2;
    }
    println!("{}", y);
}

// 总结:
// let 可改变数据类型
// let mut 不可改变数据类型
// let 和 let mut 都可以 shadow
// const 必须显式标明数据类型
