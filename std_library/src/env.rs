use std::env;

pub fn entry() {
    println!("{:?}", env::args());
    println!("{:?}", env::args_os());
    println!("{:?}", env::current_dir());
    println!("{:?}", env::current_exe());
}
