mod parser;
mod code;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::parser::Lines;

fn main() {
    let input_file_pass = "../asm_hack/";
    let output_file_pass = "../asm_hack/";
    let input_file_name = vec!["Add.asm", "MaxL.asm", "RectL.asm", "PongL.asm"];
    let output_file_name = vec!["Add.hack", "MaxL.hack", "RectL.hack", "PongL.hack"];
    
    for i in 0..input_file_name.len() {
        let input_args = format!("{}{}", input_file_pass, input_file_name[i]);
        let output_args = format!("{}{}", output_file_pass, output_file_name[i]);

        let lines = Lines::new(&input_args);
        let bins = lines.to_binary();

        let mut output = String::new();
        for bin in bins {
            output = output + &bin + &"\n".to_string();
        } 

        let mut file = BufWriter::new(File::create(output_args).unwrap());
        write!(file, "{}", output).unwrap();
        file.flush().unwrap();
    }
}