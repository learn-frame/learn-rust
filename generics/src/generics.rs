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

// 泛型代码的性能
// Rust 通过在编译时进行泛型代码的单态化(monomorphization)来保证效率.
// 单态化是一个通过填充编译时使用的具体类型, 将通用代码转换为特定代码的过程.

// let integer = Some(5);
// let integer = Option_i32::Some(5);
