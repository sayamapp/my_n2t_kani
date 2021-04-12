mod code_writer;
mod parser;



use parser::CommandType;

use crate::code_writer::CodeWriter;
use crate::parser::Parser;

use std::fs;

fn main() {
    let input_file_path = "../FunctionCalls/NestedCall/";
    let output_file_path = "../FunctionCalls/NestedCall/NestedCall.asm";

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

                            match command {
                                CommandType::CArithmetic(arithmetic) => 
                                    code_writer.write_arithmetic(arithmetic),

                                CommandType::CPush(_, _, _) => 
                                    code_writer.write_push_pop(command),
                                CommandType::CPop(_, _, _) => 
                                    code_writer.write_push_pop(command),

                                CommandType::CLabel(label) => 
                                    code_writer.write_label(label),
                                CommandType::CGoto(label) => 
                                    code_writer.write_goto(label),
                                CommandType::CIf(label) => 
                                    code_writer.write_if(label),

                                CommandType::CFunction(f, n) => 
                                    code_writer.write_function(f, n),
                                CommandType::CReturn => 
                                    code_writer.write_return(),
                                CommandType::CCall(f, n) => 
                                    code_writer.write_call(f, n),

                                CommandType::NotCommand => {}
                            }
                            
                            parser.advance();
                        }
                    }
                }
            }
        }
    }

    code_writer.close();
}
