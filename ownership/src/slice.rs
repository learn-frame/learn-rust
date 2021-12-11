fn main() {
    let mut s = String::from("");
    let word = first_word(&s); // word 的值为 5
    println!("{}", word); // 5
    s.clear();
    // 即便字符串 s 被置空了, word 依然是 5
    println!("{}", word); // 5

    let mut s_1 = String::from("hi yancey");
    let word_1 = first_word_1(&s_1);
    // s_1.clear(); // 当拥有某值的不可变引用时, 就不能再获取一个可变引用
    // 因为下面要打印 word_1 的引用, 因为 word_1 是不可变的, 而它依赖了 s_1 也就不能变了
    println!("the first word is: {}", word_1);

    foo();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // 因为我们从 .iter().enumerate() 中获取了集合元素的引用, 所以模式中使用了 &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn learn_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 前闭后开区间
    let from_zero = &s[..2]; // 如果从开头开始截取, 就不用写前区间
    let util_last = &s[6..]; //  如果截取到最后一个, 就不用写后区间
    let all = &s[..]; // 这种就很卷了...

    // let overflow = &s[..100]; // 溢出会在编译时报错
    // let overflow = &s[0..-1]; // 不支持负索引

    println!("{} {} {} {}", hello, from_zero, util_last, all);
}

fn first_word_1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn foo() {
    // 字符串字面量也可以使用 slice
    let str = "yancey";
    println!("{}", &str[..]);

    // 复习下数组, 下面是创建 5 个元素的数组, 元素的值为 3
    let arr = [3; 5];
    // 数组也可以使用 slice 哦, 注意打印数组要使用 {:?}
    println!("{:?}", &arr[..2]);
}
