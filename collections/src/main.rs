pub mod deque;
pub mod hashmap_btreemap;
pub mod hashset_btreeset;
pub mod linkedlist;
pub mod string;
pub mod vector;
pub mod binary_heap;

// 在 Rust 标准库 std::collections 模块下有 4 种 通用集合类型, 分别如下:
// 线性序列: 向量(Vec), 双端队列(VecDeque), 链表(LinkedList).
// Key-Value 映射表: 无序哈希表(HashMap), 有序哈希表(BTreeMap).
// 集合类型: 无序集合(HashSet), 有序集合(BTreeSet).
// 优先队列: 二叉堆(BinaryHeap).

fn main() {
    vector::learn_vector();
    deque::learn_deque();
    linkedlist::learn_linklist();

    hashmap_btreemap::learn_hashmap();
    hashmap_btreemap::get_lru();
    hashset_btreeset::learn_hashset();

    string::learn_string();
    string::update_string();
    string::index_string();
    string::slice_string();
    string::traverse_string();

    binary_heap::learn_binary_heap();
}
