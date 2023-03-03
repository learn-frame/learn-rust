//! 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait
//! Drop trait 要求实现一个叫做 drop 的方法, 它获取一个 self 的可变引用
//! RAII (Resource Acquisition Is Initialization)
//! 在现代 C++ 中, RAII 的机制是使用构造函数来初始化资源, 使用析构函数来回收资源.
//! 看上去 RAII 所要做的事确实跟 GC 差不多. 但 RAII 和 GC 最大的不同在于, RAII 将资源托管给创建堆内存的指针对象本身来管理, 并保证资源在其生命周期内始终有效
//! 一旦生命周期终止, 资源马上会被回收. 而 GC 是由第三方只针对内存来统一回收垃圾的, 这样就很被动
//! 而 Drop 除了释放内存, Dro p还可以做很多其他的工作, 比如释放文件和网络连接

pub fn entry() {
    custom_drop();
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 实例离开作用域 Rust 会自动调用 drop, 并调用我们指定的代码.
// 变量以被创建时相反的顺序被丢弃, 所以 d 在 c 之前被丢弃
fn custom_drop() {
    #[allow(unused)]
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // Dropping CustomSmartPointer with data `my stuff`!

    // drop 是默认在离开作用域之前调用的
    // 如果你调用 CustomSmartPointer 中的 drop
    // 会导致在离开函数时再次被 drop, 导致 double free 而出错
    // 因此不允许你这么使用
    // c.drop();

    // 但你可用标准库提供的 std::mem::drop 来提前清理
    drop(c);
    // 此时你就无法得到 c 了, 因为它已经被提前 drop 了
    // println!("{:?}", c);

    #[allow(unused)]
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // Dropping CustomSmartPointer with data `other stuff`!

    println!("CustomSmartPointers created.");
}
