pub trait Draw {
    fn draw(&self);
}

// 这种写法, 只要实现了 Draw trait 的均可放到 components Vector
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 下面这种写法, 需要 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub trait ButtonEvent {
    fn on_click(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制 button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制 select box");
    }
}

pub fn render() {
    let screen = Screen {
        // 这样, 实现了 Draw trait 的组件均可放到 components Vector 中
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // 如果你卷进一个未实现 Draw 的组件就会报错
            // Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}

// 单态化所产生的代码进行静态分发(static dispatch). 静态分发发生于编译器在编译时就知晓调用了什么方法的时候.
// 这与动态分发(dynamic dispatch)相对, 这时编译器在编译时无法知晓调用了什么方法. 在动态分发的情况下, 编译器会生成在运行时确定调用了什么方法的代码.
// 动态分发有一定的灵活性, 但阻碍了优化
