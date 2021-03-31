pub fn dest_to_binary(s: &Option<String>) -> String {
    if let Some(s) = s.clone() {
        s.to_uppercase();
        let str = match &*s {
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            _ => "111",
        };
        str.to_string()
    } else {
        "000".to_string()
    }
}

pub fn comp_to_binary(s: &str) -> String {
    let str = match &*s {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" | "1+D" => "0011111",
        "A+1" | "1+A" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" | "A+D" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" | "A&D" => "0000000",
        "D|A" | "A|D" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" | "1+M" => "1110111",
        "M-1" => "1110010",
        "D+M" | "M+D" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" | "M&D" => "1000000",
        "D|M" | "M|D" => "1010101",
        _ => "0000000",
    };
    str.to_string()
}

pub fn jump_to_binary(s: &Option<String>) -> String {
    if let Some(s) = s.clone() {
        s.to_uppercase();
        let str = match &*s {
            "JGT"   => "001",
            "JEQ"   => "010",
            "JGE"   => "011",
            "JLT"   => "100",
            "JNE"   => "101",
            "JLE"   => "110",
            "JMP"   => "111",
            _       => "000",
        };
        str.to_string()
    } else {
        "000".to_string()
    }
}
