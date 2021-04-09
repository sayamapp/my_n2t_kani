use std::collections::HashMap;
#[derive(Debug)]
pub struct SymbolTable(HashMap<String, usize>);
impl SymbolTable {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        hashmap.insert("SP".to_string(), 0); 
        hashmap.insert("LCL".to_string(), 1);
        hashmap.insert("ARG".to_string(), 2);
        hashmap.insert("THIS".to_string(), 3);
        hashmap.insert("THAT".to_string(), 4);
        
        for i in 0..=15 {
            let key = format!("R{}", i);
            hashmap.insert(key, i);
        }
        
        hashmap.insert("SCREEN".to_string(), 16384);
        hashmap.insert("KBD".to_string(), 24576);

        SymbolTable(hashmap)
    }

    pub fn add_entry(&mut self, s: &str, n: usize) {
        self.0.insert(s.to_string(), n);
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains_key(s)
    }

    pub fn get_address(&self, s: &str) -> usize {
        *self.0.get(s).unwrap()
    }
}
