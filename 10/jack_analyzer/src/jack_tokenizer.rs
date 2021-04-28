use regex::Regex;
#[derive(Debug)]
pub struct JackTokenizer {
    contents: Vec<String>,
    string_consts: Vec<String>,
    idx: usize,
}
impl JackTokenizer {
    pub fn new(contents: String) -> Self {
        tokenize(contents)
    }

    pub fn print(&self) {
        for word in &self.contents {
            println!("{}", word);
        }
    }
}

#[derive(Debug)]
pub struct Tokens(Vec<Token>);
impl Tokens {
    pub fn new(jtn: JackTokenizer) -> Self {
        let mut tokens = Vec::new();
        let mut idx = 0;
        for word in jtn.contents {
            let token = Token::get_token(&word, jtn.string_consts[idx].clone());
            if let Token::TStringVal(_) = token {
                idx = std::cmp::min(idx + 1, jtn.string_consts.len() - 1);
            }
            tokens.push(token);
        }
        Tokens(tokens)
    }
    pub fn get_xml(&self) -> Vec<String> {
        let mut xmls = Vec::new();
        xmls.push("<tokens>".to_string());
        for token in &self.0 {
            xmls.push(token.get_xml());
        }
        xmls.push("</tokens>".to_string());
        xmls
    }
}

#[derive(Debug)]
enum Token {
    TKeyword(Keyword),
    TSymbol(String),
    TIdentifier(String),
    TIntVal(i16),
    TStringVal(String),
    TOther,
}
impl Token {
    fn get_token(word: &str, string_const: String) -> Self {
        match word  {
            "class" => Token::TKeyword(Keyword::Class),
            "method" => Token::TKeyword(Keyword::Method),
            "function" => Token::TKeyword(Keyword::Function),
            "constructor" => Token::TKeyword(Keyword::Constructor),
            "int" => Token::TKeyword(Keyword::Int),
            "boolean" => Token::TKeyword(Keyword::Boolean),
            "char" => Token::TKeyword(Keyword::Char),
            "void" => Token::TKeyword(Keyword::Void),
            "var" => Token::TKeyword(Keyword::Var),
            "static" => Token::TKeyword(Keyword::Static),
            "field" => Token::TKeyword(Keyword::Field),
            "let" => Token::TKeyword(Keyword::Let),
            "do" => Token::TKeyword(Keyword::Do),
            "if" => Token::TKeyword(Keyword::If),
            "else" => Token::TKeyword(Keyword::Else),
            "while" => Token::TKeyword(Keyword::While),
            "return" => Token::TKeyword(Keyword::Return),
            "true" => Token::TKeyword(Keyword::True),
            "false" => Token::TKeyword(Keyword::False),
            "null" => Token::TKeyword(Keyword::Null),
            "this" => Token::TKeyword(Keyword::This),

            "/*STRING_CONST*/" => Token::TStringVal(string_const),

            word => {
                if let Ok(n) = word.parse() {
                    Token::TIntVal(n)
                } else {
                    if word.len() == 1 {
                        let cs: Vec<char> = word.chars().collect();
                        if !cs[0].is_lowercase() & !cs[0].is_uppercase() {
                            if cs[0] == '<' {
                                Token::TSymbol("&lt;".to_string())
                            } else if cs[0] == '>' {
                                Token::TSymbol("&gt;".to_string())
                            } else if cs[0] == '&' {
                                Token::TSymbol("&amp;".to_string())
                            } else {
                                Token::TSymbol(cs[0].to_string())
                            }
                        } else {
                            Token::TIdentifier(word.to_string())
                        }
                    } else {
                        Token::TIdentifier(word.to_string())
                    }
                }
            }
        }
    }

    pub fn get_xml(&self) -> String {
        let s = 
        match self {
            Token::TKeyword(keyword) => {
                format!("<keyword>{}</keyword>", keyword.get_xml())
            }
            Token::TSymbol(symbol) => {
                format!("<symbol> {} </symbol>", symbol)
            }
            Token::TIdentifier(identifier) => {
                format!("<identifier> {} </identifier>", identifier)
            }
            Token::TIntVal(i) => {
                format!("<integerConstant> {} </integerConstant>", i)
            }
            Token::TStringVal(s) => {
                format!("<stringConstant> {} </stringConstant>", s)
            }
            Token::TOther => {format!("<error> ***ERROR*** </error>")}
        };
        s
    }
}

#[derive(Debug)]
enum Keyword {
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
        let keyword = 
            match self {
                Keyword::Class => {"class"}
                Keyword::Method => {"method"}
                Keyword::Function => {"function"}
                Keyword::Constructor => {"constructor"}
                Keyword::Int => {"int"}
                Keyword::Boolean => {"boolean"}
                Keyword::Char => {"char"}
                Keyword::Void => {"void"}
                Keyword::Var => {"var"}
                Keyword::Static => {"static"}
                Keyword::Field => {"field"}
                Keyword::Let => {"let"}
                Keyword::Do => {"do"}
                Keyword::If => {"if"}
                Keyword::Else => {"else"}
                Keyword::While => {"while"}
                Keyword::Return => {"return"}
                Keyword::True => {"true"}
                Keyword::False => {"false"}
                Keyword::Null => {"null"}
                Keyword::This => {"this"}
            };
        let ret = format!(" {} ", keyword);    
        ret 
    }
}

fn tokenize(contents: String) -> JackTokenizer {
    let re_comment_line = Regex::new(r"//.*\n").unwrap();
    let re_comment_block = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let contents = re_comment_line.replace_all(&contents, "");
    let contents = re_comment_block.replace_all(&contents, "");

    let re_string_const = Regex::new(r#"".*""#).unwrap();
    let string_const_matches = re_string_const.find_iter(&contents);
    let mut string_consts: Vec<String> = string_const_matches.map(|m| {
        m.as_str().replace("\"", "").to_string()
        }).collect();

    let re_alphanumeric = Regex::new(r"(?P<symbol>[^ 0-9a-zA-Z_])").unwrap();
    let contents = re_alphanumeric.replace_all(&contents, " $symbol ");

    let contents = re_string_const.replace_all(&contents, " /*STRING_CONST*/ ");
    let contents: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    if string_consts.len() == 0 {
        string_consts.push("".to_string());
    }

    println!("{:?}", contents);

    JackTokenizer{
        contents,
        string_consts,
        idx: 0,
    }
}

#[test]
fn test() {
    let input = 
r#"
// comment1
let c = "string_dayo";
let x = (A+B)+C;
let y = ( a * b )* (c + d) - e;
class Class {
    field aaa;
}
// comment2
not_comment // comment 
not_comment / not_comment
let x = "string literal dayo -  "
/* comment3 */
bbbbb
/* comment4
comment5 */
"#
    .to_string();
    let output = tokenize(input);
    let output = Tokens::new(output);
    let output = output.get_xml();
    for o in output {
        println!("{}", o);
    }
    assert!(!'&'.is_alphanumeric());
    assert!(!'*'.is_alphabetic());
    assert!('a'.is_alphabetic());
    assert!('a'.is_alphanumeric());
}