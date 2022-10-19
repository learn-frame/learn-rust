/// 方法语法
/// 其实就可以理解为类嘛

#[derive(Debug, PartialEq)] // 自动让结构体实现 trait, 比如引入 Debug 和 PartialEq 可以使用 assert_eq!
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[allow(unused)]
fn bar() {
    assert_eq!(
        Rectangle {
            width: 1,
            height: 2
        },
        Rectangle {
            width: 1,
            height: 2
        }
    );
}

// 所有在 impl 块中定义的函数被称为关联函数(associated functions)
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可以定义一个跟属性同名的方法
    #[allow(unused)]
    fn width(&self) -> bool {
        self.width > 0
    }

    #[allow(unused)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    #[allow(unused)]
    fn new(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 可以定义多个同名的 impl
impl Rectangle {
    #[allow(unused)]
    fn same_name(&self) {}
}

pub fn foo() {
    let square = Rectangle::new(5);
    println!("{}", square.area());
}
