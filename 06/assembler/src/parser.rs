use std::{fmt::format, fs::File, io::Read};

use crate::code::*;
use crate::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct CCommand {
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}
impl CCommand {
    fn new(s: &str) -> Self {
        let mut ccommand = CCommand{dest: None, comp: "NOP".to_string(), jump: None};
        let dc_j = s.split(";").collect::<Vec<&str>>();
        if dc_j.len() > 1 {
            ccommand.jump = Some(dc_j[1].to_string());
        }
        let d_c = dc_j[0].split("=").collect::<Vec<&str>>();
        if d_c.len() > 1 {
            ccommand.dest = Some(d_c[0].to_string());
            ccommand.comp = d_c[1].to_string();
        } else {
            ccommand.comp = d_c[0].to_string();
        }

        ccommand
    }
}

#[derive(Debug)]
pub enum Line {
    ACommand(String),
    CCommand(CCommand),
    LCommand(String),
    NotCommand,
}
#[derive(Debug)]
pub struct Lines(pub Vec<Line>);
impl Lines {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found!");
        let mut strings = String::new();
        file.read_to_string(&mut strings).expect("Something went wrong reading the file!");

        let mut lines = Vec::new();
        for line in strings.split('\n') {
            let line = line.to_string();
            let line = line.split("//").collect::<Vec<&str>>()[0];
            let line = line.trim();

            if line == "" {lines.push(Line::NotCommand)}
            else if line.chars().nth(0).unwrap() == '@' {
                let mut line = line.to_string();
                line.retain(|c| c != '@' && c != ' ');
                lines.push(Line::ACommand(line.to_string()));
            }
            else if line.chars().nth(0).unwrap() == '(' {
                let mut line = line.to_string();
                line.retain(|c| c != '(' && c !=')' && c != ' ');
                lines.push(Line::LCommand(line.to_string()));
            }
            else {
                let mut line = line.to_string();
                line.retain(|c| c != ' ');
                lines.push(Line::CCommand(CCommand::new(&line)));
            } 

        }

        Lines(lines) 
    }

    pub fn to_binary(&self, symbol_table: SymbolTable) -> Vec<String> {
        let mut binaries = Vec::new();
        for line in &self.0 {
            match line {
                Line::ACommand(s) => {
                    let value = s.parse::<usize>();
                    if let Ok(value) = value {
                        let v_string = format!("0{:015b}", value);
                        binaries.push(v_string);
                    } else {
                        let symbol = s;
                        let value = symbol_table.get_address(&symbol);
                        let v_string = format!("0{:015b}", value);
                        binaries.push(v_string);
                    }
                },
                Line::CCommand(c) => {
                    let dest = dest_to_binary(&c.dest);
                    let comp = comp_to_binary(&c.comp);
                    let jump = jump_to_binary(&c.jump);
                    let c_string = format!("111{}{}{}", comp, dest, jump);
                    binaries.push(c_string);
                },
                Line::LCommand(_) => {
                }
                Line::NotCommand => {}
            }
        }
        binaries
    }
}



// use crate::commands::CCommand;

// #[derive(Debug)]
// pub struct Commands(Vec<Command>);

// #[derive(Debug)]
// pub enum Command {
//     ACommand(ACommand),
//     CCommand(CCommand),
//     LCommand(LCommand),
//     NotCommand,
// }
// #[derive(Debug)]
// pub enum ACommand {
//     Value(usize),
//     Symbol(String),
// }
// impl ACommand {
//     pub fn new(s: &str) -> Self {
//         let mut s = s.to_string();
//         s.retain(|c| c != '@');
//         let value = s.parse::<usize>();
//         if let Ok(value) = value {
//             ACommand::Value(value)
//         } else {
//             ACommand::Symbol(s)
//         }
//     }
// }
// #[derive(Debug)]
// struct LCommand(String);
// impl LCommand {
//     pub fn new(s: &str) -> Self {
//         let mut s = s.to_string();
//         s.retain(|c| c != '(' && c !=')');
//         LCommand(s)
//     }
// }

// pub struct Parser {
//     lines: Lines,
//     commands: Vec<Command>,
//     binary_codes: Vec<Option<String>>,
// }

// #[derive(Debug)]
// pub struct Lines(Vec<Line>);
// impl Lines {
//     pub fn new(path: &str) -> Self {
//         let mut file = File::open(path).expect("File not found");
//         let mut strings = String::new();
//         file.read_to_string(&mut strings).expect("Something went wrong reading the file!");

//         let mut lines = Vec::new();
//         for line in strings.split('\n') {
//             lines.push(Line::new(line));
//         }
//         Lines(lines)
//     }

//     pub fn to_commands(&self) -> Commands {
//         let mut commands : Vec<Command> = Vec::new();
//         for line in &self.0 {
//             if let Some(line) = &line.0 {
//                 match line.chars().nth(0) {
//                     Some('@') => {
//                         commands.push(Command::ACommand(ACommand::new(line)));
//                     },
//                     Some('(') => {
//                         let mut symbol = line.to_string();
//                         symbol.retain(|c| c != '(' && c != ')');
//                         commands.push(Command::LCommand(LCommand(symbol)));
//                     },
//                     Some(_) => {
//                         commands.push(Command::CCommand(CCommand::new(line)));
//                     },
//                     None => {
//                         commands.push(Command::NotCommand);
//                     }
//                 }
//             } else { 
//                 commands.push(Command::NotCommand);
//             }
//         }
//         Commands(commands)
//     }
// }

// #[derive(Debug)]
// struct Line(Option<String>);
// impl Line {
//     pub fn new(line: &str) -> Self {
//         println!("{:?}", line);
//         let line = line.trim();
//         let line: Vec<&str> = line.split("//").collect();
//         line[0].to_string().retain(|c| c != ' ');

//         if line[0] != "" {
//             Line(Some(line[0].to_string()))
//         } else {
//             Line(None)
//         }
//     }
// }
