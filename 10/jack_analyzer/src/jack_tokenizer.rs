use std::str;

use regex::Regex;
#[derive(Debug)]
pub struct JackTokenizer {
    contents: Vec<String>,
    string_consts: Vec<String>,
}
impl JackTokenizer {
    pub fn new(contents: String) -> Self {
        function(contents)
    }

    pub fn print(&self) {
        for word in &self.contents {
            println!("{}", word);
        }
    }

    pub fn toTokens(&self) -> Tokens {
        let mut tokens = Vec::new();

        for word in &self.contents {
            tokens.push(Token::get_token(word));
        }


        Tokens(tokens)
    }
}

pub struct Tokens(Vec<Token>);
enum Token {
    TKeyword(Keyword),
    TSymbol(String),
    TIdentifier(String),
    TIntVal(i16),
    TStringVal(String),
    TOther,
}
impl Token {
    fn get_token(word: &str) -> Self {
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

            "/*STRING_CONST*/" => Token::TStringVal(""),

            word => {
                if let Ok(n) = word.parse() {
                    Token::TIntVal(n)
                } else {
                    if word.len() == 1 {
                        let cs: Vec<char> = word.chars().collect();
                        if cs[0].is_alphanumeric() {
                            Token::TSymbol(cs[0].to_string())
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
}

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

fn function(contents: String) -> JackTokenizer {
    let re_comment_line = Regex::new(r"//.*\n").unwrap();
    let re_comment_block = Regex::new(r"/\*[\s\S]*\*/").unwrap();
    let contents = re_comment_line.replace_all(&contents, "");
    let contents = re_comment_block.replace_all(&contents, "");

    let re_string_const = Regex::new(r#"".*""#).unwrap();
    let string_const_matches = re_string_const.find_iter(&contents);
    let string_consts: Vec<String> = string_const_matches.map(|m| {
        m.as_str().replace("\"", "").to_string()
        }).collect();

    let re_alphanumeric = Regex::new(r"(?P<symbol>[^ 0-9a-zA-z])").unwrap();
    let contents = re_alphanumeric.replace_all(&contents, " $symbol ");

    let contents = re_string_const.replace_all(&contents, " /*STRING_CONST*/ ");
    let contents: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    JackTokenizer{
        contents,
        string_consts,
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
/* comment4
comment5 */
"#
    .to_string();
    let output = function(input);
    output.print();
}