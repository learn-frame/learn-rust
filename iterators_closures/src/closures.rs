use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

pub fn entry() {
    // 该函数的目的是如果运动强度(intensity) 小于 25
    // 就执行两次昂贵计算, 来建议用户再进行一定数量的俯卧撑和仰卧起坐
    // 如果大于等于 25, 并且 random_number 为 3, 告诉用户要休息了, 此时不需要执行昂贵计算
    // 如果大于等于 25, 并且 random_number 不为 3, 就执行一次昂贵计算, 来建议用户再进行一定数量的跑步
    generate_workout(14, 4);

    capturing_the_environment_with_closures();

    let vec = vec![1, 2, 3];
    println!("my_map: {:?}", my_map(&vec, cb));
    println!(
        "my_map_2: {:?}",
        my_map_2(&vec, |idx, val, _| format!("{} -> {}", idx, val))
    );

    use_move_2()(4);
}

#[allow(unused)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 这段代码确实能执行得起来, 但需要在不同位置调用 simulated_expensive_calculation
// 不优雅
#[allow(unused)]
fn generate_workout_before(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    closure: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(closure: T) -> Cacher<T> {
        Cacher {
            closure,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.closure)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// 通常闭包不需要注解, 像下面这个, example_closure 会优先采用第一个调用它的参数的类型
// 因此它默认的参数类型就变成了 String, 第二个因为传入了 i32, 就报错了
#[allow(unused)]
fn closure_for_annotation() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// 闭包会捕获其环境
// 即便 x 并不是 equal_to_x 的一个参数, equal_to_x 闭包也被允许使用变量 x,
// 因为它与 equal_to_x 定义于相同的作用域
fn capturing_the_environment_with_closures() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

// 函数就不行
// fn capturing_the_environment_can_not_with_function() {
//     let x = 4;

//     fn equal_to_x(z: i32) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!(equal_to_x(y));
// }

// - FnOnce 消费从周围作用域捕获的变量,闭包周围的作用域被称为其环境(environment).
//   为了消费捕获到的变量,闭包必须获取其所有权并在定义闭包时将其移动进闭包.
//   其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实,所以它只能被调用一次.
// - FnMut 获取可变的借用值所以可以改变其环境
// - Fn 从其环境获取不可变的借用值

// 当创建一个闭包时, Rust 根据其如何使用环境中变量来推断我们希望如何引用环境. 由于所有闭包都可以被调用至少一次, 所以所有闭包都实现了 FnOnce.
// 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut, 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

#[allow(unused)]
fn use_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // 因为 x 的所有权已经被 move 到闭包 equal_to_x 中了
    // 所以这里就取不到 x 了
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// 这里返回了一个闭包, 由于闭包使用了变量 i
// 而在函数调用完毕后, i 就销毁了
// 那么随闭包返回的变量 i 的引用, 也将成为悬垂指针
// 因此你必须使用 move 将 i 的所有权转移到闭包中
fn use_move_2() -> impl Fn(i32) -> i32 {
    let i = 1;
    move |j| j + i
}

fn my_map<T, S>(vec: &Vec<T>, func: fn(idx: usize, val: &T, vec: &Vec<T>) -> S) -> Vec<S> {
    let mut new_vec = Vec::new();

    for (idx, val) in vec.iter().enumerate() {
        let res = func(idx, val, &vec);
        new_vec.push(res);
    }

    new_vec
}

fn cb(_: usize, val: &i32, _: &Vec<i32>) -> i32 {
    let mut curr_val = *val;
    curr_val *= 2;
    curr_val
}

fn my_map_2<T, S, F: Fn(usize, &T, &Vec<T>) -> S>(vec: &Vec<T>, f: F) -> Vec<S> {
    let mut new_vec = Vec::new();

    for (idx, val) in vec.iter().enumerate() {
        let res = f(idx, val, vec);
        new_vec.push(res);
    }

    new_vec
}
