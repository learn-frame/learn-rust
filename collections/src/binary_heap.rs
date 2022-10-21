use std::collections::BinaryHeap;

// rust 的优先队列基于最大堆
// https://algorithm.yanceyleo.com/data-structure/queue/priority-queue
pub fn learn_binary_heap() {
    let mut heap = BinaryHeap::new();
    let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];
    for &i in arr.iter() {
        heap.push(i);
    }

    println!("heap: {:?}", heap);

    heap.pop();
    println!("{:?}", heap);
}
