use std::collections::VecDeque;

// 双端队列(Double-ended Queue, 缩写为 Deque), 是一种同时具有队列(先进先出)和栈(后进先出)性质的数据结构.
pub fn learn_deque() {
    let mut buf = VecDeque::new();
    // push_front 的行为像栈
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    // push_back 的行为像队列
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}
