use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use regex::Regex;


#[derive(Debug)]
pub struct Tokenizer {
    token_data: Option<TokenData>,
    tokens: Vec<String>,
    idx: usize,
}
impl Tokenizer {
    pub fn new(pb: &PathBuf) -> Self {
        let jack = fs::read_to_string(pb).unwrap();
        let jack = remove_comments(jack);
        let (jack, string_consts) = get_string_consts(&jack);

        let mut tokens: Vec<String> = jack.split_whitespace().map(|s| s.to_string()).collect();
        let mut i = 0;
        for token in &mut tokens {
            if token == "/*STRING_CONST*/" {
                let string_const = format!("\"{}\"", string_consts[i].clone());
                *token = string_const;
                i += 1;
            }
        }

        Tokenizer {
            token_data: None,
            tokens,
            idx: 0,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.tokens.len() > self.idx
    }

    pub fn advance(&mut self) {
        if self.has_more_tokens() {
                self.set_token_data();
                self.idx += 1;
        }
    }

    pub fn get_token_debug(&self) -> String {
        format!("{:?}", self.token_data)
    }

    pub fn get_token(&self) -> &Option<TokenData> {
        &self.token_data
    }
    pub fn peek_token(&self) -> Option<TokenData> {
        if self.idx < self.tokens.len() {
            let token_data = TokenData::new(&self.tokens[self.idx]);
            Some(token_data)
        } else {
            None
        }
    }
    
    pub fn get_xml(&self) -> String {
        if let Some(token) = &self.token_data {
            match token {
                TokenData::TKeyword(keyword) => {
                    xml_helper("keyword", &keyword.get_xml())
                },
                TokenData::TSymbol(symbol) => {
                    xml_helper("symbol", &symbol)
                },
                TokenData::TIdentifier(id) => {
                    xml_helper("identifier", &id)
                },
                TokenData::TIntVal(n) => {
                    xml_helper("integerConstant", &n.to_string())
                }
                TokenData::TStringVal(string) => {
                    xml_helper("stringConstant", &string)
                }
            }
        } else {
            "".to_string()
        }
    }



    fn set_token_data(&mut self) {
        let token_data = TokenData::new(&self.tokens[self.idx]);
        self.token_data = Some(token_data);
    }

}

#[derive(Debug, PartialEq)]
pub enum TokenData {
    TKeyword(Keyword),
    TSymbol(String),
    TIdentifier(String),
    TIntVal(u16),
    TStringVal(String),
    // TNotToken,
}
impl TokenData {
    fn new(str: &str) -> Self {
        match str {
            "class" => TokenData::TKeyword(Keyword::Class),
            "method" => TokenData::TKeyword(Keyword::Method),
            "function" => TokenData::TKeyword(Keyword::Function),
            "constructor" => TokenData::TKeyword(Keyword::Constructor),
            "int" => TokenData::TKeyword(Keyword::Int),
            "boolean" => TokenData::TKeyword(Keyword::Boolean),
            "char" => TokenData::TKeyword(Keyword::Char),
            "void" => TokenData::TKeyword(Keyword::Void),
            "var" => TokenData::TKeyword(Keyword::Var),
            "static" => TokenData::TKeyword(Keyword::Static),
            "field" => TokenData::TKeyword(Keyword::Field),
            "let" => TokenData::TKeyword(Keyword::Let),
            "do" => TokenData::TKeyword(Keyword::Do),
            "if" => TokenData::TKeyword(Keyword::If),
            "else" => TokenData::TKeyword(Keyword::Else),
            "while" => TokenData::TKeyword(Keyword::While),
            "return" => TokenData::TKeyword(Keyword::Return),
            "true" => TokenData::TKeyword(Keyword::True),
            "false" => TokenData::TKeyword(Keyword::False),
            "null" => TokenData::TKeyword(Keyword::Null),
            "this" => TokenData::TKeyword(Keyword::This),

            // "/*STRING_CONST*/" => TokenData::TStringVal(str.to_string()),

            str => {
                if let Ok(n) = str.parse::<u16>() {
                    TokenData::TIntVal(n)
                } else if str.len() == 1 {
                    let c = str.chars().into_iter().next().unwrap();
                    match c {
                        c if c.is_alphabetic() => TokenData::TIdentifier(c.to_string()),
                        '<' => TokenData::TSymbol("&lt;".to_string()),
                        '>' => TokenData::TSymbol("&gt;".to_string()),
                        '&' => TokenData::TSymbol("&amp;".to_string()),
                        _ => TokenData::TSymbol(c.to_string()),
                    }
                } else if str.contains("\"") {
                    let str = str[1..(str.len() - 1)].to_string();
                    TokenData::TStringVal(str)
                } else {
                    TokenData::TIdentifier(str.to_string())
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}
impl Keyword {
    fn get_xml(&self) -> String {
        let keyword = match self {
            Keyword::Class => "class",
            Keyword::Method => "method",
            Keyword::Function => "function",
            Keyword::Constructor => "constructor",
            Keyword::Int => "int",
            Keyword::Boolean => "boolean",
            Keyword::Char => "char",
            Keyword::Void => "void",
            Keyword::Var => "var",
            Keyword::Static => "static",
            Keyword::Field => "field",
            Keyword::Let => "let",
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::Return => "return",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
            Keyword::This => "this",
        };
        keyword.to_string()
    }
}



fn remove_comments(contents: String) -> String {
    let re_comment_line = Regex::new(r"//.*\n").unwrap();
    let re_comment_block = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let contents = re_comment_line.replace_all(&contents, "");
    let contents = re_comment_block.replace_all(&contents, "");
    contents.deref().to_string()
}

fn get_string_consts(contents: &str) -> (String, Vec<String>) {
    let re_string_const = Regex::new(r#"".*""#).unwrap();
    let string_const_matches = re_string_const.find_iter(&contents);
    let string_consts: Vec<String> = string_const_matches.map(|m| {
        m.as_str().replace("\"","").to_string()
        }).collect();
    let re_alphanumeric = Regex::new(r"(?P<symbol>[^ 0-9a-zA-Z_])").unwrap();
    let contents = re_alphanumeric.replace_all(&contents, " $symbol ");
    let contents = re_string_const.replace_all(&contents, " /*STRING_CONST*/ ");

    (contents.to_string(), string_consts)
}

fn xml_helper(tag: &str, content: &str) -> String {
    format!("<{}> {} </{}>", tag, content, tag)
}


