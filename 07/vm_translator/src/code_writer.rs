use std::{
    fmt::format,
    fs::File,
    io::{BufWriter, Write},
};

use crate::parser::CommandType;
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
                if command == "add" {
                    self.commands.push("// ADD //".to_string());
                    self.commands.push(pop());
                    self.commands.push(add("D=D+M"));
                }
                if command == "eq" {
                    self.commands.push("// EQ //".to_string());
                    self.commands.push(pop());
                    self.commands.push(eq("JEQ", self.commands.len()));
                }
                if command == "lt" {
                    self.commands.push("// LT //".to_string());
                    self.commands.push(pop());
                    self.commands.push(eq("JLT", self.commands.len()));
                }
                if command == "gt" {
                    self.commands.push("// GT //".to_string());
                    self.commands.push(pop());
                    self.commands.push(eq("JGT", self.commands.len()));
                }
                if command == "sub" {
                    self.commands.push("// SUB //".to_string());
                    self.commands.push(pop());
                    self.commands.push(add("D=M-D"));
                }
                if command == "neg" {
                    self.commands.push("// NEG //".to_string());
                    self.commands.push(neg());
                }
                if command == "and" {
                    self.commands.push("// AND //".to_string());
                    self.commands.push(pop());
                    self.commands.push(add("D=D&M"));
                }
                if command == "or" {
                    self.commands.push("// OR //".to_string());
                    self.commands.push(pop());
                    self.commands.push(add("D=D|M"));
                }
                if command == "not" {
                    self.commands.push("// NOT //".to_string());
                    self.commands.push(not());
                }

            },
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
                if segment == "constant" {
                    self.commands.push("// PUSH //".to_string());
                    self.commands.push(push(&index));
                }
            }
            CommandType::CPop(segment, index) => {}
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

fn push(idx: &usize) -> String {
    let dist = format!("@{}", idx);
    let v: Vec<&str> = vec![&dist, "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1", ""];
    v.join("\n")
}

fn pop() -> String {
    let v = vec!["@SP", "M=M-1", "A=M", "D=M", ""];
    v.join("\n")
}

fn add(command: &str) -> String {
    let v = vec![
        "@SP", "M=M-1", "A=M", command, "", "@SP", "A=M", "M=D", "@SP", "M=M+1", "",
    ];

    v.join("\n")
}

fn neg() -> String {
    let v = vec![
        "@SP",
        "M=M-1",
        "A=M",
        "M=-M",
        "@SP",
        "M=M+1",
    ];
    v.join("\n")
}

fn not() -> String {
    let v = vec![
        "@SP",
        "M=M-1",
        "A=M",
        "M=!M",
        "@SP",
        "M=M+1",
    ];
    v.join("\n")
}


fn eq(command: &str, n: usize) -> String {
    let a_true = format!("@TRUE.{}", n);
    let a_false = format!("@FALSE.{}", n);
    let a_end = format!("@END.{}", n);
    let l_true = format!("(TRUE.{})", n);
    let l_false = format!("(FALSE.{})", n);
    let l_end = format!("(END.{})", n);
    let command = format!("D;{}", command);

    let v = vec![
        "@SP",
        "M=M-1",
        "A=M",
        "D=M-D",

        &a_true,
        &command,

        &a_false,
        "0;JMP",

        &l_true,
        "@0",
        "D=!A",
        "@SP",
        "A=M",
        "M=D",
        
        &a_end,
        "0;JMP",

        &l_false,
        "@0",
        "D=A",
        "@SP",
        "A=M",
        "M=D",

        &l_end,
        "@SP",
        "M=M+1",

    ];
    v.join("\n")
}
