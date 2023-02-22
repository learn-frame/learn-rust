pub fn entry() {
    learn_char();
    kind_of_string();
    learn_string_slice();
    learn_string();
    visit_ele_of_string();
    visit_range_of_string();
    modify_string();
    delete_string();
}

pub fn learn_char() {
    println!("{}", '🐶' as u32); // 128054
    println!("U+{:x}", '🐶' as u32); // U+1f436
    println!("{}", '🐶'.escape_unicode()); // \u{1f436}
    println!("{}", char::from(65)); // A, 该方法只能传 u8 类型的数字
    println!("{}", char::from_u32(128054).unwrap()); // 🐶
    println!("{}", char::from_u32(0x1f436).unwrap()); // 🐶
    println!("{}", '🐶'.encode_utf8(&mut [0; 4])); // 🐶
    println!("{}", '🐶'.len_utf8()); // 4
    println!("{}", '🐶'.len_utf16()); // 2

    println!("{}", '1'.is_digit(10)); // true
    println!("{}", '1'.to_digit(10).unwrap()); // 1
    println!("{}", '1'.is_lowercase()); // false
    println!("{}", '1'.is_uppercase()); // false
    println!("{}", '1'.is_whitespace()); // false
    println!("{}", '1'.to_lowercase()); // 1
    println!("{}", '1'.to_uppercase()); // 1
    println!("{}", ' '.is_whitespace()); // true
    println!("{}", ' '.is_alphabetic()); // false
    println!("{}", ' '.is_alphanumeric()); // false
    println!("{}", ' '.is_numeric()); // false
    println!("{}", ' '.is_control()); // false
    println!("{}", '\n'.escape_default()); // \n
}

/// - **str**, 表示固定长度的字符串, 即不可变的 UTF-8 字节序列, 创建后无法再为其追加内容或更改其内容.
/// - **String**, 表示可增长的字符串.
/// - **CStr**, 表示由 C 分配而被 Rust 借用的字符串, 一般用于和 C 语言交互.
/// - **CString**, 表示由 Rust 分配且可以传递给 C 函数使用的 C 字符串 , 同样用于和 C 语言交互.
/// - **OsStr**, 表示和操作系统相关的字符串. 这是为了兼容 Windows 系统.
/// - **OsString**, 表示 OsStr 的可变版本. 与 Rust 字符串可以相互转换.
/// - **Path**, 表示路径, 定义于 std::path 模块中. Path包装了 OsStr.
/// - **PathBuf**. 跟 Path 配对, 是 Path 的可变版本. PathBuf 包装了 OsString.
pub fn kind_of_string() {}

/// &str 可以存在以下三个地方:
/// - 静态存储区, 如字符串字面量, &'static str 类型的字符串
/// - 堆分配, 如果 &str 类型的字符串是通过堆 String 类型的字符串取切片生成的, 则存储在堆上
/// - 栈分配, 比如使用 str::from_utf8 方法, 就可以将栈分配的 [u8; N] 数组转换为一个 &str 字符串
pub fn learn_string_slice() {}

/// &str 是一个引用类型, 而 String 类型的字符串拥有所有权.
/// String 是由标准库提供的可变字符串, 可以在创建后为其追加内容或更改其内容.
/// String 类型本质为一个成员变量是 Vec<u8> 类型的结构体, 所以它是直接将字符内容存放于堆中的.
/// String 类型由三部分组成: 指向堆中字节序列的指针(as_ptr 方法), 记录堆中字节序列的字节长度(len方法)和堆分配的容量(capacity方法〉
pub fn learn_string() {
    let str: &'static str = "hello"; // 字符串字面量
    let str_slice = "world"; // 字符串切片
    let string = String::from(str_slice);

    println!(
        "{:p} {} {}",
        string.as_ptr(),   // 0x7feaf48042d0
        string.len(),      // 22
        string.capacity()  // 22
    );

    println!(
        "{} {} {} {} {} {}",
        String::new(),             // 空字符串, 但并未在堆上开辟空间
        String::with_capacity(20), // 如果预先知道最终要创建的字符串长度, 则用此方法可以降低分配堆空间的频率
        str.to_owned(),            // to_owned 方法利用 &str 切片字节序列生成新的 String 字符串
        str.to_string(),           // to_string 方法是对 String::from 的包装
        str.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        &str_slice[1..2]
    );
}

/// Rust 中的字符串不能使用索引访问其中的字符, 因为字符串是 UTF-8 字节序列, 到底是返回字节还是码点是一个问题
/// 因此 Rust 提供了 string.chars() 和 string.bytes(), 即按字节处理和按字符处理
/// 两者都返回一个可迭代对象, chars 的 next 方法按照按码位进行, bytes 的 next 方法按字节进行迭代
pub fn visit_ele_of_string() {
    let mut chars = "a🐶c".chars();
    println!(
        "{:?} {:?} {:?} {:?}",
        chars.next(), // Some('a')
        chars.next(), // Some('🐶')
        chars.next(), // Some('c')
        chars.next()  // None
    );

    let mut bytes = "a🐶c".bytes();
    assert_eq!(6, bytes.len());
    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        bytes.next(), // Some(97)
        bytes.next(), // Some(240)
        bytes.next(), // Some(159)
        bytes.next(), // Some(144)
        bytes.next(), // Some(182)
        bytes.next(), // Some(99)
        bytes.next()  // None
    );
}

/// Rust 虽然不能按索引访问字符串中的字符,
/// 但可以通过 get / get_mut 获取字符串中的一个范围
pub fn visit_range_of_string() {
    let mut string = String::from("a🐶c");

    println!(
        "{:?} {:?} {} {:?}",
        string.get(0..1),           // Some("a")
        string.get(0..=1), // None, 因为 🐶 占了 4 个字节, 如果只取到索引 1, 会把 🐶 切碎导致出错
        string.is_char_boundary(2), // false, 2 恰好在 🐶 的
        string.get(0..=4), // Some("a🐶")
    );

    assert!(string.get_mut(1..).is_some());
    assert!(string.get_mut(0..=1).is_none());
    assert_eq!(("a", "🐶c"), string.split_at(1));

    // 😈: 报错
    // assert_eq!(("a", "🐶c"), string.split_at(2));
}

// 从追加, 连接, 更新吗
pub fn modify_string() {
    // 追加
    let mut string: String = "hello, world".into();
    string.push('!');
    string.push_str("!!");
    string.extend([' ', '喵', '喵', '喵'].iter());
    string.extend(" 喵喵喵".chars());
    println!("{}", string); // hello, world!!! 喵喵喵 喵喵喵

    // 插入
    let mut string: String = "helloworld".into();
    string.insert_str(5, ", ");
    string.insert(12, '!');
    println!("{}", string); // hello, world!

    // 😈: thread 'main' panicked at 'assertion failed: self.is_char_boundary(idx)'
    // insert 和 insert_str 是按照来的字节来的, 因此也会有 is_char_boundary 的问题, 下面两个就会报错
    // let mut string = "a🐶c".to_string();
    // string.insert(2, '🐱');
    // string.insert_str(2, "🐱");

    // 连接
    let mut string = "hello".to_string();
    string += ", world!";
    println!("{}", string); // hello, world!

    // 更新
    // Rust 不支持按索引修改字符, 比如 JavaScript 中的 splice, 只能靠着迭代
    // 最好用 chars, 因为上面我们看到了按字节处理会有问题.
    // 不过下面还是展示两种用字节处理的
    let mut string_bytes = String::from("hello thank you thank you very much").into_bytes();
    (0..string_bytes.len()).for_each(|i| {
        if string_bytes[i] == b' ' {
            string_bytes[i] = b'-';
        }
    });
    println!("{}", String::from_utf8(string_bytes.to_vec()).unwrap()); // hello-thank-you-thank-you-very-much

    let string_bytes = String::from("hello thank you thank you very much")
        .into_bytes()
        .iter()
        .fold(vec![], |mut vec, byte| {
            if *byte != b' ' {
                vec.push(*byte);
            } else {
                vec.push(b'-');
            }

            vec
        });
    println!("{}", String::from_utf8(string_bytes).unwrap()); // hello-thank-you-thank-you-very-much

    let string_chars = String::from("hello thank you thank you very much")
        .chars()
        .map(|ch| match ch == ' ' {
            true => "-".to_string(),
            _ => ch.to_string(),
        })
        .collect::<String>();
    println!("{}", string_chars); // hello-thank-you-thank-you-very-much
}

// 删除字符串
pub fn delete_string() {
    let mut string = String::from("a🐶c");
    string.remove(1);
    println!("{}", string); // ac
    // 😈: panicked at 'byte index 1000 is out of bounds of `ac`'
    string.remove(1000);
}
