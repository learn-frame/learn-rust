use std::collections::{BTreeSet, HashSet};

pub fn learn_hashset() {
    let mut h = HashSet::new();
    let mut b = BTreeSet::new();

    h.insert("Yancey");
    h.insert("Sayaka");
    h.insert("Toshi");

    b.insert("Yancey");
    b.insert("Sayaka");
    b.insert("Toshi");

    println!("HashSet: {:?}", h); // {"Toshi", "Sayaka", "Yancey"} 无序的
    println!("BTreeSet: {:?}", b); // {"Sayaka", "Toshi", "Yancey"} 有序的
}
