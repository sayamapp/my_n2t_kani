use crate::parser::{Line, Lines};
use crate::symbol_table::SymbolTable;

pub struct Assembler {
    lines: Lines,
    symbol_table: SymbolTable,
}
impl Assembler {
    pub fn new(path: &str) -> Self {
        let lines = Lines::new(path);
        let mut symbol_table = SymbolTable::new();

        let mut row = 0;
        let mut count = 0;

        for line in &lines.lines {
            match line {
                Line::ACommand(_) => { count += 1;}
                Line::CCommand(_) => { count += 1;}
                Line::LCommand(s) => {
                    for i in count..lines.lines.len() {
                        let command = lines.lines.get(row - count + i);
                        if let Some(n) = command {
                            match n {
                                Line::ACommand(_) => {
                                    symbol_table.add_entry(&s, count);
                                }
                                Line::CCommand(_) => {
                                    symbol_table.add_entry(&s, count);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Line::NotCommand => {}
            }
            row += 1;
        }

        Assembler {
            lines,
            symbol_table,
        }
    }

    pub fn assemble(&mut self) {
        let mut address = 16;
        for line in &self.lines.lines {
            match line {
                Line::ACommand(s) => {
                    let value = s.parse::<usize>();
                    if let Err(_) = value {
                        if !self.symbol_table.contains(&s) {
                            self.symbol_table.add_entry(s, address);
                            address += 1;
                        }
                    }
                }
                Line::CCommand(_) => {}
                Line::LCommand(_) => {}
                Line::NotCommand => {}
            }
        }
    }

    pub fn to_binary(&self) -> String {
        let mut output = String::new();
        let bins = self.lines.to_binary(&self.symbol_table);
        for bin in bins {
            output = output + &bin + &("\n".to_string());
        } 
        output
    }
}
