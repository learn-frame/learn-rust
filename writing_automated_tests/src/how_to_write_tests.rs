/// 1. 设置任何所需的数据或状态
/// 2. 运行需要测试的代码
/// 3. 断言其结果是我们所期望的
///
/// 复习: 属性(attribute) 是关于 Rust 代码片段的元数据, 常见的如
/// #[derive], #[test], #[allow]
///

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(unused)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(unused)]
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(unused)]
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[allow(unused)]
pub fn make_error() {
    panic!("I'm error!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
        assert_ne!(add_two(2), 5);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // 自定义错误信息
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // #[should_panic] 用来验证那些应该出错的函数
    #[test]
    #[should_panic(expected = "出错就对咯!")]
    fn need_error() {
        make_error();
    }

    // 下面这个函数因为不出错, 所以测试会失败
    #[test]
    #[should_panic]
    fn need_error_1() {
        add_two(3);
    }

    // 但吊诡的是, 如果你执行多个函数
    // 只要有一个出错, 就能通过
    // 因此一个 should_panic 宏执行一个函数比较好
    #[test]
    #[should_panic]
    fn need_error_2() {
        add_two(3);
        make_error();
    }

    // 也可以是用 Result<T, E>
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解
    #[test]
    fn use_result() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// 执行 cargo test
//
// test how_to_write_tests::tests::it_works ... ok
// running 1 test
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
