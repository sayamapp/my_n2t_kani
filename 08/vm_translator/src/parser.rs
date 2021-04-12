use std::{ffi::OsString, fs::File};
use std::io::Read;

#[derive(Debug)]
pub struct Parser {
    lines: Vec<String>,
    idx: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CommandType {
    CArithmetic(String),
    CPush(String, usize),
    CPop(String, usize),
    CLabel(String),
    CGoto(String),
    CIf(String),
    CFunction(String, usize),
    CReturn,
    CCall(String, usize),
    NotCommand,
}
impl CommandType {
    fn new(words: Vec<&str>) -> Self {
        if let Some(&word) = words.get(0) {
            match word {
                "push" => {
                    CommandType::CPush(words[1].to_string(), words[2].parse::<usize>().unwrap())
                },
                "pop" => {
                    CommandType::CPop(words[1].to_string(), words[2].parse::<usize>().unwrap())
                }
                "add" => CommandType::CArithmetic(words[0].to_string()),
                "eq" => CommandType::CArithmetic(words[0].to_string()),
                "lt" => CommandType::CArithmetic(words[0].to_string()),
                "gt" => CommandType::CArithmetic(words[0].to_string()),
                "sub" => CommandType::CArithmetic(words[0].to_string()),
                "neg" => CommandType::CArithmetic(words[0].to_string()),
                "and" => CommandType::CArithmetic(words[0].to_string()),
                "or" => CommandType::CArithmetic(words[0].to_string()),
                "not" => CommandType::CArithmetic(words[0].to_string()),

                "function" => CommandType::CFunction(words[1].to_string(), words[2].parse::<usize>().unwrap()),
                "return" => CommandType::CReturn,
                "label" => CommandType::CLabel(words[1].to_string()),
                "goto" => CommandType::CGoto(words[1].to_string()),
                
                "call" => CommandType::CCall(words[1].to_string(), words[2].parse::<usize>().unwrap()),

                "if-goto" => CommandType::CIf(words[1].to_string()),
                _ => CommandType::NotCommand,
            }
        } else {
            CommandType::NotCommand
        }
    }
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found!");
        let mut strings = String::new();
        file.read_to_string(&mut strings)
            .expect("Something went wrong reading the file!");

        let test = strings
            .split('\n')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        Parser {
            lines: test,
            idx: 0,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.lines.get(self.idx).is_some()
    }

    pub fn advance(&mut self) {
        self.idx += 1;
    }

    pub fn command_type(&self) -> CommandType {
        let line = self.lines.get(self.idx).unwrap();
        let line = line.split("//").collect::<Vec<&str>>();
        let words = line[0].split_whitespace().collect::<Vec<&str>>();
        CommandType::new(words)
    }
}
