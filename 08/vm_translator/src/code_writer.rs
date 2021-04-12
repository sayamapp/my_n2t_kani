use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::parser::{Arithmetic, CommandType};

macro_rules! pop {
    () => {
        "@SP\nM=M-1\nA=M"
    };
}

macro_rules! push {
    () => {
        "@SP\nA=M\nM=D\n@SP\nM=M+1"
    };
}

pub struct CodeWriter {
    file_name: String,
    commands: Vec<String>,
}

impl CodeWriter {
    pub fn new(path: &str) -> Self {
        CodeWriter {
            file_name: path.to_string(),
            commands: Vec::new(),
        }
    }

    pub fn write_init(&mut self) {
        self.commands.push("@256\nD=A\n@SP\nM=D".to_string());
        self.write_call("Sys.init".to_string(), 0);
    }

    pub fn write_arithmetic(&mut self, arithmetic: Arithmetic) {
        let n = self.commands.len();
        match arithmetic {
            Arithmetic::Add => self.commands.push(binary_function("D=D+M")),

            // Arithmetic::Sub => self.commands.push(binary_function!("// SUB", "D=M-D")),
            // Arithmetic::And => self.commands.push(binary_function!("// AND", "D=D&M")),
            // Arithmetic::Or  => self.commands.push(binary_function!("// OR", "D=D|M")),
            // Arithmetic::Not => self.commands.push(unary_function!("// NOT", "M=!M")),
            // Arithmetic::Neg => self.commands.push(unary_function!("// NEG", "M=-M")),
            // Arithmetic::Eq =>  self.commands.push(logic_command!("// EQ", "D;JEQ", n)),
            // Arithmetic::Lt =>  self.commands.push(logic_command!("// LT", "D;JLT", n)),
            // Arithmetic::Gt =>  self.commands.push(logic_command!("// GT", "D;JGT", n)),
            Arithmetic::Sub => self.commands.push(binary_function("D=M-D")),
            Arithmetic::And => self.commands.push(binary_function("D=D&M")),
            Arithmetic::Or => self.commands.push(binary_function("D=D|M")),
            Arithmetic::Not => self.commands.push(unary_function("M=!M")),
            Arithmetic::Neg => self.commands.push(unary_function("M=-M")),
            Arithmetic::Eq => self.commands.push(logic_command("D;JEQ", n)),
            Arithmetic::Lt => self.commands.push(logic_command("D;JLT", n)),
            Arithmetic::Gt => self.commands.push(logic_command("D;JGT", n)),
        }
    }

    pub fn write_push_pop(&mut self, command: CommandType) {
        match command {
            CommandType::CPush(segment, index) => {
                let seg: &str = &segment;
                match seg {
                    "constant" => self.commands.push(push("constant", index)),
                    "argument" => self.commands.push(push("@ARG", index)),
                    "local" => self.commands.push(push("@LCL", index)),
                    "that" => self.commands.push(push("@THAT", index)),
                    "this" => self.commands.push(push("@THIS" , index)),
                    "temp" => self.commands.push(push("@TEMP", index)),
                    "pointer" => self.commands.push(push("pointer", index + 3)),
                    "static" => self.commands.push(push("static", index + 16)),
                    // "constant" => self.commands.push(push_constant!(&index)),
                    // "argument" => self.commands.push(push_to!("@ARG", &index)),
                    // "local" => self.commands.push(push_to!("@LCL", &index)),
                    // "that" => self.commands.push(push_to!("@THAT", &index)),
                    // "this" => self.commands.push(push_to!("@THIS", &index)),
                    // "temp" => self.commands.push(push_to!("@TEMP", &index)),

                    // "pointer" => self.commands.push(push_pointer!(index + 3)),
                    // "static" => self.commands.push(push_pointer!(index + 16)),
                    _ => {}
                }
            }
            CommandType::CPop(segment, index) => {
                let seg: &str = &segment;
                match seg {
                    // "local" => self.commands.push(pop_to!("@LCL", &index)),
                    // "argument" => self.commands.push(pop_to!("@ARG", &index)),
                    // "this" => self.commands.push(pop_to!("@THIS", &index)),
                    // "that" => self.commands.push(pop_to!("@THAT", &index)),
                    // "temp" => self.commands.push(pop_to!("@TEMP", &index)),

                    // "pointer" => self.commands.push(pop_pointer!(index + 3)),
                    // "static" => self.commands.push(pop_pointer!(index + 16)),
                    "local" => self.commands.push(pop("@LCL", index)),
                    "argument" => self.commands.push(pop("@ARG", index)),
                    "this" => self.commands.push(pop("@THIS", index)),
                    "that" => self.commands.push(pop("@THAT", index)),
                    "temp" => self.commands.push(pop("@TEMP", index)),
                    "pointer" => self.commands.push(pop("pointer", index + 3)),
                    "static" => self.commands.push(pop("static", index + 16)),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn write_call(&mut self, f: String, n: usize) {
        self.commands.push(call(&f, n, self.commands.len()));
    }

    pub fn write_function(&mut self, f: String, n: usize) {
        self.commands.push(function(f, n))
    }

    pub fn write_return(&mut self) {
        self.commands.push(function_return());
    }

    pub fn write_label(&mut self, label: String) {
        let label = format!("({})", label);
        self.commands.push(label);
    }

    pub fn write_goto(&mut self, label: String) {
        let command = format!("@{}\n0;JMP", label);
        self.commands.push(command);
    }

    pub fn write_if(&mut self, label: String) {
        let label = format!("@{}", label);
        let commands: Vec<&str> = vec![pop!(),"D=M", &label, "D;JNE"];
        let commands = commands.join("\n");
        self.commands.push(commands);
    }

    pub fn close(&self) {
        let mut output = String::new();
        for line in &self.commands {
            output = output + &line + &"\n";
        }
        let mut file = BufWriter::new(File::create(&self.file_name).unwrap());
        write!(file, "{}", output).unwrap();
        file.flush().unwrap();
    }
}

fn binary_function(command: &str) -> String {
    let commands: Vec<&str> = vec![pop!(), "D=M", pop!(), command, push!()];
    commands.join("\n")
}

fn unary_function(command: &str) -> String {
    let commands: Vec<&str> = vec![pop!(), command, "@SP", "M=M+1"];
    commands.join("\n")
}

fn logic_command(dist: &str, n: usize) -> String {
    let at = format!("@TRUE.{}", n);
    let af = format!("@FALSE.{}", n);
    let ae = format!("@END.{}", n);
    let lt = format!("(TRUE.{})", n);
    let lf = format!("(FALSE.{})", n);
    let le = format!("(END.{})", n);

    let commands: Vec<&str> = vec![
        pop!(), "D=M", pop!(), "D=M-D",
        &at, dist, &af, "0;JMP",
        &lt, "@0", "D=!A", &ae, "0;JMP",
        &lf, "@0", "D=A", &le, push!(),
        ];
    commands.join("\n")
}

fn push(dist: &str, n: usize) -> String {
    match dist {
        "pointer" => {
            let idx = format!("@{}", if n == 0 {3} else {4});
            let commands: Vec<&str> = vec![&idx, "D=M", push!()];
            commands.join("\n")
        },
        "constant" => {
            let n = format!("@{}", n);
            let commands: Vec<&str> = vec![&n, "D=A", push!()];
            commands.join("\n")
        },
        dist@ _ => {
            let idx = format!("@{}", n);
            let commands = vec![
                dist, if dist == "@TEMP" {"@5\nD=A"} else {"A=M\nD=A"},
                &idx, "D=D+A", "A=D", "D=M", push!()
            ];
            commands.join("\n")
        }
    }
}

fn pop(dist: &str, n: usize) -> String {
    match dist {
        "pointer" => {
            let idx = if n == 0 {"@3"} else {"@4"};
            let commands: Vec<&str> = vec![pop!(), "D=M", idx, "M=D"];
            commands.join("\n")
        },
        "static" => {
            let idx = format!("@{}", n + 16);
            let commands: Vec<&str> = vec![pop!(), "D=M", &idx, "M=D"];
            commands.join("\n")
        },
        dist@ _ => {
            let idx = format!("@{}", n);
            let commands: Vec<&str> = vec![
                if dist == "@TEMP" {"@5\nD=A"} else {dist},
                "D=M", &idx, "D=D+A", "@SP", "A=M", "M=D",
                pop!(), "D=M", "@SP", "A=M", "A=A+1", "A=M", "M=D",
            ];
            commands.join("\n")
        }
    }
}

fn call(f: &str, n: usize, len: usize) -> String {
    let return_address_a = format!("@return-address.{}", len);
    let return_address_l = format!("(return-address.{})", len);
    let n = format!("@{}", n);
    let f = format!("@{}", f);
    let commands: Vec<&str> = vec![
        &return_address_a, "D=A", push!(),
        "@LCL", "D=M", push!(),
        "@ARG", "D=M", push!(),
        "@THIS", "D=M", push!(),
        "@THAT", "D=M", push!(),
        "@SP", "D=M", 
        &n, "D=D-A", "@5", "D=D-A",
        "@ARG", "M=D", "@SP", "D=M",
        "@LCL", "M=D", 
        &f, "0;JMP", &return_address_l,
    ];
    commands.join("\n")
}

fn function(name: String, locals: usize) -> String {
    let label: &str = &format!("({})", name);
    let mut commands = vec![label];
    for _ in 0..locals {
        commands.push("@SP\nA=M\nM=0\n@SP\nM=M+1")
    }
    commands.join("\n")
}

fn function_return() -> String {
    let commands: Vec<&str> = vec![
        "@LCL", "D=M", "@R13", "M=D",
        "@5", "A=D-A", "D=M", "@R14", "M=D",
        pop!(), "D=M", "@ARG", "A=M", "M=D",
        "@ARG", "D=M+1", "@SP", "M=D",
        "@R13", "D=M", "@1", "D=D-A", "A=D", "D=M", "@THAT", "M=D",
        "@R13", "D=M", "@2", "D=D-A", "A=D", "D=M", "@THIS", "M=D",
        "@R13", "D=M", "@3", "D=D-A", "A=D", "D=M", "@ARG", "M=D",
        "@R13", "D=M", "@4", "D=D-A", "A=D", "D=M", "@LCL", "M=D",
        "@R14", "A=M", "0;JMP",
    ];
    commands.join("\n")
}



// #[macro_export]
// macro_rules! write_call {
//     ($f: expr, $n: expr, $len: expr) => {{
//         let ra = format!("@return-address.{}", $len);
//         let rl = format!("(return-address.{})", $len);
//         let n = format!("@{}", $n);
//         let f = format!("@{}", $f);
//         let commands: Vec<&str> = vec![
//             &ra,
//             "D=A",
//             push!(),
//             "@LCL",
//             "D=M",
//             push!(),
//             "@ARG",
//             "D=M",
//             push!(),
//             "@THIS",
//             "D=M",
//             push!(),
//             "@THAT",
//             "D=M",
//             push!(),
//             "@SP",
//             "D=M",
//             &n,
//             "D=D-A",
//             "@5",
//             "D=D-A",
//             "@ARG",
//             "M=D",
//             "@SP",
//             "D=M",
//             "@LCL",
//             "M=D",
//             &f,
//             "0;JMP",
//             &rl,
//         ];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! pop_to {
//     ($dist: expr, $idx: expr) => {{
//         let index = format!("@{}", $idx);
//         let commands = vec![
//             if $dist == "@TEMP" { "@5\nD=A" } else { $dist },
//             "D=M",
//             &index,
//             "D=D+A",
//             "@SP",
//             "A=M",
//             "M=D",
//             pop!(),
//             "D=M",
//             "@SP",
//             "A=M",
//             "A=A+1",
//             "A=M",
//             "M=D",
//         ];
//         commands.join("\n")
//     }};
// }


// #[macro_export]
// macro_rules! pop_pointer {
//     ($idx: expr) => {{
//         let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
//         let commands = vec![pop!(), "D=M", &index, "M=D"];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! pop_static {
//     ($idx: expr) => {{
//         let index = format!("@{}", *$idx + 16);
//         let commands = vec![pop!(), "D=M", &index, "M=D"];
//         commands.join("\n")
//     }};
// }


// #[macro_export]
// macro_rules! push_constant {
//     ($n: expr) => {{
//         let n: &str = &format!("@{}", $n);
//         let commands = vec![n, "D=A", push!()];
//         commands.join("\n")
//     }};
// }
// #[macro_export]
// macro_rules! push_to {
//     ($dist: expr, $idx: expr) => {{
//         let index = format!("@{}", $idx);
//         let commands = vec![
//             $dist,
//             if $dist == "@TEMP" {
//                 "@5\nD=A"
//             } else {
//                 "A=M\nD=A"
//             },
//             &index,
//             "D=D+A",
//             "A=D",
//             "D=M",
//             push!(),
//         ];
//         commands.join("\n")
//     }};
// }
// #[macro_export]
// macro_rules! push_pointer {
//     ($idx: expr) => {{
//         let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
//         let commands: Vec<&str> = vec![&index, "D=M", push!()];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! logic_command {
//     ($comment: expr, $command: expr, $n: expr) => {{
//         let at = format!("@TRUE.{}", $n);
//         let af = format!("@FALSE.{}", $n);
//         let ae = format!("@END.{}", $n);
//         let lt = format!("(TRUE.{})", $n);
//         let lf = format!("(FALSE.{})", $n);
//         let le = format!("(END.{})", $n);
//         let commands = vec![
//             $comment,
//             pop!(),
//             "D=M",
//             pop!(),
//             "D=M-D",
//             &at,
//             $command,
//             &af,
//             "0;JMP",
//             &lt,
//             "@0",
//             "D=!A",
//             &ae,
//             "0;JMP",
//             &lf,
//             "@0",
//             "D=A",
//             &le,
//             push!(),
//         ];
//         commands.join("\n")
//     }};
// }



// #[macro_export]
// macro_rules! write_if {
//     ($label: expr) => {{
//         let ra = format!("@{}", $label);
//         let command = vec![pop!(), "D=M", &ra, "D;JNE"];
//         command.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! write_call {
//     ($f: expr, $n: expr, $len: expr) => {{
//         let ra = format!("@return-address.{}", $len);
//         let rl = format!("(return-address.{})", $len);
//         let n = format!("@{}", $n);
//         let f = format!("@{}", $f);
//         let commands: Vec<&str> = vec![
//             &ra,
//             "D=A",
//             push!(),
//             "@LCL",
//             "D=M",
//             push!(),
//             "@ARG",
//             "D=M",
//             push!(),
//             "@THIS",
//             "D=M",
//             push!(),
//             "@THAT",
//             "D=M",
//             push!(),
//             "@SP",
//             "D=M",
//             &n,
//             "D=D-A",
//             "@5",
//             "D=D-A",
//             "@ARG",
//             "M=D",
//             "@SP",
//             "D=M",
//             "@LCL",
//             "M=D",
//             &f,
//             "0;JMP",
//             &rl,
//         ];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! function_return {
//     () => {{
//         let commands = vec![
//             "@LCL\nD=M\n@R13\nM=D",
//             "@5\nA=D-A\nD=M\n@R14\nM=D",
//             pop!(),
//             "D=M\n@ARG\nA=M\nM=D",
//             "@ARG\nD=M+1\n@SP\nM=D",
//             "@R13\nD=M\n@1\nD=D-A\nA=D\nD=M\n@THAT\nM=D",
//             "@R13\nD=M\n@2\nD=D-A\nA=D\nD=M\n@THIS\nM=D",
//             "@R13\nD=M\n@3\nD=D-A\nA=D\nD=M\n@ARG\nM=D",
//             "@R13\nD=M\n@4\nD=D-A\nA=D\nD=M\n@LCL\nM=D",
//             "@R14\nA=M\n0;JMP",
//         ];
//         commands.join("\n")
//     }};
// }

// // #[macro_export]
// // macro_rules! push_constant {
// //     ($n: expr) => {{
// //         let n: &str = &format!("@{}", $n);
// //         let commands = vec![n, "D=A", push!()];
// //         commands.join("\n")
// //     }};
// // }

// #[macro_export]
// macro_rules! pop_to {
//     ($dist: expr, $idx: expr) => {{
//         let index = format!("@{}", $idx);
//         let commands = vec![
//             if $dist == "@TEMP" { "@5\nD=A" } else { $dist },
//             "D=M",
//             &index,
//             "D=D+A",
//             "@SP",
//             "A=M",
//             "M=D",
//             pop!(),
//             "D=M",
//             "@SP",
//             "A=M",
//             "A=A+1",
//             "A=M",
//             "M=D",
//         ];
//         commands.join("\n")
//     }};
// }

// // #[macro_export]
// // macro_rules! push_to {
// //     ($dist: expr, $idx: expr) => {{
// //         let index = format!("@{}", $idx);
// //         let commands = vec![
// //             $dist,
// //             if $dist == "@TEMP" {
// //                 "@5\nD=A"
// //             } else {
// //                 "A=M\nD=A"
// //             },
// //             &index,
// //             "D=D+A",
// //             "A=D",
// //             "D=M",
// //             push!(),
// //         ];
// //         commands.join("\n")
// //     }};
// // }

// #[macro_export]
// macro_rules! pop_pointer {
//     ($idx: expr) => {{
//         let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
//         let commands = vec![pop!(), "D=M", &index, "M=D"];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! pop_static {
//     ($idx: expr) => {{
//         let index = format!("@{}", *$idx + 16);
//         let commands = vec![pop!(), "D=M", &index, "M=D"];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! push_static {
//     ($idx: expr) => {{
//         let index = format!("@{}", *$idx + 16);
//         let commands: Vec<&str> = vec![&index, "D=M", push!()];
//         commands.join("\n")
//     }};
// }

// #[macro_export]
// macro_rules! push_pointer {
//     ($idx: expr) => {{
//         let index = format!("@{}", if $idx == 0 { 3 } else { 4 });
//         let commands: Vec<&str> = vec![&index, "D=M", push!()];
//         commands.join("\n")
//     }};
// }

// // #[macro_export]
// // macro_rules! unary_function {
// //     ($comment: expr, $x: expr) => {{
// //         let commands = vec![pop!(), $x, "@SP", "M=M+1"];
// //         commands.join("\n")
// //     }};
// // }

// // #[macro_export]
// // macro_rules! binary_function {
// //     ($comment: expr, $x: expr) => {{
// //         let v = vec![$comment, pop!(), "D=M", pop!(), $x, push!()];
// //         v.join("\n")
// //     }};
// // }

// // #[macro_export]
// // macro_rules! logic_command {
// //     ($comment: expr, $command: expr, $n: expr) => {{
// //         let at = format!("@TRUE.{}", $n);
// //         let af = format!("@FALSE.{}", $n);
// //         let ae = format!("@END.{}", $n);
// //         let lt = format!("(TRUE.{})", $n);
// //         let lf = format!("(FALSE.{})", $n);
// //         let le = format!("(END.{})", $n);
// //         let commands = vec![
// //             $comment,
// //             pop!(),
// //             "D=M",
// //             pop!(),
// //             "D=M-D",
// //             &at,
// //             $command,
// //             &af,
// //             "0;JMP",
// //             &lt,
// //             "@0",
// //             "D=!A",
// //             &ae,
// //             "0;JMP",
// //             &lf,
// //             "@0",
// //             "D=A",
// //             &le,
// //             push!(),
// //         ];
// //         commands.join("\n")
// //     }};
// // }
