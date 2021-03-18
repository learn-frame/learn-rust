extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = "垃圾螃蟹!".as_bytes();
    let width = 12;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
