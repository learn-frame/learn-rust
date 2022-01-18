/// 方法语法
/// 其实就可以理解为类嘛

#[derive(Debug)]
pub(crate) struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

// 所有在 impl 块中定义的函数被称为关联函数(associated functions)
impl Rectangle {
    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可以定义一个跟属性同名的方法
    #[allow(unused)]
    pub(crate) fn width(&self) -> bool {
        self.width > 0
    }

    #[allow(unused)]
    pub(crate) fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    #[allow(unused)]
    pub(crate) fn square(size: u32) -> Rectangle {
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
    let square = Rectangle::square(5);
    println!("{}", square.area());
}
