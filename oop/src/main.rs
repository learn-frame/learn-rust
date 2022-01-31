/// 面向对象的程序是由对象组成的. 一个对象包含数据和操作这些数据的过程. 这些过程通常被称为方法或操作.
/// 在 Rust, 结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法. 虽然带有方法的结构体和枚举并不被称为对象, 但是他们提供了与对象相同的功能
/// 另一个通常与面向对象编程相关的方面是封装(encapsulation)的思想: 对象的实现细节不能被使用对象的代码获取到. 所以唯一与对象交互的方式是通过对象提供的公有 API;
/// 使用对象的代码无法深入到对象内部并直接改变数据或者行为. 封装使得改变和重构对象的内部时无需改变使用对象的代码

fn main() {
    println!("Hello, world!");
}

// Rust 可以使用 pub 关键字来决定模块, 类型, 函数和方法是公有的, 而默认情况下其他一切都是私有的
pub mod user_mod {}

// 结构体自身被标记为 pub, 这样其他代码就可以使用这个结构体, 但是在结构体内部的字段仍然是私有的
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
}

pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
        Some(value) => {
            self.update_average();
            Some(value)
        }
        None => None,
    }
}

pub fn average(&self) -> f64 {
    self.average
}

fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
}
