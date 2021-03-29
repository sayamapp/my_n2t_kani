mod parser;
mod commands;
use std::env;
use crate::parser::Commands;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let parser = Lines::new(args[1].to_string());
    // println!("{:?}", parser);
    let args: Vec<String> = env::args().collect();
    let commands = Commands::new(args[1].to_string());
    println!("{:?}", commands);
}