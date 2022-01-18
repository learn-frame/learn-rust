use rand::Rng;
use std::cmp::Ordering;
use std::io; // 使用标准库 std 里的 io 库 // Rng 是一个 trait, 它定义了随机数生成器应实现的方法, 想使用这些方法的话, 此 trait 必须在作用域中

// 程序将会随机生成一个 1 到 100 之间的随机整数. 接着它会请玩家猜一个数并输入,
// 然后提示猜测是大了还是小了. 如果猜对了, 它会打印祝贺信息并退出.

fn main() {
    println!("Guess the number!");

    // 来个循环, 直到用户猜对了才退出程序
    loop {
        // 定义 secret_number, gen_range 也可写成 gen_range(1..=100)
        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("Please input your guess.");

        // rust 中的 let 是不可变的, 如下 name 不能被重新赋值
        // let name = "yancey";
        // name = "Sayaka"; // 报错

        // 如果一个变量是可变的, 那需要加上 mut 标识符
        // 这一句就是创建一个可变变量 guess, 并赋值为一个空字符串的实例
        // 要注意这是个字符串实例, 不是字符串字面量!!!
        let mut guess = String::new();

        // rust 的 const 必须标明其类型, 否则报错
        // Rust 中的常量并没有固定的内存地址. 这是因为实际上它们会被内联到用到它们的地方. 为此对同一常量的引用并不能保证引用到相同的内存地址.
        // const 常量可以看作一个 C 中的 #define, 它有元数据开销但无运行时开销.
        // const PI: f32 = 3.14;
        // const BASE_URL: &'static str = "https://yanceyleo.com";
        // const IS_TRUE: bool = true;

        // 使用标准流输入
        io::stdin()
            // 按行读取, 它会将用户输入追加到 guess 字符串之后
            // & 表示这个参数是一个引用(reference), 它允许多处代码访问同一处数据, 而无需在内存中多次拷贝, 这里就是传递 guess 这个变量
            // read_line 返回一个 Result 类型, 该类型是个枚举类型
            // 它的成员是 Ok 和 Err, Ok 成员表示操作成功, 内部包含成功时产生的值. Err 成员则意味着操作失败, 并且包含失败的前因后果
            .read_line(&mut guess)
            // expect 用于在错误的时候返回一串文字
            // 如果你在 read_line 后面不加上 expect, 分析器会报 warning, 因为有潜在的错误你没去处理
            // 当然后面正经代码应该使用诸如 try...catch 来处理
            .expect("Failed to read line");

        // 和 JavaScript 不同, rust 中的 let 变量允许用一个新值来隐藏(shadow) guess 之前的值
        // 因为生成的 secret_number 是个数字, 所以你需要将数字字符串转一下
        // 为了避免错误直接退出程序, 出错时继续程序.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println! 这个宏, 里面的 {}, 就是个占位符, 后面可以跟多个变量或值
        // 有点儿写 JavaScript 中的占位符, console.log("My name is %s, I'm %s.", "Yancey", "Japanese");
        // println!("My name is {}, I'm {}.", "Yancey", "Japanese");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(
                "secret_number is {}, your guess is too small",
                secret_number
            ),
            // 直到用户猜对了才退出程序
            Ordering::Equal => {
                println!("secret_number is {}, You win", secret_number);
                break;
            }
            Ordering::Greater => {
                println!("secret_number is {}, your guess is Too big", secret_number)
            }
        }
    }
}
