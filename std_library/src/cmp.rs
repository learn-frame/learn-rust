use std::cmp::*;

pub fn entry() {
    ordering(1, 2);
    compare();
}

fn ordering(a: i32, b: i32) {
    match a.cmp(&b) {
        Ordering::Equal => println!("a is equal than b"),
        Ordering::Greater => println!("a is greater than b"),
        Ordering::Less => println!("a is less than b"),
    }
}

fn compare() {
    assert_eq!(max(1, 2), 2);
    assert_eq!(min(1, 2), 1);
    assert_eq!(min("z", "abc"), "abc");
    assert_eq!(max_by(-2, 1, |x: &i32, y: &i32| x.abs().cmp(&y.abs())), -2);
    assert_eq!(min_by(-2, 1, |x: &i32, y: &i32| x.abs().cmp(&y.abs())), 1);
    assert_eq!(max_by_key(-2, 1, |x: &i32| x.abs()), -2);
    assert_eq!(min_by_key(-2, 1, |x: &i32| x.abs()), 1);
}
