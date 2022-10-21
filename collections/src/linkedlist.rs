use std::collections::LinkedList;

// rust 的链表是双向链表, 允许在任意一端插入或弹出元素
pub fn learn_linklist() {
    let mut buf = LinkedList::new();
    buf.push_front(1);
    buf.push_front(2);
    buf.push_back(3);
    buf.push_back(4);

    println!("buf {:?}", buf); // 2 -> 1 -> 3 -> 4

    let mut buf_2 = LinkedList::new();
    buf_2.push_front(5);
    buf.append(&mut buf_2);
    println!("buf {:?}", buf); // 2 -> 1 -> 3 -> 4 -> 5

    buf.pop_back();
    buf.pop_front();
    println!("buf {:?}", buf); // 1 -> 3 -> 4
}
