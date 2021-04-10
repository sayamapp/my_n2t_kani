use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::parser::CommandType;

use crate::binary_function;
use crate::unary_function;
use crate::logic_command;
use crate::push;
use crate::pop;
use crate::push_to;
use crate::pop_to;
use crate::push_constant;

pub struct CodeWriter {
    file_name: String,
    commands: Vec<String>,
}

impl CodeWriter {
    pub fn new(path: &str) -> Self {
        CodeWriter {
            file_name: path.to_string(),
            commands: Vec::new(),
        }
    }

    pub fn write_arithmetic(&mut self, command: &CommandType) {
        match command {
            CommandType::CArithmetic(command) => {
                let com: &str = command;
                let n = self.commands.len();
                match com {
                    "add" => self.commands.push(binary_function!("// ADD", "D=D+M")),
                    "sub" => self.commands.push(binary_function!("// SUB", "D=M-D")),
                    "and" => self.commands.push(binary_function!("// AND", "D=D&M")),
                    "or" => self.commands.push(binary_function!("// OR", "D=D|M")),

                    "not" => self.commands.push(unary_function!("// NOT", "M=!M")),
                    "neg" => self.commands.push(unary_function!("// NEG", "M=-M")),

                    "eq" => self.commands.push(logic_command!("// EQ", "D;JEQ", n)),
                    "lt" => self.commands.push(logic_command!("// LT", "D;JLT", n)),
                    "gt" => self.commands.push(logic_command!("// GT", "D;JGT", n)),
                    _ => {}
                }
            }
            CommandType::CPush(_, _) => {}
            CommandType::CPop(_, _) => {}
            CommandType::CLabel => {}
            CommandType::CGoto => {}
            CommandType::CIf => {}
            CommandType::CFunction(_, _) => {}
            CommandType::CReturn => {}
            CommandType::CCall(_, _) => {}
            CommandType::NotCommand => {}
        }
    }

    pub fn write_push_pop(&mut self, command: &CommandType) {
        match command {
            CommandType::CPush(segment, index) => {
                let seg: &str = segment;
                match seg {
                    "constant" => self.commands.push(push_constant!(&index)),
                    "argument" => self.commands.push(push_to!("@ARG", &index)),
                    "local" => self.commands.push(push_to!("@LCL", &index)),
                    "that" => self.commands.push(push_to!("@THAT", &index)),
                    "this" => self.commands.push(push_to!("@THIS", &index)),
                    "temp" => self.commands.push(push_to!("@TEMP", &index)),
                    _ => {},
                }
            }
            CommandType::CPop(segment, index) => {
                let seg: &str = segment;
                match seg {
                    "local" => self.commands.push(pop_to!("@LCL", &index)),
                    "argument" => self.commands.push(pop_to!("@ARG", &index)),
                    "this" => self.commands.push(pop_to!("@THIS", &index)),
                    "that" => self.commands.push(pop_to!("@THAT", &index)),
                    "temp" => self.commands.push(pop_to!("@TEMP", &index)),
                    _ => {},
                }
            }
            _ => {}
        }
    }

    pub fn close(&self) {
        let mut output = String::new();
        for line in &self.commands {
            output = output + &line + &"\n";
        }
        let mut file = BufWriter::new(File::create(&self.file_name).unwrap());
        write!(file, "{}", output).unwrap();
        file.flush().unwrap();
    }
}


#[macro_export]
macro_rules! pop {
    () => {
        "@SP\nM=M-1\nA=M"
    };
}

#[macro_export]
macro_rules! push {
    () => {
        "@SP\nA=M\nM=D\n@SP\nM=M+1"
    };
}

#[macro_export]
macro_rules! push_constant {
    ($n: expr) => {{
        let n: &str = &format!("@{}", $n);
        let commands = vec![
            n,
            "D=A",
            push!(),
        ];
        commands.join("\n")
    }
        
    };
}

#[macro_export]
macro_rules! pop_to {
    ($dist: expr, $idx: expr) => {{
        let index = format!("@{}", $idx);
        let commands = vec![
            if $dist == "@TEMP" {"@5\nD=A"} else {$dist},
            "D=M",
            &index,
            "D=D+A",
            "@SP",
            "A=M",
            "M=D",
            pop!(),
            "D=M",
            "@SP",
            "A=M",
            "A=A+1",
            "A=M",
            "M=D",
        ];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! push_to {
    ($dist: expr, $idx: expr) => {{
        let index = format!("@{}", $idx);
        let commands = vec![
            $dist, 
            if $dist == "@TEMP" {"@5\nD=A"} else {"A=M\nD=A"},
            &index,
            "D=D+A",
            "A=D",
            "D=M",
            push!(),
        ];
        commands.join("\n")
    }
        
    };
}

#[macro_export]
macro_rules! unary_function {
    ($comment: expr, $x: expr) => {{
        let commands = vec![pop!(), $x, "@SP", "M=M+1"];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! binary_function {
    ($comment: expr, $x: expr) => {{
        let v = vec![$comment, pop!(), "D=M", pop!(), $x, push!()];
        v.join("\n")
    }};
}

#[macro_export]
macro_rules! logic_command {
    ($comment: expr, $command: expr, $n: expr) => {{
        let at = format!("@TRUE.{}", $n);
        let af = format!("@FALSE.{}", $n);
        let ae = format!("@END.{}", $n);
        let lt = format!("(TRUE.{})", $n);
        let lf = format!("(FALSE.{})", $n);
        let le = format!("(END.{})", $n);
        let commands = vec![
            $comment,
            pop!(),
            "D=M",
            pop!(),
            "D=M-D",
            &at,
            $command,
            &af,
            "0;JMP",
            &lt,
            "@0",
            "D=!A",
            &ae,
            "0;JMP",
            &lf,
            "@0",
            "D=A",
            &le,
            push!(),
        ];
        commands.join("\n")
    }};
}