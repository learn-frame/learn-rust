// 谈到 Rust 的 "字符串"时, 它们通常指的是 String 和字符串 slice &str 类型

// 通过这个函数, &str 和 String 本质是指向一块内存地址的
// 不像 Javascript, "" 是栈内存, new String("") 是堆内存, 它俩本质不是一个东西
pub fn learn_string() {
    let s_1 = "hello world";
    let s_2 = "hello world";

    match s_1 == s_2 {
        true => println!("s_1 == s_2"), // ✅
        false => println!("s_1 != s_2"),
    }

    let s_3 = String::from("hello world");
    let s_4 = String::from("hello world");

    match s_3 == s_4 {
        true => println!("s_3 == s_4"), // ✅
        false => println!("s_3 != s_4"),
    }

    let s_5 = s_1.to_string();
    match s_1 == s_5 {
        true => println!("s_1 == s_5"), // ✅
        false => println!("s_1 != s_5"),
    }
}

pub fn update_string() {
    let mut s1 = String::from("hello");
    // push_str 方法采用字符串 slice, 因为我们并不需要获取参数的所有权
    // 如果不是这样, 那么将 s2 的内容附加到 s1 之后, 自身不能被使用就糟糕了
    let s2 = " world";
    s1.push_str(s2);
    println!("{}", s1);

    // push 只能拼接一个 char 类型
    let mut s3 = String::from("hello");
    let s4 = 'l';
    s3.push(s4);

    // fn add(self, s: &str) -> String {}
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    // + 只能将 String 和 &str 相加, 不能将两个 String 值相加
    // 并且只能严格的 String + &str, 连 &str + String 也不行
    let s7 = s5 + &s6;
    // 此时, 由于 s5 的所有权将被移动到 add(+) 调用中, 下面就不能再使用 s5 了
    println!("s7: {}", s7);
    // 多个也是可以冲的
    let s8 = String::from("tic") + "-" + "tac" + "-" + "toe";
    println!("s8: {}", s8);

    // 不过上面用 + 串联显然不优雅, 你可以使用 format! 宏来替代
    // 这个版本就好理解的多, 并且不会获取任何参数的所有权.
    // 并且你可以加入任何类型, char, String, &str, 数字等等
    let s9 = format!(
        "{}-{}-{}-{}-{}",
        String::from("hello"),
        " ",
        String::from("world"),
        '!',
        1
    );
    println!("s9: {}", s9);
}

pub fn index_string() {
    let s_1 = "hello world";
    let s_2 = String::from("hello world");
    // 😈 Rust 的字符串不支持索引
    // println!("s_1[0]: {}", s_1[0]);
    // println!("s_2[0]: {}", s_2[0]);

    // String 是一个 Vec<u8> 的封装
    println!("{}", String::from("Hola").len()); // 4

    // 像这种毛子的语言, 一个 Unicode 标量值需要两个字节存储
    // 所以如果你用索引语法, 对于这种双字节的字符, 就没得取了
    // 英明的 rust 干脆一刀切, 不让你用索引语法了
    println!("{}", String::from("Здравствуйте").len()); // 24
}

// 基于上述原因, 你无法在字符串上使用索引语法
// 但你可以通过获取字符串中的某个范围来截取字符串
pub fn slice_string() {
    let s_1 = "hello world";
    let s_2 = &s_1[0..5];
    println!("s_2: {}", s_2); // hello

    let s_3 = "Здравствуйте";
    println!("{}", &s_3[0..4]); // Зд

    // 但下面这种会触发运行时 panic, 因为 s_3 中的字符串是双字节的
    // 只取了一个字节, 导致出错
    // println!("{}", &s_3[0..1]);
}

pub fn traverse_string() {
    let s = "こんにちは";

    // こ
    // ん
    // に
    // ち
    // は
    for c in s.chars() {
        println!("{}", c);
    }

    // 227
    // 129
    // 147
    // 227
    // 130
    // 147
    // 227
    // 129
    // 171
    // 227
    // 129
    // 161
    // 227
    // 129
    // 175
    for b in s.bytes() {
        println!("{}", b);
    }
}
