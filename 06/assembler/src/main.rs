mod parser;
mod commands;
use std::env;
use parser::Lines;


fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = Lines::new(&args[1].to_string());
}