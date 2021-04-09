use std::{fs::File, io::Read};

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
pub struct Lines {
    line_number: usize,
    pub lines: Vec<Line>,
}

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

        // Lines(lines) 
        Lines{line_number: 0, lines: lines}
    }

    pub fn to_binary(&self, symbol_table: &SymbolTable) -> Vec<String> {
        let mut binaries = Vec::new();
        for line in &self.lines {
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

