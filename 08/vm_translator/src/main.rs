mod code_writer;
mod parser;
use parser::CommandType;

use crate::code_writer::CodeWriter;
use crate::parser::Parser;

use std::fs;
use std::path::Path;

fn main() {
    let input_file_path = "./FunctionCalls/SimpleFunction/SimpleFunction.vm";
    let output_file_path = "./FunctionCalls/SimpleFunction/SimpleFunction.asm";
    // let input_file_path = "../StackArithmetic/StackTest/StackTest.vm";
    // let output_file_path = "../StackArithmetic/StackTest/StackTest.asm";

    let input_file_path = "./FunctionCalls/FibonacciElement/";
    let output_file_path = "./FunctionCalls/FibonacciElement/FibonacciElement.asm";

    let mut code_writer = CodeWriter::new(output_file_path);
    code_writer.write_init();

    match fs::read_dir(input_file_path) {
        Err(why) => println!("Not found dir!"),
        Ok(paths) => {
            for path in paths {
                match path {
                    Err(_) => println!("?????"),
                    Ok(path) => {
                        let path = path.file_name();
                        let path = format!("{}{}", input_file_path, path.to_str().unwrap());
                        let mut parser = Parser::new(&path);

                        while parser.has_more_commands() {
                            let command = parser.command_type();

                            match &command {
                                CommandType::CArithmetic(_) => {
                                    code_writer.write_arithmetic(&command)
                                }
                                CommandType::CPush(_, _) => code_writer.write_push_pop(&command),
                                CommandType::CPop(_, _) => code_writer.write_push_pop(&command),
                                CommandType::CLabel(_) => code_writer.write_label(&command),
                                CommandType::CGoto(_) => code_writer.write_goto(&command),
                                CommandType::CIf(_) => code_writer.write_if(&command),
                                CommandType::CFunction(_, _) => {
                                    code_writer.write_function(&command)
                                }
                                CommandType::CReturn => code_writer.write_return(),
                                CommandType::CCall(_, _) => code_writer.write_call(&command),
                                CommandType::NotCommand => {}
                            }

                            println!("{:?}", command);
                            parser.advance();
                        }
                    }
                }
            }
        }
    }

    code_writer.close();
}
