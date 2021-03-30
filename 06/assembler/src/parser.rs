use std::{fs::File, io::Read, str};

use crate::commands::CCommand;

struct Commands(Vec<Command>);
enum Command {
    ACommand(ACommand),
    CCommand(CCommand),
    LCommand(LCommand),
    NotCommand,
}
enum ACommand {
    Value(usize),
    Symbol(String),
}
impl ACommand {
    pub fn new(s: &str) -> Self {
        let s = s.to_string();
        s.retain(|c| c != '@');
        let value = s.parse::<usize>();
        if let Ok(value) = value {
            ACommand::Value(value)
        } else {
            ACommand::Symbol(s)
        }
    }
}
struct LCommand(String);

pub struct Parser {
    lines: Lines,
    commands: Vec<Command>,
    binary_codes: Vec<Option<String>>,
}

#[derive(Debug)]
pub struct Lines(Vec<Line>);
impl Lines {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found");
        let mut strings = String::new();
        file.read_to_string(&mut strings).expect("Something went wrong reading the file!");

        let mut lines = Vec::new();
        for line in strings.split('\n') {
            lines.push(Line::new(line));
        }
        Lines(lines)
    }

    pub fn to_commands(&self) -> Commands {
        let mut commands : Vec<Command> = Vec::new();
        for line in &self.0 {
            if let Some(line) = &line.0 {
                match line.chars().nth(0) {
                    Some('@') => {
                        commands.push(Command::ACommand(ACommand::new(line)));
                    },
                    Some('(') => {
                        let symbol = line.to_string();
                        symbol.retain(|c| c != '(' && c != ')');
                        commands.push(Command::LCommand(LCommand(symbol)));
                    },
                    Some(_) => {

                    },
                    None => {
                        commands.push(Command::NotCommand);
                    }
                }
            } else {
                commands.push(Command::NotCommand);
            }
        }
        Commands(commands)
    }
}

#[derive(Debug)]
struct Line(Option<String>);
impl Line {
    pub fn new(line: &str) -> Self {
        println!("{:?}", line);
        let line = line.trim();
        let line: Vec<&str> = line.split("//").collect();
        let line[0].to_string().retain(|c| c != ' ');

        if line[0] != "" {
            Line(Some(line[0].to_string()))
        } else {
            Line(None)
        }
    }
}
// use std::fs::File;
// use std::io::Read;
// use std::collections::HashMap;

// use crate::commands::{Dest, Comp, Jump};

// #[derive(Debug)]
// pub struct Commands(Vec<Command>);

// struct Line {
//     dest: Option<String>,
//     comp: Option<String>,
//     jump: Option<String>,
// }

// #[derive(Debug)]
// enum Command {
//     ACommand(ACommand),
//     CCommand(CCommand),
//     LCommand(Symbol),
// }

// type Symbol = String;

// #[derive(Debug)]
// enum ACommand {
//     Symbol(Symbol),
//     Value(usize),
// }

// #[derive(Debug)]
// struct CCommand {
//     dest: Option<Dest>,
//     comp: Comp,
//     jump: Option<Jump>,
// }

// impl Commands {
//     pub fn new(path: String) -> Self {
//         let mut f = File::open(path).expect("File not found!");
//         let mut lines = String::new();
//         f.read_to_string(&mut lines)
//             .expect("Something wernt wrong reading the file");

//         let mut iter = lines.split('\n');
//         let mut commands = Vec::new();

//         for line in iter {
//             let line = Self::format_line(line);
//             if let Some(line) = line {
//                 if line.dest == None && line.jump == None {
//                     let comp = line.comp.unwrap();
//                     if &comp[0..1] == "@" {
//                         let symbol = &comp[1..].to_string();
//                         let value = symbol.parse::<usize>();
//                         if let Ok(value) = value {
//                             commands.push(
//                                 Command::ACommand(ACommand::Value(value))
//                             );
//                         } else {
//                             commands.push(
//                                 Command::ACommand(ACommand::Symbol(symbol.to_string()))
//                             );
//                         }
//                     } else {
//                         let symbol = &comp[1..comp.len()].to_string();
//                         commands.push(
//                             Command::LCommand(symbol.to_string())
//                         );
//                     }
//                 } else {
//                     let dest =
//                         if let Some(dest) = line.dest {
//                             Some(Dest::to_enum(dest))
//                         } else {
//                             None
//                         };

//                     let comp =
//                         Comp::to_enum(line.comp.unwrap());

//                     let jump =
//                             if let Some(jump) = line.jump {
//                                 Some(Jump::to_enum(jump))
//                             } else {
//                                 None
//                             };

//                     commands.push(
//                         Command::CCommand(
//                             CCommand {
//                                 dest: dest,
//                                 comp: comp,
//                                 jump: jump,
//                             }
//                         )
//                     )
//                 }
//             }
//         }
//         Commands(commands)
//     }

//     fn format_line(line: &str) -> Option<Line> {
//         let line = line.trim();
//         let line: Vec<&str> = line.split("//").collect();
//         // println!("{:?}", line);
//         if line[0] == "" { return None; }
//         let line: Vec<&str> = line[0].trim().split(";").collect();
//         let (dest_comp, jump) = if line.len() == 2 {
//             (line[0], Some(line[1]))
//         } else { (line[0], None) };
//         let line: Vec<&str> = dest_comp.trim().split("=").collect();
//         let (dest, comp) = if line.len() == 2 {
//             (Some(line[0]), Some(line[1]))
//         } else {
//             (None, Some(line[0]))
//         };
//         let dest = dest.and_then(|x| Self::del_space(x));
//         let comp = comp.and_then(|x| Self::del_space(x));
//         let jump = jump.and_then(|x| Self::del_space(x));

//         println!("DEST: {:?} CMP: {:?}  JMP: {:?}", dest, comp, jump);
//         Some(Line { dest, comp, jump })
//     }

//     fn del_space(str: &str) -> Option<String> {
//         let str = str.chars();
//         let mut res = Vec::new();
//         for c in str {
//             if !c.is_whitespace() {
//                 res.push(c);
//             }
//         }
//         Some(res.into_iter().collect())
//     }
// }
