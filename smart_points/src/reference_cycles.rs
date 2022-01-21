use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use List::{Cons, Nil};

pub fn entry() {
    // use_list();
    // use_weak();
    full_example();
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // tail 方法来方便我们在有 Cons 成员的时候访问其第二项
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

/// 这里在变量 a 中创建了一个 Rc<List> 实例来存放初值为 5, Nil 的 List 值. 接着在变量 b 中创建了存放包含值 10 和指向列表 a 的 List 的另一个 Rc<List> 实例.
/// 最后, 修改 a 使其指向 b 而不是 Nil, 这就创建了一个循环. 为此需要使用 tail 方法获取 a 中 RefCell<Rc<List>> 的引用,
/// 并放入变量 link 中. 接着使用 RefCell<Rc<List>> 的 borrow_mut 方法将其值从存放 Nil 的 Rc<List> 修改为 b 中的 Rc<List>.
pub fn use_list() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a)); // a initial rc count = 1
    println!("a next item = {:?}", a.tail()); // a next item = Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // a rc count after b creation = 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // b initial rc count = 1
    println!("b next item = {:?}", b.tail()); // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // 创建一个引用循环, a.tail() 本来是 Nil, 但让它指向 b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // a 5 -> Nil
    // b 10 -> 5 -> Nil

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // b rc count after changing a = 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // a rc count after changing a = 2

    // 溢出
    // println!("a next item = {:?}", a.tail());
}

// 避免引用循环: 将 Rc<T> 变为 Weak<T>
#[derive(Debug)]
struct Node {
    value: i32,
    // 一个节点就能够引用其父节点, 但不拥有其父节点
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn use_weak() {
    let leaf = Rc::new(Node {
        value: 3,
        // leaf 开始时没有父节点, 所以我们新建了一个空的 Weak 引用实例.
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 尝试使用 upgrade 方法获取 leaf 的父节点引用时, 会得到一个 None 值
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}

pub fn full_example() {
    // 一旦创建了 leaf, 其 Rc<Node> 的强引用计数为 1, 弱引用计数为 0. 在内部作用域中创建了 branch 并与 leaf 相关联,
    // 此时 branch 中 Rc<Node> 的强引用计数为 1, 弱引用计数为 1(因为 leaf.parent 通过 Weak<Node> 指向 branch).
    // 这里 leaf 的强引用计数为 2, 因为现在 branch 的 branch.children 中储存了 leaf 的 Rc<Node> 的拷贝, 不过弱引用计数仍然为 0.
    // 一旦创建了 leaf, 其 Rc<Node> 的强引用计数为 1, 弱引用计数为 0. 在内部作用域中创建了 branch 并与 leaf 相关联,
    // 此时 branch 中 Rc<Node> 的强引用计数为 1, 弱引用计数为 1(因为 leaf.parent 通过 Weak<Node> 指向 branch).
    // 这里 leaf 的强引用计数为 2, 因为现在 branch 的 branch.children 中储存了 leaf 的 Rc<Node> 的拷贝, 不过弱引用计数仍然为 0.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0


    // 当内部作用域结束时, branch 离开作用域, Rc<Node> 的强引用计数减少为 0, 
    // 所以其 Node 被丢弃. 来自 leaf.parent 的弱引用计数 1 与 Node 是否被丢弃无关, 所以并没有产生任何内存泄漏!
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        ); // branch strong = 1, weak = 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        ); // leaf strong = 2, weak = 0
    }

    // 如果在内部作用域结束后尝试访问 leaf 的父节点, 会再次得到 None. 在程序的结尾, 
    // leaf 中 Rc<Node> 的强引用计数为 1, 弱引用计数为 0, 因为现在 leaf 又是 Rc<Node> 唯一的引用了.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None

    // 所有这些管理计数和值的逻辑都内建于 Rc<T> 和 Weak<T> 以及它们的 Drop trait 实现中. 
    // 通过在 Node 定义中指定从子节点到父节点的关系为一个Weak<T>引用, 就能够拥有父节点和子节点之间的双向引用而不会造成引用循环和内存泄漏. 
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0
}
