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

// 结构体可以定义泛型
pub struct Point<T> {
    x: T,
    y: T,
}

pub fn struct_generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

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

// 也可以给 Point 一个固定的泛型
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point_1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point_1<T, U> {
    fn mixup<V, W>(self, other: Point_1<V, W>) -> Point_1<T, W> {
        Point_1 {
            x: self.x,
            y: other.y,
        }
    }
}

// 枚举也可以定义泛型
enum Results<T, E> {
    Ok(T),
    Err(E),
}
