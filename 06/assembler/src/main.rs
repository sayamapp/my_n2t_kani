mod parser;
mod code;

use std::env;
use crate::parser::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = Lines::new(&args[1].to_string());
    let bins = lines.to_binary();
    for bin in bins {
        println!("{}", bin);
    }
}