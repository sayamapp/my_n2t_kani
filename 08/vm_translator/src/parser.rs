use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Parser {
    name: String,
    lines: Vec<String>,
    idx: usize,
}
impl Parser {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found!");
        let class_name = path.split("/").collect::<Vec<&str>>().pop().unwrap();
        println!("___ {}", class_name);
        let mut strings = String::new();
        file.read_to_string(&mut strings)
            .expect("Something went wrong reading the file!");

        let test = strings
            .split('\n')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        Parser {
            name: class_name.to_string(),
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
        CommandType::new(self.name.clone(), words)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Arithmetic {
    Add,
    Sub,
    And,
    Or,
    Not,
    Neg,
    Eq,
    Lt,
    Gt,
}
#[derive(Debug, Eq, PartialEq)]
pub enum CommandType {
    CArithmetic(Arithmetic),
    CPush(String, String, usize),
    CPop(String, String, usize),
    CLabel(String),
    CGoto(String),
    CIf(String),
    CFunction(String, usize),
    CReturn,
    CCall(String, usize),
    NotCommand,
}
impl CommandType {
    fn new(class_name: String, words: Vec<&str>) -> Self {
        println!("PARSER {:?}", words);
        match &words.len() {
            1 => {
                let command = words[0];
                match command {
                    "add"   => CommandType::CArithmetic(Arithmetic::Add),
                    "sub"   => CommandType::CArithmetic(Arithmetic::Sub),
                    "and"   => CommandType::CArithmetic(Arithmetic::And),
                    "or"    => CommandType::CArithmetic(Arithmetic::Or),
                    "not"   => CommandType::CArithmetic(Arithmetic::Not),
                    "neg"   => CommandType::CArithmetic(Arithmetic::Neg),
                    "eq"    => CommandType::CArithmetic(Arithmetic::Eq),
                    "lt"    => CommandType::CArithmetic(Arithmetic::Lt),
                    "gt"    => CommandType::CArithmetic(Arithmetic::Gt),
                    "return"=> CommandType::CReturn,
                    _ => CommandType::NotCommand,
                }
            },
            2 => {
                let command = words[0];
                let arg = words[1].to_string();
                match command {
                    "label"     => CommandType::CLabel(arg),
                    "goto"      => CommandType::CGoto(arg),
                    "if-goto"   => CommandType::CIf(arg),
                    _ => CommandType::NotCommand,
                }
            },
            3 => {
                let command = words[0];
                let arg1 = words[1].to_string();
                let arg2: Result<usize, _> = words[2].parse();
                match command {
                    "push"      => CommandType::CPush(class_name, arg1, arg2.unwrap()),
                    "pop"       => CommandType::CPop(class_name, arg1, arg2.unwrap()),
                    "function"  => CommandType::CFunction(arg1, arg2.unwrap()),
                    "call"      => CommandType::CCall(arg1, arg2.unwrap()),
                    _ => CommandType::NotCommand,
                }
            },
            _ => CommandType::NotCommand
        }
    }
}

