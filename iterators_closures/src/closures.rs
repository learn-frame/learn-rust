/// 闭包 (Closure)通常是指词法闭包, 是一个持有外部环境变量的函数. 外部环境是指闭包定义时所在的词法作用域
/// 外部环境变量, 在函数式编程范式中也被称为**自由变量**, 是指并不是在闭包内定义的变量. 将自由变量和自身绑定的函数就是闭包.
///
/// Rust 的函数是不能直接使用外部变量的(除了 const 和 static)
/// 因此函数只能通过传参的方式使用外部变量
/// 而闭包则可以使用外部变量
///
/// 闭包有两大特性:
/// - **延迟执行**: 返回的闭包只有在需要调用的时候才会执行.
/// - **捕获环境变量**: 闭包会获取其定义时所在作用域中的自由变量, 以供之后调用时使用.
///
/// 逃逸闭包和非逃逸闭包
/// 在函数调用之后才会使用的闭包, 叫作逃逸闭包(escape closure). 因为该闭包捕获的环境变量逃离了函数的栈帧, 所以在函数栈帧销毁之后依然可用.
/// 与之相对应, 如果是跟随函数一起调用的闭包, 则是非逃逸闭包 (non-escape closure).
/// 只有逃逸闭包才能装箱, 可以用 'static 生命周期用来约束该闭包必须是一个逃逸闭包
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
    generate_workout(14, 4);
    capturing_the_environment_with_closures();

    clousures_as_return_val();

    fn_once();
    r#fn();
    fn_mut();
    use_move_1();
    use_move_2()(4);

    use_trait_bound_closure();
    use_fn_params_closure();
}

#[allow(unused)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

/// 该函数的目的是如果运动强度(intensity) 小于 25
/// 就执行两次昂贵计算, 来建议用户再进行一定数量的俯卧撑和仰卧起坐
/// 如果大于等于 25, 并且 random_number 为 3, 告诉用户要休息了, 此时不需要执行昂贵计算
/// 如果大于等于 25, 并且 random_number 不为 3, 就执行一次昂贵计算, 来建议用户再进行一定数量的跑步
///
/// 这段代码确实能执行得起来, 但需要在不同位置调用 simulated_expensive_calculation, 不优雅
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

/// 闭包作为返回值
/// 因为闭包的大小在编译期是未知, 你得返回一个动态包装类型
fn return_closures_old(b: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |a| a + b)
}

fn return_fnonce_closures_old(b: i32) -> Box<dyn FnOnce(i32) -> i32> {
    Box::new(move |a| a + b)
}

/// 现如今不用这么折腾了, 统一用 impl trait 吧, 累了
fn return_closures(b: i32) -> impl Fn(i32) -> i32 {
    move |a| a + b
}

fn clousures_as_return_val() {
    let f1 = return_closures_old(1);
    let f2 = return_closures(1);
    let f3 = return_fnonce_closures_old(1);
    f1(1);
    f2(1);
    f3(1);
}

// - FnOnce 消费从周围作用域捕获的变量,闭包周围的作用域被称为其环境(environment).
//   为了消费捕获到的变量,闭包必须获取其所有权并在定义闭包时将其移动进闭包.
//   其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实,所以它只能被调用一次.
// - FnMut 获取可变的借用值所以可以改变其环境
// - Fn 从其环境获取不可变的借用值

// 当创建一个闭包时, Rust 根据其如何使用环境中变量来推断我们希望如何引用环境. 由于所有闭包都可以被调用至少一次, 所以所有闭包都实现了 FnOnce.
// 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut, 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

/// - 如果闭包中没有捕获任何环境变量, 则默认自动实现 Fn
/// - 如果闭包中捕获了复制语义类型的环境变量, 则:
///   - 如果不需要修改环境变量, 无论是否使用 move 关键字, 均会自动实现 Fn.
///   - 如果需要修改环境变量, 则自动实现 FnMut.
/// - 如果闭包中捕获了移动语义类型的环境变量, 则:
///   - 如果不需要修改环境变量, 且没有使用 move 关键字, 则自动实现 FnOnce.
///   - 如果不需要修改环境变量, 且使用了 move 关键字, 则自动实现 Fn.
///   - 如果需要修改环境变量, 则自动实现 FnMut.
/// - 使用 move 关键字, 如果捕获的变量是复制语义类型的, 则闭包会自动实现 Copy/Clone, 否则不会自动实现 Copy/Clone.

fn fn_once() {
    // s 是移动语义类型, 导致闭包 c 默认实现了 FnOnce Trait
    let s = "hello".to_string();
    let c = || s;

    c();

    // 😈: 因此 c 不能调用多次
    // c();

    // 😈: s 也被转移了所有权
    // println!("{}", s);
}

fn r#fn() {
    // s 是复制语义类型, 因此闭包默认实现 Fn Trait
    let s = "hello";
    let c = || s;

    c();
    c();

    println!("{}", s);
}

fn fn_mut() {
    let mut s = "rush".to_string();
    // 由于捕获的 s 是可变的, 闭包必须是 mut 的
    let mut c = || s += " rust";

    c();
    c();

    println!("{}", s);
}

fn use_move_1() {
    // 这里的 s 没有作为闭包的返回值, 只是打印了下
    // 因此闭包实现的是 Fn, 即可以多次使用闭包
    let s = "hello".to_string();
    let c = || println!("{}", s);
    c();
    c();
    println!("{}", s);

    // 你可以人为地将 s 移动到到闭包中, 即 s 的所有权被转移到 c 中
    let s = "hello".to_string();
    let c = move || println!("{}", s);
    c();
    c();
    // 😈: 这里就无法获取 s 了
    // println!("{}", s);
}

// 这里返回了一个闭包, 由于闭包使用了变量 i
// 而在函数调用完毕后, i 就销毁了
// 那么随闭包返回的变量 i 的引用, 也将成为悬垂指针
// 因此你必须使用 move 将 i 的所有权转移到闭包中
fn use_move_2() -> impl Fn(i32) -> i32 {
    let i = 1;
    move |j| j + i
}

// 函数作为函数参数
fn my_map<T, S>(vec: &Vec<T>, func: fn(idx: usize, val: &T, vec: &Vec<T>) -> S) -> Vec<S> {
    let mut new_vec = Vec::new();

    for (idx, val) in vec.iter().enumerate() {
        let res = func(idx, val, &vec);
        new_vec.push(res);
    }

    new_vec
}

// 闭包作为函数参数
fn my_map_2<T, S, F>(vec: &Vec<T>, f: F) -> Vec<S>
where
    F: Fn(usize, &T, &Vec<T>) -> S,
{
    let mut new_vec = Vec::new();

    for (idx, val) in vec.iter().enumerate() {
        let res = f(idx, val, vec);
        new_vec.push(res);
    }

    new_vec
}

fn use_fn_params_closure() {
    let vec = vec![1, 2, 3];

    fn cb(_: usize, val: &i32, _: &Vec<i32>) -> i32 {
        let mut curr_val = *val;
        curr_val *= 2;
        curr_val
    }
    println!("my_map: {:?}", my_map(&vec, cb));

    println!(
        "my_map_2: {:?}",
        my_map_2(&vec, |idx, val, _| format!("{} -> {}", idx, val))
    );
}

// 以 trait 限定的方式实现 any 闭包
trait Any<T> {
    // 静态分发
    fn any<F>(&self, f: F) -> bool
    where
        F: Fn(T) -> bool;

    // trait object
    fn any_obj<F>(&self, f: &dyn Fn(u32) -> bool) -> bool;

    // impl trait
    fn any_impl_obj<F>(&self, f: impl Fn(u32) -> bool) -> bool;
}

impl Any<u32> for Vec<u32> {
    fn any<F>(&self, f: F) -> bool
    where
        F: Fn(u32) -> bool,
    {
        for &i in self {
            if f(i) {
                return true;
            }
        }

        return false;
    }

    fn any_obj<F>(&self, f: &(dyn Fn(u32) -> bool)) -> bool {
        for &i in self {
            if f(i) {
                return true;
            }
        }

        return false;
    }

    fn any_impl_obj<F>(&self, f: impl Fn(u32) -> bool) -> bool {
        for &i in self {
            if f(i) {
                return true;
            }
        }

        return false;
    }
}

fn use_trait_bound_closure() {
    let v: Vec<u32> = vec![5, 1, 2, 4, 5];

    // any 的参数可以是个闭包
    let b = v.any(|x| x > 3);
    assert!(!b);

    // any 的参数也可以是一个函数指针
    fn c(x: u32) -> bool {
        x > 3
    }
    let d = v.any(c);
    assert!(!d);
}
