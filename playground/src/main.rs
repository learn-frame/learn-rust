use std::ops::Drop;

fn main() {
    varible_shadowing_drop();
}

#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop for {}", self.0);
    }
}

fn varible_shadowing_drop() {
    let x = S(1);
    println!("create x: {:?}", x);
    let x = S(2);
    println!("create shadowing x: {:?}", x);
}
