use std::{rc::Rc, sync::Arc};

pub fn entry() {
    lexical_scope();
    borrow();
    lifetimes_params();
    omit_lifetimes_params("Yancey Leo");
    lifetimes_bound("hi", "world", &String::from(""));
    smart_point_with_lifetimes();
}

///  变量绑定具有"时空"双重属性
/// - 空间属性是指标识符与内存空间进行了绑定.
/// - 时间属性是指绑定的时效性, 也就是指它的生存周期.
/// 一个绑定的生存周期也被称为生命周期(lifetime), 是和词法作用域有关的
fn lexical_scope() {
    /* let 称为 let binding, 它创建一个默认的词法作用域, 该作用域就是它的生命周期 */
    let a = 1;
    let b = "hello".to_string();

    /* 花括号也有词法作用域 */
    {
        let inner_a = 1;
        let c = a;
        let d = b;
        println!("{} {} {}", inner_a, c, d);
    }

    // println!("{}", inner_a); // 😈: 外部取不到内部的作用域
    // println!("{}", b); // 😈: b 是引用语义, 被转移到了内部词法作用域, 外部取不到
    println!("{}", a); // 1, a 是复制语义

    /* Option 作用域 */
    let a = Some(String::from(""));
    match a {
        Some(s) => println!("{}", s),
        None => (),
    }
    // 😈: a 是 Some 类型, Some 类型是否为复制语义还是引用语义取决于内部值的类型
    //这段代码 a 被移动到 match 里, 下面代码就取不到 a 了, 除非你使用 match a {}
    // println!("{:?}", a);

    /*
       此外, for, loop 以及 while 这些循环语句; if let 和 while let 块; 函数; 闭包都会产生词法作用域
    */
    let s = "hello".to_string();
    let join = |i: &str| s + i;
    println!("{}", join(" world")); // "hello, world"
                                    //  println!("{}", s); // 😈:
}

/// 借用规则
/// 规则一: 借用的生命周期不能长于出借方(拥有所有权的对象)的生命周期
/// 规则二: 可变借用(引用)不能有别名(Alias), 因为可变借用具有独占性
/// 规则三: 不可变借用(引用)不能再次出借为可变借用
///
/// 规则一主要是为了防止悬垂引用, 因为如果出借方已经被析构了, 但借用依然存在, 那就会出现悬垂指针
/// 规则二和规则三可以统一总结为: **共享不可变, 可变不共享**, 即:
/// 不可变借用可以被出借多次, 因为它不能修改内存数据, 因此它也被称为共享借用(引用). 可变借用只能出借一次, 否则, 难以预料数据何时何地会被修改.
///
/// Rust 编译器有一个借用检查器(borrow checker), 它比较作用域来确保所有的借用都是有效的
fn borrow() {
    // 不允许一个变量即用作可变借用, 也用作不可变借用
    // 😈: cannot borrow `i` as mutable because it is also borrowed as immutable
    // fn add(a: &i32, b: &mut i32) {}
    // let mut i = 20;
    // add(&i, &mut i);

    // 解引用操作会获得所有权
    // 下面这段代码是错的, 因为 *s 得到了 String, 未实现 Copy trait, 不能够被借用
    // fn join(s: &String) -> String {
    //     let append = *s;
    //     "Hello".to_string() + &append
    // }

    // 😈: 如下这个写法是错误的, 因为返回值是 s 的引用
    // 但 s 在函数之后就被销毁, 所以外面是不可能拿到的
    // pub fn say_hi() -> &i32 {
    //     let s = 1;

    //     &s
    // }
    // ^ `x` dropped here while still borrowed

    // 😈: r 一开始的生命周期是 'a 这个区域, 但因为后面引用了 x,
    // 所以生命周期缩到 'b 这个区域, 因此最后打印 r 时,
    // 超出了 'b 这个区域, 故报错
    // 即借用的生命周期不能长于出借方的生命周期
    // pub fn say_hello() {
    //     {
    //         let r;                // ---------+-- 'a
    //                               //          |
    //         {                     //          |
    //             let x = 5;        // -+-- 'b  |
    //             r = &x;           //  |       |
    //         }                     // -+       |
    //                               //          |
    //         println!("r: {}", r); //          |
    //     }                         // ---------+
    // }
}

/// 生命周期参数
///
/// 上面说到对于函数本地声明的拥有所有权的值或者借用, Rust 编译器包含的借用检查器(borrow checker)可以检查它们的生命周期
/// 但是对于跨词法作用域的借用, 借用检查器就无法自动推断借用的合法性了
/// 这种情况下, 你需要主动标记生命周期参数, 来辅助 Rust 进行借用检查
/// 标注生命周期参数并不能改变任何引用的生命周期长短, 它只用于编译器的借用检查, 来防止悬垂指针.
fn lifetimes_params() {
    /// 函数的生命周期
    ///
    /// 函数名后面的 <'a> 为生命周期参数的声明, 与泛型参数类似, 必须先声明才能使用.
    /// 函数或方法参数的生命周期叫作输入生命周期(input lifetime), 而返回值的生命周期被称为输出生命周期(output lifetime).
    ///
    /// 😈: 下面这个例子, 返回的是 x 或者 y 的引用, 但 Rust 会默认 x 和 y 的生命中周期是不同的, 就会出错
    /// ```
    /// fn longest(x: &str, y: &str) -> &str {
    ///     if x.len() > y.len() {
    ///         x
    ///     } else {
    ///         y
    ///     }
    /// }
    /// ```
    ///
    /// 所以你要显式标明他们的生命周期是相同.
    /// 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
    /// 这就是我们告诉 Rust 需要其保证的约束条件
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() {
            x
        } else {
            y
        }
    }
    /// 如果你想为每个参数都标注不同的生命周期也可以
    /// 因为这个函数要求 x 和 y 的生命周期相同, 你可以让 'b: 'a
    /// 意思是泛型生命周期参数 'b 的存活时间长于泛型生命周期参数 'a(即 'b outlive 'a)
    fn the_longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() >= y.len() {
            x
        } else {
            y
        }
    }
    assert_eq!("a", longest("a", "b"));
    assert_eq!("a", the_longest("a", "b"));

    /// 😈: 禁止在没有任何输入参数的情况下返回引用
    /// 下面的错误也是显而易见的, s 是函数内部的, 在函数执行完就被析构了, 所以外面拿不到 s 的引用
    /// ```
    /// fn return_str<'a>() -> &'a str {
    ///     let mut s = "Rust".to_string();
    ///
    ///     for _ in 0..3 {
    ///         s.push_str(" Good");
    ///     }
    ///
    ///     &s[..]
    /// }
    /// ```
    ///
    /// 换成返回 String 类型就好了
    fn return_str() -> String {
        let mut s = "Rust".to_string();

        for _ in 0..3 {
            s.push_str(" Good");
        }

        s
    }
    assert_eq!(String::from("Rust Good Good Good"), return_str());

    /// 结构体的生命周期
    ///
    /// 一个常见的例子, 如果结构体中的成员用到了 &str 类型, 就必须得使用 'a
    /// 这里的生命周期参数标记, 实际上是和编译器约定了一个规则: 结构体实例的生命周期应短于或等于任意一个成员的生命周期.
    #[allow(unused)]
    struct ImportantExcerpt<'a, T> {
        part: &'a str,
        age: T,
    }

    #[allow(unused)]
    fn exec_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let instance = ImportantExcerpt {
            part: first_sentence,
            age: 1,
        };
        println!("{}", instance.part);
    }

    /// 方法定义中的生命周期
    ///
    /// 因为 struct ImportantExcerpt 用到了生命周期参数, 所以在 impl 也必须标注生命周期参数
    impl<'a, T> ImportantExcerpt<'a, T> {
        #[allow(unused)]
        fn level(&self) -> i32 {
            3
        }

        #[allow(unused)]
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    ///
    /// trait 对象和生命周期有默认遵循的规则:
    /// - trait 对象的生命周期默认是 'static.
    /// - 如果实现 trait 的类型包含 &'a X 或 &'a mut X, 则默认生命周期就是 'a.
    /// - 如果实现 trait 的类型只有 T:'a, 则默认生命周期就是 'a.
    /// - 如果实现 trait 的类型包含多个类似 T:'a 的从 句, 则生命周期需要明确指定
    ///
    /// 下面这个就不用

    trait Foo {}
    #[allow(unused)]
    struct Bar<'a> {
        x: &'a i32,
    }
    impl<'a> Foo for Bar<'a> {}
    fn trait_object_unneed_lifetimes() {
        let num = 5;
        let box_bar = Box::new(Bar { x: &num });
        #[allow(unused)]
        let obj = box_bar as Box<dyn Foo>;
    }
    trait_object_unneed_lifetimes();
    ///
    /// 下面这个必须
    /// Box<Foo<'a>> 是一个 trait 对象, 它的默认生命周期是 'static 的.
    /// 而现在实现 trait Trait 的类型 Struct 有一个 &'a [u32] 类型的成员, 所以此时的 trait 对象生命周期应该是 'a.
    /// 因此, 如果想修复上面的错误, 只需要显式地为 trait 对象增加生命周期参数,
    /// 将 Box<dyn Trait<'a>> 改为 Box<dyn Trait<'a> + 'a> 即可, 此时该 trait 对象的生命周期就是 'a, 覆盖了默认的 'static 生命周期
    trait Trait<'a> {}
    #[allow(unused)]
    struct Struct<'a> {
        s: &'a [u32],
    }
    impl<'a> Trait<'a> for Struct<'a> {}
    fn trait_object_lifetimes<'a>(s: &'a [u32]) -> Box<dyn Trait<'a> + 'a> {
        Box::new(Struct { s })
    }
    trait_object_lifetimes(&[1, 2, 3]);

    /// 静态生命周期参数
    ///
    /// 'static 的生命周期能够存活于整个程序期间. 所有的字符串字面值都拥有 'static 生命周期
    #[allow(unused)]
    fn static_lifetimes_params() {
        let s: &'static str = "I have a static lifetime.";
        let y = s;
        println!("{} {}", s, y);
    }
}

/// 省略生命周期参数
///
/// 对于理论上需要显式地标注生命周期参数的情况, 实际中依然存在可以省略生命周期参数的可能
///
/// 生命周期省略规则(Lifetime Elision Rule)共有三条:
/// 1. 每一个是引用的参数都有它自己的生命周期参数
/// 2. 如果只有一个输入生命周期参数, 那么它被赋予所有输出生命周期参数
/// 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self, 说明是个对象的方法, 那么所有输出生命周期参数被赋予 self 的生命周期
///
/// 下面是可以忽略的一些例子
fn omit_lifetimes_params(s: &str) -> &str {
    &s[..]
}
trait A {
    fn get_mut<T>(&mut self) -> &mut T;
    fn get<T>(&self) -> &T;
}

/// 生命周期限定
///
/// announcement 的类型是泛型 T, 它可以被放入任何实现了 where 从句中指定的 Display trait 的类型
/// 在 Rust 2018 版本中编译器已经可以自己判断了
fn lifetimes_bound<'a, T: 'a>(x: &'a str, y: &'a str, announcement: &'a T) -> &'a T
where
    T: std::fmt::Display + 'a,
{
    println!("Announcement! {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    };

    announcement
}

/// 除了普通的引用(借用)类型, Rust 还提供具有移动语义(引用语义)的智能指针.
/// 智能指针和普通引用的区别之一就是所有权的不同. 智能指针拥有资源的所有权, 而普通引用只是对所有权的借用.
fn smart_point_with_lifetimes() {
    let a = Box::new(1);
    #[allow(unused)]
    let b = a;
    // println!("{}", a); 😈: borrow of moved value: `a`

    // 这种对 Box<T> 使用操作符(*)进行解引用而转移所有权的行为, 被称为解引用移动
    // 之所以 Box 支持解引用移动, 是因为它支持 #[lang = "owned_box"]
    let c = Box::new(1);
    let d = Box::new(String::from("abc"));
    let e = *c;
    let f = *d;

    println!("{}", c); // 1
    println!("{} {}", e, f);

    // 😈: d 被解引用后是一个引用语义, 它被赋值给 f 后所有权也被转移走了, 因此后面就不能再读取 d 了
    // println!("{}", d);

    // Rc<T> 或 Arc<T> 智能指针不支持解引用移动(但解引用复制是可以的)
    #[allow(unused)]
    let h = Rc::new(vec![1, 2, 3]);
    #[allow(unused)]
    let i = Arc::new(vec![1, 2, 3]);
    // let j = *h; // 😈: cannot move out of an `Rc`
    // let k = *i; // 😈: cannot move out of an `Arc`
}
