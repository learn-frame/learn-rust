use std::borrow::Cow;

/// 写时复制( Copy on Write)技术是一种程序中的优化策略
/// Cow<T> 以不可变的方式访问借用内容, 以及在需要可变借用或所有权的时候再克隆一份数据
/// Cow<T>旨在减少复制操作, 提高性能, 一般用于读多写少的场景
///
/// 比如 Linux 中父进程创建子进程时, 并不是立刻让子进程复制一份进程空间,
/// 而是先让子进程共享父进程的进程空间, 只有等到子进程真正需要写入的时候才复制进程空间
///
/// 使用 Cow 使我们在返回数据时提供了两种可能: 要么返回一个借用的数据(只读), 要么返回一个拥有所有权的数据(可读写).
///
/// ```
/// #[stable(feature = "rust1", since = "1.0.0")]
/// #[cfg_attr(not(test), rustc_diagnostic_item = "Cow")]
/// pub enum Cow<'a, B: ?Sized + 'a>
/// where
///     B: ToOwned,
/// {
///     /// Borrowed data.
///     #[stable(feature = "rust1", since = "1.0.0")]
///     Borrowed(#[stable(feature = "rust1", since = "1.0.0")] &'a B),
///
///     /// Owned data.
///     #[stable(feature = "rust1", since = "1.0.0")]
///     Owned(#[stable(feature = "rust1", since = "1.0.0")] <B as ToOwned>::Owned),
/// }
/// ```
///
pub fn entry() {
    let s1 = String::from("hellobadhello");
    let s2 = String::from("hellogoodhello");
    remove_sensitive_word_old(&s1);
    remove_sensitive_word_old(&s2);
    remove_sensitive_word_new(&s1);
    remove_sensitive_word_new(&s2);
    learn_to_mut();
    use_insert_prefix_cow();
}

// 当 words 包含 "bad" 时, 我想把 "bad" 去掉, 变成 new_words, 然后返回
// 由于 new_words 是个 String 类型, 并且它是函数的内部变量, 随函数执行完而析构
// 所以对这一部分, 只能返回 String 类型, 不能返回 new_words 的引用, 进而导致函数整体只能返回 String 类型
// 这就导致如果 words 不包含 "bad", 也要 to_string() 做一次复制, 变成 String 类型
//
// 当然你说传入的 words 是 String 类型不就得了
// 老子偏不
// 因为传入 String 类型的变量, 所有权就被 move 到函数里了, 我不想这么做.
fn remove_sensitive_word_old(words: &str) -> String {
    if words.contains("bad") {
        let new_words = words.replace("bad", "");
        return new_words;
    }

    words.to_string()
}

fn remove_sensitive_word_new(words: &str) -> Cow<str> {
    if words.contains("bad") {
        let new_words = words.replace("bad", "");
        return Cow::Owned(new_words);
    }

    // 不需要做一次拷贝
    Cow::Borrowed(words)
}

struct LazyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> LazyBuffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: Cow::Borrowed(data),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn append(&mut self, data: &[u8]) {
        self.data.to_mut().extend(data);
    }
}

fn learn_to_mut() {
    let data = [0u8; 10];

    // No memory copied yet
    let mut buffer = LazyBuffer::new(&data);
    println!("{:?}", buffer.data());

    // The data is cloned
    buffer.append(&[1, 2, 3]);
    println!("{:?}", buffer.data());

    // The data is not cloned on further attempts
    buffer.append(&[4, 5, 6]);
    println!("{:?}", buffer.data());
}

fn insert_prefix_cow<'a, T>(strs: T, prefix: &'a str) -> Vec<Cow<'a, String>>
where
    T: IntoIterator<Item = &'a String>,
{
    strs.into_iter()
        .filter_map(|s| match s.starts_with(prefix) {
            true => Some(Cow::Borrowed(s)),
            false => Some(Cow::Owned(
                String::with_capacity(prefix.len() + s.len()) + prefix + s,
            )),
        })
        .collect::<Vec<Cow<'a, String>>>()
}

fn use_insert_prefix_cow() {
    let strs = vec!["row_rust".to_string(), "rust".to_string()];
    let p = "row_";
    let fixed = insert_prefix_cow(&strs, &p);

    let s0 = &strs[0]; // 第一个元素已经有指定前缀名了
    let f0 = &*fixed[0]; // Cow 实现了 Deref, 所以可以直接解引用

    println!("source addr: {:?}", s0.as_ptr()); // 0x7ff266004280
    println!("cow addr: {:?}", f0.as_ptr()); //    0x7ff266004280, 地址相同

    let s1 = &strs[1]; // 第二个元素插入了前缀名
    let f1 = &*fixed[1];

    println!("source addr: {:?}", s1 as *const String); // 0x7fd0b21040f8
    println!("cow addr: {:?}", f1 as *const String); //    0x7fd0b2104168, 地址已经发生了变化
}

// https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55
// https://mp.weixin.qq.com/s/KwHNzZoSwdTzxqRpwsv1PQ
// https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247485082&idx=1&sn=578711e71b2f2c7789558bae60ce2ca9&chksm=cfc2abf9f8b522efa59d52292587ca90c82aacc9f7308e468c47645f65cb6d0d8c9160c7dd3a&cur_album_id=2361592668113420289&scene=189#wechat_redirect
