use std::collections::HashMap;

pub struct SymbolTable {
    class_table: HashMap<String, JackVariable>,
    subroutine_table: HashMap<String, JackVariable>,
    static_idx: usize,
    field_idx: usize,
    arg_idx: usize,
    var_idx: usize,
}
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            class_table: HashMap::new(),
            subroutine_table: HashMap::new(),
            static_idx: 0,
            field_idx: 0,
            arg_idx: 0,
            var_idx: 0,
        }
    }

    pub fn startSubroutine(&mut self) {
        self.subroutine_table.clear();
        self.arg_idx = 0;
        self.var_idx = 0;
    }

    pub fn define(&mut self, v_name: &str, v_type: &str, v_kind: &VarKind) {
        let name = v_name.to_string();
        let t = v_type.to_string();

        match v_kind {
            VarKind::Static => {
                self.class_table.insert(name, JackVariable {
                    var_type: t,
                    var_kind: VarKind::Static,
                    number: self.static_idx,
                });
                self.static_idx += 1;
            },
            VarKind::Field => {
                self.class_table.insert(name, JackVariable {
                    var_type: t,
                    var_kind: VarKind::Field,
                    number: self.field_idx,
                });
                self.field_idx += 1;
            }
            VarKind::Argument => {
                self.subroutine_table.insert(name, JackVariable {
                    var_type: t,
                    var_kind: VarKind::Argument,
                    number: self.arg_idx,
                });
                self.arg_idx += 1;
            }
            VarKind::Var => {
                self.subroutine_table.insert(name, JackVariable {
                    var_type: t,
                    var_kind: VarKind::Var,
                    number: self.var_idx,
                });
                self.var_idx += 1;
            }
        }
    }

    pub fn var_count(&self, kind: &VarKind) -> usize {
        match kind {
            VarKind::Static => self.static_idx,
            VarKind::Field => self.field_idx,
            VarKind::Argument => self.arg_idx,
            VarKind::Var => self.var_idx,
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<&VarKind> {
        if let Some(var) = self.get_variable(name) {
            Some(&var.var_kind)
        } else {
            None
        }
    }

    pub fn type_of(&self, name: &str) -> Option<String> {
        if let Some(var) = self.get_variable(name) {
            Some(var.var_type.to_string())
        } else {
            None
        }
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        if let Some(var) = self.get_variable(name) {
            Some(var.number)
        } else {
            None
        }
    }

    fn get_variable(&self, name: &str) -> Option<&JackVariable> {
        if self.class_table.contains_key(name) {
            self.class_table.get(name)
        } else {
            self.subroutine_table.get(name)
        }
    }

    pub fn debug_print_class_table(&self) {
        println!("*** CLASS VARIABLES *** ");
        for class in &self.class_table {
            println!("{:?}", class);
        }
        println!();
    }
    
    pub fn debug_print_subroutine_table(&self) {
        println!("*** SUBROUTINE VARIABLES ***");
        for subroutine in &self.subroutine_table {
            println!("{:?}", subroutine);
        }
        println!();
    }
}

#[derive(Debug)]
struct JackVariable {
    var_type: String,
    var_kind: VarKind,
    number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarKind {
    Static,
    Field,
    Argument,
    Var,
}
impl VarKind {
    pub fn to_string(&self) -> String {
        match self {
            VarKind::Static => "static".to_string(),
            VarKind::Field => "this".to_string(),
            VarKind::Argument => "argument".to_string(),
            VarKind::Var => "local".to_string(),
        }
    }
}

