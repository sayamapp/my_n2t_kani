use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::parser::CommandType;

use crate::binary_function;
use crate::function_return;
use crate::logic_command;
use crate::pop;
use crate::pop_pointer;
use crate::pop_static;
use crate::pop_to;
use crate::push;
use crate::push_constant;
use crate::push_pointer;
use crate::push_static;
use crate::push_to;
use crate::unary_function;
use crate::write_call;
use crate::write_if;

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

    pub fn write_init(&mut self) {
        self.commands.push("@256\nD=A\n@SP\nM=D".to_string());
        let command = CommandType::CCall("Sys.init".to_string(), 0);
        self.write_call(&command);
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
            CommandType::CLabel(_) => {}
            CommandType::CGoto(_) => {}
            CommandType::CIf(_) => {}
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

                    "pointer" => self.commands.push(push_pointer!(index + 3)),
                    "static" => self.commands.push(push_pointer!(index + 16)),
                    _ => {}
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

                    "pointer" => self.commands.push(pop_pointer!(index + 3)),
                    "static" => self.commands.push(pop_pointer!(index + 16)),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn write_if(&mut self, command: &CommandType) {
        match command {
            CommandType::CIf(label) => {
                self.commands.push(write_if!(label));
            }
            _ => {},
        }
    }

    pub fn write_call(&mut self, command: &CommandType) {
        match command {
            CommandType::CCall(f, n) => self.commands.push(write_call!(f, n, self.commands.len())),
            _ => {}
        }
    }

    pub fn write_function(&mut self, command: &CommandType) {
        match command {
            CommandType::CFunction(name, locals) => {
                self.commands.push(function_call(name, *locals))
            }
            _ => {}
        }
    }

    pub fn write_return(&mut self) {
        self.commands.push(function_return!());
    }

    pub fn write_label(&mut self, command: &CommandType) {
        match command {
            CommandType::CLabel(label) => {
                let label = format!("({})", label);
                self.commands.push(label);
            }
            _ => {}
        }
    }

    pub fn write_goto(&mut self, command: &CommandType) {
        match command {
            CommandType::CGoto(label) => {
                let label = format!("@{}\n0;JMP", label);
                self.commands.push(label);
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

fn function_call(name: &str, locals: usize) -> String {
    let label: &str = &format!("({})", name);
    let mut commands = vec![label];
    for _ in 0..locals {
        commands.push("@SP\nA=M\nM=0\n@SP\nM=M+1")
    }
    commands.join("\n")
}

#[macro_export]
macro_rules! write_if {
    ($label: expr) => {{
        let ra = format!("@{}", $label);
        let command = vec![
            pop!(),
            "D=M",
            &ra,
            "D;JNE"
        ];
        command.join("\n")
    }};
}

#[macro_export]
macro_rules! write_call {
    ($f: expr, $n: expr, $len: expr) => {{
        let ra = format!("@return-address.{}", $len);
        let rl = format!("(return-address.{})", $len);
        let n = format!("@{}", $n);
        let f = format!("@{}", $f);
        let commands: Vec<&str> = vec![
            &ra,
            "D=A",
            push!(),
            "@LCL",
            "D=M",
            push!(),
            "@ARG",
            "D=M",
            push!(),
            "@THIS",
            "D=M",
            push!(),
            "@THAT",
            "D=M",
            push!(),
            "@SP",
            "D=M",
            &n,
            "D=D-A",
            "@5",
            "D=D-A",
            "@ARG",
            "M=D",
            "@SP",
            "D=M",
            "@LCL",
            "M=D",
            &f,
            "0;JMP",
            &rl,
        ];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! function_return {
    () => {{
        let commands = vec![
            "@LCL\nD=M\n@R13\nM=D",
            "@5\nA=D-A\nD=M\n@R14\nM=D",
            pop!(),
            "D=M\n@ARG\nA=M\nM=D",
            "@ARG\nD=M+1\n@SP\nM=D",
            "@R13\nD=M\n@1\nD=D-A\nA=D\nD=M\n@THAT\nM=D",
            "@R13\nD=M\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D",
            "@R13\nD=M\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D",
            "@R13\nD=M\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D",
            "@R14\nA=M\n0;JMP",
        ];
        commands.join("\n")
    }};
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
        let commands = vec![n, "D=A", push!()];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! pop_to {
    ($dist: expr, $idx: expr) => {{
        let index = format!("@{}", $idx);
        let commands = vec![
            if $dist == "@TEMP" { "@5\nD=A" } else { $dist },
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
            if $dist == "@TEMP" {
                "@5\nD=A"
            } else {
                "A=M\nD=A"
            },
            &index,
            "D=D+A",
            "A=D",
            "D=M",
            push!(),
        ];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! pop_pointer {
    ($idx: expr) => {{
        let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
        let commands = vec![pop!(), "D=M", &index, "M=D"];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! pop_static {
    ($idx: expr) => {{
        let index = format!("@{}", *$idx + 16);
        let commands = vec![pop!(), "D=M", &index, "M=D"];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! push_static {
    ($idx: expr) => {{
        let index = format!("@{}", *$idx + 16);
        let commands: Vec<&str> = vec![&index, "D=M", push!()];
        commands.join("\n")
    }};
}

#[macro_export]
macro_rules! push_pointer {
    ($idx: expr) => {{
        let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
        let commands: Vec<&str> = vec![&index, "D=M", push!()];
        commands.join("\n")
    }};
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
