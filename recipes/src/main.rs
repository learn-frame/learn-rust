fn main() {
    #[allow(unused)]
    let arr_1 = [1, 2, 3];
    #[allow(unused)]
    let arr_2 = vec![1, 2, 3];
    #[allow(unused)]
    let arr_3 = [3; 5]; // [3, 3, 3, 3, 3]
    #[allow(unused)]
    let arr_4 = [[""; 2]; 2]; // [["", ""], ["", ""]];
    #[allow(unused)]
    let arr_5 = 1..=4; // [1, 2, 3, 4]
    #[allow(unused)]
    let arr_6 = 1..4; // [1, 2, 3]
}
