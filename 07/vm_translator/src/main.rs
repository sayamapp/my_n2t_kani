mod parser;
mod code_writer;
use parser::CommandType;

use crate::parser::Parser;
use crate::code_writer::CodeWriter;


fn main() {
    let input_file_path = "../MemoryAccess/StaticTest/StaticTest.vm";
    let output_file_path = "../MemoryAccess/StaticTest/StaticTest.asm";
    // let input_file_path = "../StackArithmetic/StackTest/StackTest.vm";
    // let output_file_path = "../StackArithmetic/StackTest/StackTest.asm";
    let mut parser = Parser::new(input_file_path);
    let mut code_writer = CodeWriter::new(output_file_path);

    
    while parser.has_more_commands() {
        let command = parser.command_type();

        match &command {
            CommandType::CArithmetic(_) => {code_writer.write_arithmetic(&command)}
            CommandType::CPush(_, _) => {code_writer.write_push_pop(&command)},
            CommandType::CPop(_, _) => {code_writer.write_push_pop(&command)},
            CommandType::CLabel => {}
            CommandType::CGoto => {}
            CommandType::CIf => {}
            CommandType::CFunction(_, _) => {}
            CommandType::CReturn => {}
            CommandType::CCall(_, _) => {}
            CommandType::NotCommand => {}
        }

        println!("{:?}", command);
        parser.advance();
    }

    code_writer.close();
}
