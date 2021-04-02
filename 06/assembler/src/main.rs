mod parser;
mod code;
mod symbol_table;
mod assembler;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::parser::Lines;
use crate::symbol_table::SymbolTable;

fn main() {
    let input_file_pass = "../asm_hack/";
    let output_file_pass = "../asm_hack/";
    let input_file_name = vec!["Add.asm", "MaxL.asm", "RectL.asm", "PongL.asm", "Max.asm", "Rect.asm", "Pong.asm"];
    let output_file_name = vec!["Add.hack", "MaxL.hack", "RectL.hack", "PongL.hack", "Max.hack", "Rect.hack", "Pong.hack"];
    // let input_file_name = vec!["Pong.asm"];
    // let output_file_name = vec!["Pong.hack"];
    
    for i in 0..input_file_name.len() {
        let input_args = format!("{}{}", input_file_pass, input_file_name[i]);
        let output_args = format!("{}{}", output_file_pass, output_file_name[i]);

        let lines = Lines::new(&input_args);

// 2nd
        let mut symbol_table = SymbolTable::new();

        let mut row = 0;
        let mut count = 0;
        for line in &lines.lines  {
            match line {
                parser::Line::ACommand(_) => {
                    count += 1;
                }
                parser::Line::CCommand(_) => {
                    count += 1;
                }
                parser::Line::LCommand(s) => {
                    for i in count.. {
                        if i >= lines.lines.len() {break;}
                        let command = lines.lines.get(row - count + i);
                        match command {
                            Some(n) => {
                                match n {
                                    parser::Line::ACommand(_) => {
                                        symbol_table.add_entry(s, count);
                                        break;
                                    }
                                    parser::Line::CCommand(_) => {
                                        symbol_table.add_entry(s, count);
                                        break;
                                    }
                                    parser::Line::LCommand(_) => {}
                                    parser::Line::NotCommand => {}
                                }
                            }
                            None => {}
                        }
                    }
                }
                parser::Line::NotCommand => {}
            }
            row += 1;
        }

        let mut address = 16;
        for line in &lines.lines {
            match line {
                parser::Line::ACommand(s) => {
                    let value = s.parse::<usize>();
                    if let Err(_) = value {
                        if !symbol_table.contains(&s) {
                            symbol_table.add_entry(s, address);
                            address += 1;
                        }
                    }

                }
                parser::Line::CCommand(_) => {}
                parser::Line::LCommand(_) => {}
                parser::Line::NotCommand => {}
            }
        }

        println!("{:?}", symbol_table);



//
        let bins = lines.to_binary(symbol_table);

        let mut output = String::new();
        for bin in bins {
            output = output + &bin + &"\n".to_string();
        } 

        let mut file = BufWriter::new(File::create(output_args).unwrap());
        write!(file, "{}", output).unwrap();
        file.flush().unwrap();
    }
}