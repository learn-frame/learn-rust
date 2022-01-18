pub fn bar() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("{}", len);

    // s1 被移动到 calculate_length 中, 它也将返回值移给 len
    // 导致 s1 在这里已经被销毁了
    // 👿 报错, 因此你在 println 中拿不到 s1 了
    // println!("The length of '{}' is {}.", s1, len);
}

pub fn calculate_length(s: String) -> usize {
    s.len()
}

pub fn main_1() {
    let s1 = String::from("hello");

    // ✅ 通过使用引用就可以了
    // 它们允许你使用值但不获取其所有权
    let len = calculate_length_1(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

pub fn calculate_length_1(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这里, s 离开了作用域. 但因为它并不拥有引用值的所有权,
  // 所以什么也不会发生

// 我们将创建一个引用的行为称为借用(borrowing)
// 借用默认是不能修改原变量的, 除非你加上 mut
pub fn main_2() {
    let mut s = String::from("hello");
    change(&mut s);
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn bars() {
    let mut s = String::from("hello");

    // 👿 在同一时间只能有一个对某一特定数据的可变引用
    // 如果变两次, 会导致报错

    // 这其实就避免了数据竞争(data race)
    // 两个或更多指针同时访问同一数据.
    // 至少有一个指针被用来写入数据.
    // 没有同步数据访问的机制.
    let r1 = &mut s;
    println!("{}", r1);
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

// 当然你可以创建一个作用域
pub fn bar_1() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 在这里离开了作用域, 所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("{}", r2);
}

pub fn bar_2() {
    #[allow(unused)]
    let mut s = String::from("hello");
    println!("{}", &s);

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} {}", r1, r2);

    // let r3 = &mut s; // 👿 出错, 同时使用可变与不可变引用会出问题

    // println!("{}, {}, and {}", r1, r2, r3);
}

pub fn bar_3() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

// 悬垂引用 (Dangling References)
// 即 s 是 dangle 内部的变量, 它在函数执行完就被释放了, 因此外部不能拿到它的引用
// pub fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// 总结:
// 在任意给定时间, 要么只能有一个可变引用, 要么只能有多个不可变引用.
// 引用必须总是有效的.
