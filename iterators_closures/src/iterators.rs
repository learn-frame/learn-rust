/// 迭代器(iterator)负责遍历序列中的每一项和决定序列何时结束的逻辑
pub fn entry() {
    create_and_consume_iterator();
    using_other_iterator_trait_methods();
}

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

// Iterator 有一个 next 方法
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

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

// 创建自己的迭代器
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
