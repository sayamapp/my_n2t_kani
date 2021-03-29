use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use crate::commands::{Dest, Comp, Jump};

#[derive(Debug)]
pub struct Commands(Vec<Command>);

struct Line {
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>,
}

#[derive(Debug)]
enum Command {
    ACommand(ACommand),
    CCommand(CCommand),
    LCommand(Symbol),
}

type Symbol = String;

#[derive(Debug)]
enum ACommand {
    Symbol(Symbol),
    Value(usize),
}

#[derive(Debug)]
struct CCommand {
    dest: Option<Dest>,
    comp: Comp,
    jump: Option<Jump>,
}


impl Commands {
    pub fn new(path: String) -> Self {
        let mut f = File::open(path).expect("File not found!");
        let mut lines = String::new();
        f.read_to_string(&mut lines)
            .expect("Something wernt wrong reading the file");

        let mut iter = lines.split('\n');
        let mut commands = Vec::new();

        for line in iter {
            let line = Self::format_line(line);
            if let Some(line) = line {
                if line.dest == None && line.jump == None {
                    let comp = line.comp.unwrap();
                    if &comp[0..1] == "@" {
                        let symbol = &comp[1..].to_string();
                        let value = symbol.parse::<usize>();
                        if let Ok(value) = value {
                            commands.push(
                                Command::ACommand(ACommand::Value(value))
                            );
                        } else {
                            commands.push(
                                Command::ACommand(ACommand::Symbol(symbol.to_string()))
                            );
                        }
                    } else {
                        let symbol = &comp[1..comp.len()].to_string();
                        commands.push(
                            Command::LCommand(symbol.to_string())
                        );
                    }
                } else {
                    let dest =
                        if let Some(dest) = line.dest {
                            Some(Dest::to_enum(dest))
                        } else {
                            None
                        };

                    let comp =
                        Comp::to_enum(line.comp.unwrap());

                    let jump =
                            if let Some(jump) = line.jump {
                                Some(Jump::to_enum(jump))
                            } else {
                                None
                            };

                    commands.push(
                        Command::CCommand(
                            CCommand {
                                dest: dest,
                                comp: comp,
                                jump: jump,
                            }
                        )
                    )
                }
            }
        }
        Commands(commands)
    }

    fn format_line(line: &str) -> Option<Line> {
        let line = line.trim();
        let line: Vec<&str> = line.split("//").collect();
        // println!("{:?}", line);
        if line[0] == "" { return None; }
        let line: Vec<&str> = line[0].trim().split(";").collect();
        let (dest_comp, jump) = if line.len() == 2 {
            (line[0], Some(line[1]))
        } else { (line[0], None) };
        let line: Vec<&str> = dest_comp.trim().split("=").collect();
        let (dest, comp) = if line.len() == 2 {
            (Some(line[0]), Some(line[1]))
        } else {
            (None, Some(line[0]))
        };
        let dest = dest.and_then(|x| Self::del_space(x));
        let comp = comp.and_then(|x| Self::del_space(x));
        let jump = jump.and_then(|x| Self::del_space(x));

        println!("DEST: {:?} CMP: {:?}  JMP: {:?}", dest, comp, jump);
        Some(Line { dest, comp, jump })
    }

    fn del_space(str: &str) -> Option<String> {
        let str = str.chars();
        let mut res = Vec::new();
        for c in str {
            if !c.is_whitespace() {
                res.push(c);
            }
        }
        Some(res.into_iter().collect())
    }
}
