#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
