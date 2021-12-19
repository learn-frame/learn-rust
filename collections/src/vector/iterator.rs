pub fn learn_iterator_of_vector() {
    let v = vec![1, 2, 3, 4];
    for i in &v {
        println!("{}", i);
    }

    let mut count = 0;
    let mut v_1 = vec![1, 2, 3, 4];
    for i in &mut v_1 {
        // 为了修改可变引用所指向的值, 必须使用解引用
        count += *i;
        println!("{}", count);

        return *i += 50;
    }
}
