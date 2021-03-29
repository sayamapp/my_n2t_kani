#[derive(Debug)]
pub struct CCommand {
    dest: Option<Dest>,
    comp: Comp,
    jump: Option<Jump>,
}

#[derive(Debug)]
pub enum Dest {
    A, D, M, AD, AM, MD, AMD,
}
impl Dest {
    pub fn to_enum(d: String) -> Self {
        let d = d.to_uppercase();
        let dest: &str = &d;
        match dest {
            "A" => Dest::A,
            "D" => Dest::D,
            "M" => Dest::M,
            "AD" | "DA" => Dest::AD,
            "AM" | "MA" => Dest::AM,
            "MD" | "DM" => Dest::MD,
            _ => Dest::AMD,
        }
    }
    pub fn to_code(&self) -> String {
        let res = match  &self {
            A => "100",
            D => "010",
            M => "001",
            AD => "110",
            AM => "101",
            MD => "011",
            AMD => "111",
        };
        res.to_string()
    }
}

#[derive(Debug)]
pub enum Comp {
    C0, C1, M1, D, A, 
    ND, NA, MD, MA,
    DP1, AP1, DM1, AM1, DPA, DMA, AMD, DANDA, DORA,
    M, NM, MM, MP1, MM1, DPM, DMM, MMD, DANDM, DORM,
}
impl Comp {
    pub fn to_enum(com: String) -> Self {
        let com: &str = &com.to_uppercase();
        match com {
            "0" => Comp::C0,
            "1" => Comp::C1,
            "-1" => Comp::M1,
            "D" => Comp::D,
            "A" => Comp::A,
            "!D" => Comp::ND,
            "!A" => Comp::NA,
            "-D" => Comp::MD,
            "-A" => Comp::MA,
            "D+1" | "1+D" => Comp::DP1,
            "A+1" | "1+A" => Comp::AP1,
            "D-1" => Comp::DM1,
            "A-1" => Comp::AM1,
            "D+A" | "A+D" => Comp::DPA,
            "D-A" => Comp::DMA,
            "A-D" => Comp::AMD,
            "D&A" | "A&D" => Comp::DANDA,
            "D|A" | "A|D" => Comp::DORA,
            "M" => Comp::M,
            "!M" => Comp::NM,
            "-M" => Comp::MM,
            "M+1" | "1+M" => Comp::MP1,
            "M-1" => Comp::MM1,
            "D+M" | "M+D" => Comp::DPM,
            "D-M" => Comp::DMM,
            "M-D" => Comp::MMD,
            "D&M" | "M&D" => Comp::DANDM,
            "D|M" | "M|D" => Comp::DORM,
            _ => Comp::C0,
        }
    }
}

#[derive(Debug)]
pub enum Jump {
    JGT, JGE, JEQ, JLT, JNE, JLE, JMP,
}
impl Jump {
    pub fn to_enum(jump: String) -> Self {
        let jump: &str = &jump.to_uppercase();
        match jump {
            "JGT" => Jump::JGT,
            "JEQ" => Jump::JEQ,
            "JGE" => Jump::JGE,
            "JLT" => Jump::JLE,
            "JNE" => Jump::JNE,
            "JLE" => Jump::JLE,
            _ => Jump::JMP,
        }

    }
}
