use std::env;

fn main() {
    println!("{:?}", env::args());
    println!("{:?}", env::args_os());
    println!("{:?}", env::current_dir());
    println!("{:?}", env::current_exe());
}
