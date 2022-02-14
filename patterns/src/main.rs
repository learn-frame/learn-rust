//! 模式由如下一些内容组合而成
//! 字面值
//! 解构的数组, 枚举, 结构体或者元组
//! 变量
//! 通配符
//! 占位符
//! match 表达式必须是穷尽(exhaustive)的, 意为 match 表达式所有可能的值都必须被考虑到
//! 当然你可以使用 _ 来匹配所有情况

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
enum CarBrands {
    BENZ,
    BMW,
    HONDA,
    TOYOTA,
}

fn learn_let_match(brand: CarBrands) {
    if let brand = CarBrands::BENZ {
        println!("is_benz");
    } else if let brand = CarBrands::BMW {
        println!("is_bmw");
    } else if brand == CarBrands::HONDA {
        println!("is_honda");
    } else {
        println!("is_toyota");
    }
}
