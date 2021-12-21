use std::collections::HashMap;
use uuid::Uuid;

pub enum Gender {
    Man,
    Woman,
}

pub struct User {
    #[allow(unused)]
    name: String,
    #[allow(unused)]
    age: i8,
    #[allow(unused)]
    gender: Gender,
    #[allow(unused)]
    address: String,
}

// 和 vector 一样, 哈希 map 将它们的数据储存在堆上
pub fn learn_hashmap() {
    create_hashmap_by_class();
    create_hashmap_by_vector();
    ownership_of_hashmap();
    visit_hashmap();
    update_hashmap();
    traverse_hashmap();
}

pub fn create_hashmap_by_class() {
    let mut users: HashMap<String, User> = HashMap::new();

    let user1_id = Uuid::new_v4().to_hyphenated().to_string();
    let user1_msg = User {
        name: String::from("Yancey Leo"),
        age: 18,
        gender: Gender::Man,
        address: String::new(),
    };

    let user2_id = Uuid::new_v4().to_hyphenated().to_string();
    let user2_msg = User {
        name: String::from("Sayaka Yamamoto"),
        age: 18,
        gender: Gender::Woman,
        address: String::new(),
    };

    users.insert(user1_id, user1_msg);
    users.insert(user2_id, user2_msg);
}

pub fn create_hashmap_by_vector() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

pub fn ownership_of_hashmap() {
    let field1_name = 1;
    let field1_value = String::from("Blue");

    let field2_name = 1;
    let field2_value = String::from("Blue");

    let mut map1 = HashMap::new();
    // 如果 value 设成 field1_value, 它的所有权就被 map "吸收"了
    map1.insert(field1_name, field1_value);
    // 😈 这里是拿不到 field1_value 的
    // println!("{}", field1_value);

    // 如果将值的引用插入哈希 map, 这些值本身将不会被移动进哈希 map.
    // 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的
    // TODO: 第十章会讲到
    let mut map2 = HashMap::new();
    map2.insert(field2_name, &field2_value);
    println!("{}", field2_value);
}

pub fn visit_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // 访问 hashmap 某个 key, 得到的是一个 Option 类型
    match score {
        Some(score) => {
            println!("{}", score);
        }

        None => {
            println!("no this value");
        }
    }
}

// 尽管键值对的数量是可以增长的, 不过任何时候, 每个键只能关联一个值.
// 当我们想要改变哈希 map 中的数据时, 必须决定如何处理一个键已经有值了的情况.
// 1. 可以选择完全无视旧值并用新值代替旧值.
// 2. 可以选择保留旧值而忽略新值, 并只在键没有对应值时增加新值.
// 3. 或者可以结合新旧两值
pub fn update_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 只在键没有对应值时增加新值
    scores.entry(String::from("Yellow")).or_insert(50); // 新增 {Yellew: 50}
    scores.entry(String::from("Blue")).or_insert(50); // 无效!

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // 如果没有 key 就新建 key, value 默认设为 0
    // 如果有 key 就让它的 value + 1
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // 如果没有 key 就新建 key, value 默认设为 42
    // 如果有 key 就让它的 value + 1
    scores
        .entry(String::from("Yellow"))
        .and_modify(|e| *e += 1)
        .or_insert(42);
}

pub fn traverse_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{} -> {}", k, v);
    }
}

// HashMap 默认使用一种 "密码学安全的(cryptographically strong)"哈希函数，它可以抵抗 Dos 攻击
// 然而这并不是可用的最快的算法, 如果有性能问题, 就换呗...
