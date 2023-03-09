/// 迭代器(iterator)负责遍历序列中的每一项和决定序列何时结束的逻辑
pub fn entry() {
    create_and_consume_iterator();
    using_other_iterator_trait_methods();
    size_hint();
}

/// 迭代器, 本质就是实现了 Iterator Trait, for 循环就是其中一个语法糖
// 我们可以创建自己的迭代器
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// 通过定义 next 方法实现 Iterator trait,
// 我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了, 因为他们都使用了 next 方法的功能
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("sum: {}", sum);
}

/// size_hint
///
/// 返回一个 (usize, Option<usize>) 的元组, 此元组表示迭代器剩余长度的边界信息
/// 元组中第一个元素表示下限(lower bound), 第二个元素表示上限(upper bound).
/// 第二个元素是 Option<usize> 类型, 代表己知上限或者上限超过 usize 的最大取值范围, 比如无穷迭代.
/// 此方法的默认返回值 (0,None) 适用于任何迭代器.
fn size_hint() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    assert_eq!((3, Some(3)), iter.size_hint());

    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());

    iter.next();
    assert_eq!((1, Some(1)), iter.size_hint());

    iter.next();
    assert_eq!((0, Some(0)), iter.size_hint());

    iter.next();
    assert_eq!((0, Some(0)), iter.size_hint());

    // size hint的目的就是优化迭代器, 如果事先知道准确的迭代器长度,
    // 就可以做到精准地扩展容器容量, 从而避免不必要的容量检查, 提高性能

    // 比如 String 有个 extends 方法, 它传入一个迭代器,
    // 用于将迭代器内部的值逐一追加到该字符串的后面
    let mut str = "Hello".to_string();
    str.extend(&vec![' ', 'R', 'u', 's', 't']);
    assert_eq!("Hello Rust", str);

    // 具体的实现, 它获取 size_hint 方法返回的元组的第一个参数 lower_bound
    // 然后 self.reserve(lower_bound), 目的是告诉该字符串将要扩展至少 lower_bound 个字符
    // 这样做是为了避免频繁分配, 示例中迭代器长度是 5, 即至少需要分配 20 个字节(因为每个 char 类型占 4 个字节)
    // 最后将迭代器的元素追加到字符串中
    // fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
    //     let iterator = iter.into_iter();
    //     let (lower_bound, _) = iterator.size_hint();
    //     self.reserve(lower_bound);
    //     iterator.for_each(move |c| self.push(c));
    // }
}

/// Intolterator 和 Fromlterator
/// Fromlterator 可以从迭代器转换为指定类型, 而 Intolterator 可以从指定类型转换为迭代器

// 和其他语言一样, 迭代器是惰性的, 这意味着在调用方法使用迭代器之前它都不会有效果
pub fn create_and_consume_iterator() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("Got: {}", val);
    }

    for (i, &item) in v1.iter().enumerate() {
        println!("Got: {} -> {}", i, &item);
    }

    // 消费迭代器
    let sum: i32 = v1.iter().sum();
    println!("sum: {}", sum);

    // Iterator trait 中定义了另一类方法, 被称为迭代器适配器(iterator adaptors)
    // 他们允许我们将当前迭代器变为不同类型的迭代器. 可以链式调用多个迭代器适配器
    // 但是由于迭代器都是惰性, 必须要调用一个可以"消费"的方法, 比如 collect.
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).filter(|x| x > &2).collect();
    println!("v2: {:?}", v2); // [3, 4]
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() 会获取 shoes 的所有权, 而 iter() 只是借用
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
