pub fn entry() {
    let num: i8 = 5;
    plus_one(&num);
}

pub fn plus_one(num: &i8) -> i8 {
    num + 1
}
