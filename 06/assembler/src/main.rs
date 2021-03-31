mod parser;
mod code;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::parser::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = Lines::new(&args[1].to_string());
    let bins = lines.to_binary();

    let mut output = String::new();
    for bin in bins {
        output = output + &bin + &"\n".to_string();
    }

    let mut file = BufWriter::new(File::create("test.hack").unwrap());
    write!(file, "{}", output).unwrap();
    file.flush().unwrap();

}