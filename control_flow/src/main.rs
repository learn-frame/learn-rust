use std::cmp::Ordering;

fn main() {
    let number = 3;

    // 草泥马为什么没有括号!!!
    // 另外, 条件必须是 boolean 类型, 不能像 JavaScript 一样随便什么类型就行
    if number < 5 {
        println!("condition was true");
    } else if number > 5 {
        println!("condition was false");
    } else {
        println!("bingo")
    }

    // TODO: 过多的 if / else 会很蛋疼, 后面会讲到 match
    match number.cmp(&3) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    };

    // 使用 if 表达式, 简单说就是把 if/else 的返回值赋值给一个变量
    let condition = true;
    // 5
    let real_num = if condition { 5 } else { 6 };
    println!("{}", real_num);
    // 😈 返回值必须是相同的类型, 否则报错
    // let incompatible_type = if condition { 5 } else { "six" };

    // 循环
    let mut num = 0;
    loop {
        num += 1;

        if num < 10 {
            println!("again!");
        } else {
            break;
        }
    }

    // 嵌套循环
    let mut count = 0;
    // 给外层循环一个标签, 以便内部循环使用
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            // 停掉内部循环
            if remaining == 9 {
                break;
            }

            // 停掉外部循环
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count); // 2

    // 从循环中返回
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); // 20

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }

    foo();
}

// 私以为大多数循环还是 for 循环吧...
fn foo() {
    let arr = [1, 2, 3, 4, 5];
    for val in arr.iter() {
        if val % 2 == 0 {
            println!("{}", val);
        }
    }

    for val in arr.iter() {
        println!("{}", val)
    }

    // 引子: 迭代器
    let mut arr_iter = arr.iter();
    arr_iter.next();
    arr_iter.next();
    arr_iter.next();
    arr_iter.next();
    arr_iter.next();
    arr_iter.next(); // 不会报错

    // 引子: rev() 用来反转可迭代对象
    for number in (1..=4).rev() {
        println!("{}", number);
    }

    // rust 创建范围数组
    // 想起了 Vue QAQ.
    // <div>
    //  <span v-for="n in 10">{{ n }} </span>
    // </div>
    let arr_1 = 1..=4; // [1, 2, 3, 4]
    let arr_2 = 1..4; // [1, 2, 3]
}
