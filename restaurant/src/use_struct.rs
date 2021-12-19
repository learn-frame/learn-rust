pub mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String, // 公有

        #[allow(unused)]
        seasonal_fruit: String, // 私有
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        #[allow(unused)]
        fn winter() {
            println!("winter");
        }
    }
}

// 上面这个例子, 你不能直接创建 Breakfast 的实例
// 因为 seasonal_fruit 是私有的, 你无法初始化这个值, 故不能创建实例
// summer 方法帮你初始化了 seasonal_fruit, 这样你自己初始化个 toast 就行了
