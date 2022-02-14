pub mod iterator;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    role: Roles,
}

#[derive(Debug)]
enum Roles {
    Admin(String),
    Custom(String),
}

pub fn learn_vector() {
    let user_1: User = User {
        name: String::from("Yancey Leo"),
        age: 25,
        role: Roles::Admin(String::from("")),
    };
    let user_2: User = User {
        name: String::from("Sayaka Yamamoto"),
        age: 28,
        role: Roles::Custom(String::from("")),
    };

    println!("{:?}", user_1);

    // 创建一个空的 vector 需要标明类型
    let vec: Vec<User> = Vec::new();
    println!("{:?}", vec);

    // 通过 vec! 宏可自动推断类型, 但你至少初始化进一个元素
    let vec_1 = vec![user_1, user_2];
    println!("{:?}", vec_1);

    // 😈: 不能通过 vec! 宏创建一个空的 vetcor
    // let vec_1 = vec![];

    // vector 同样遵循所有权, 在所有权之外是拿不到的
    {
        // 如果一个 vector 是 mut 的, 通过 Vec::new() 无需标明类型
        let mut v = Vec::new();

        // 追加元素
        v.push(0);
        v.push(1);
        v.push(2);

        // 读取元素
        // 顺便复习: & 是引用的意思, 使用值但不获取其所有权
        let index_1 = &v[1]; // 2
        println!("index_1: {}", index_1);

        // vector 越界在运行时会报错, 在编译时不会, 这和数组表现一致
        // let index_3 = &v[3];

        // 为了不出问题, 你可以使用 match 大法
        match v.get(3) {
            Some(index_3) => {
                println!("{}", index_3);
            }
            None => {
                println!("数组越界");
            }
        }
    }

    // 如下写法会报错, 非常操蛋
    // 这是因为在 push 的时候, 在没有足够空间将所有所有元素依次相邻存放的情况下
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中
    // 这时第一个元素的引用就指向了被释放的内存
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // 先取, 再插是没事的
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is: {}", first);
    v.push(6);

    iterator::learn_iterator_of_vector();

    let mut v_2 = vec![1, 2, 3, 4, 5];
    let top = v_2.pop();

    match top {
        Some(top) => {
            println!("The top element is: {}", top);
        }
        None => {
            println!("🈳️");
        }
    }

    // 用枚举来储存多种类型
    // Vector 默认只能存储一种类型, 你可以用枚举来存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    #[allow(unused)]
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
