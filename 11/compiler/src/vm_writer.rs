use std::fmt::format;

use crate::jack_tokenizer::TokenData;

#[derive(Debug, Clone)]
pub struct VMWriter {
    vm: Vec<String>
}

impl VMWriter {
    pub fn new() -> Self {
        VMWriter {
            vm: Vec::new(),
        }
    }

    pub fn write_push(&mut self, segment: &str, n: u16) {
        let line = format!("push {} {}", segment, n);
        self.vm.push(line);
    }

    pub fn write_pop(&mut self, segment: &str, n: usize) {
        let line = format!("pop {} {}", segment, n);
        self.vm.push(line);
    }

    pub fn write_function(&mut self, class_name: &str, function_name: &str, n: usize) {
        let line = format!("function {}.{} {}", class_name, function_name, n);
        self.vm.push(line);
    }

    pub fn write_call(&mut self, name: &str, n: usize) {
        let line = format!("call {} {}", name, n);
        self.vm.push(line);
    }
 
    pub fn write_arithmetic(&mut self, op: &str) {
        match op {
            "+" => self.vm.push("add".to_string()),
            "-" => self.vm.push("sub".to_string()),
            "*" => self.vm.push("call Math.multiply 2".to_string()),
            "/" => self.vm.push("call Math.divide 2".to_string()),
            "&amp;" => self.vm.push("and".to_string()),
            "|" => self.vm.push("or".to_string()),
            "&lt;" => self.vm.push("lt".to_string()),
            "&gt;" => self.vm.push("gt".to_string()),
            "=" => self.vm.push("eq".to_string()),
            _ => {},
        }
    }

    pub fn write_unary_op(&mut self, op: &str) {
        match op {
            "-" => self.vm.push("neg".to_string()),
            "~" => self.vm.push("not".to_string()),
            _ => {},
        }
    }

    pub fn write_label(&mut self, label: &str) {
        let command = format!("label {}", label);
        self.vm.push(command);
    }

    pub fn write_goto(&mut self, label: &str) {
        let command = format!("goto {}", label);
        self.vm.push(command);
    }

    pub fn write_if(&mut self, label: &str) {
        self.vm.push("not".to_string());
        let command = format!("if-goto {}", label);
        self.vm.push(command);
    }

    pub fn write_return(&mut self) {
        self.vm.push("return".to_string());
    }

    pub fn output(&self) -> Vec<String> {
        self.vm.clone()
    }

    pub fn push(&mut self, str: &str) {
        self.vm.push(str.to_string());
    }

}
