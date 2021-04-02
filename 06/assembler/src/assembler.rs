use crate::parser::Lines;
use crate::symbol_table::SymbolTable;

pub struct Assembler {
    lines: Lines,
    symbol_table: SymbolTable,
}
impl Assembler {
    pub fn new(path: &str) -> Self {
        let lines = Lines::new(path);
        let symbol_table = SymbolTable::new();

        let mut row = 0;
        let mut count = 0;

        Assembler {
            lines,
            symbol_table,
        }
    }
}
