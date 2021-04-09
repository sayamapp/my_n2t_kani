mod parser;
mod code;
mod symbol_table;
mod assembler;

use std::fs::File;
use std::io::{BufWriter, Write};
use crate::assembler::Assembler;

fn main() {
    let input_file_pass = "../files/";
    let output_file_pass = "../files/";
    let input_file_name = vec!["Add.asm", "MaxL.asm", "RectL.asm", "PongL.asm", "Max.asm", "Rect.asm", "Pong.asm"];
    let output_file_name = vec!["Add.hack", "MaxL.hack", "RectL.hack", "PongL.hack", "Max.hack", "Rect.hack", "Pong.hack"];
    // let input_file_name = vec!["Add.asm"];
    // let output_file_name = vec!["Add.hack"];
    
    for i in 0..input_file_name.len() {
        let input_args = format!("{}{}", input_file_pass, input_file_name[i]);
        let output_args = format!("{}{}", output_file_pass, output_file_name[i]);

        let mut assembler = Assembler::new(&input_args);
        assembler.assemble();
        let output = assembler.to_binary();

        let mut file = BufWriter::new(File::create(output_args).unwrap());
        write!(file, "{}", output).unwrap();
        file.flush().unwrap();
    }
}