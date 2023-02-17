pub fn entry() {
    let list = vec![3, 0, 6, 1, 4, 5];
    println!("The largest element is {}.", get_largest(&list));
}

pub fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(unused)]
// 结构体可以定义泛型
pub struct Point<T> {
    x: T,
    y: T,
}

#[allow(unused)]
pub fn struct_generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

#[allow(unused)]
impl<T> Point<T> {
    pub fn new(&self) -> Point<&T> {
        Point {
            x: &self.x,
            y: &self.y,
        }
    }

    fn get_x(&self) -> &T {
        &self.x
    }
}

#[allow(unused)]
// 也可以给 Point 一个固定的泛型
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(unused)]
// 也可以是多种泛型
struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    #[allow(unused)]
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(unused)]
fn multi_generics() {
    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}

#[allow(unused)]
// 枚举也可以定义泛型
enum Results<T, E> {
    Ok(T),
    Err(E),
}

// - - - - - - - - - - - - - - - -
struct Duck;
struct Pig;
trait Fly {
    fn fly(&self) -> bool;
}
impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

#[allow(unused)]
fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}

#[allow(unused)]
fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}

#[allow(unused)]
fn fly_fn() {
    let pig = Pig;
    let duck = Duck;

    // 这两种方式叫做静态分发
    // Rust 编译器会为 fly_static::<Pig>(pig) 和 fly_static::<Duck>(duck) 这两个具体类型的调用生成特殊化的代码.
    // 也就是说, 对于编译器来说, 这种抽象并不存在, 因为在编译阶段, 泛型己经被展开为具体类型的代码
    assert_eq!(fly_static::<Pig>(pig), false);
    assert_eq!(fly_static::<Duck>(duck), true);

    // fly_dyn 函数是动态分发方式的
    // 它会在运行时查找相应类型的方法, 会带来一定运行时开销
    assert_eq!(fly_dyn(&Duck), true);
    assert_eq!(fly_dyn(&Pig), false);
}

// 泛型代码的性能
// Rust 通过在编译时进行泛型代码的单态化(Monomorphization)来保证效率.
// 单态化是一个通过填充编译时使用的具体类型, 将通用代码转换为特定代码的过程.
// 举个例子, 下面的代码

#[allow(unused)]
fn foo<T>(x: T) -> T {
    x
}
#[allow(unused)]
fn process_foo() {
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");
}
// 会被编译成类似于如下的形式, 好处是性能好, 没有运行时开销; 缺点是会让生成的二进制变大
#[allow(unused)]
fn foo1(x: i32) -> i32 {
    x
}
#[allow(unused)]
fn foo2(x: &'static str) -> &'static str {
    x
}
#[allow(unused)]
fn process_foo_1() {
    assert_eq!(foo1(1), 1);
    assert_eq!(foo2("hello"), "hello");
}
