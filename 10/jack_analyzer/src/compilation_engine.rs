use std::fmt::format;

use crate::jack_tokenizer::Tokens;
use crate::jack_tokenizer::Token;
use crate::jack_tokenizer::Keyword;

pub struct CompilationEngine;
impl CompilationEngine {
    pub fn compile(tokens: &mut Tokens) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        loop {
            if let Some(token) = tokens.advance() {
                println!("{:?}", token);

                if token == &Token::TKeyword(Keyword::Class) {
                    CompilationEngine::compile_class(tokens, &mut output);
                }
                
            } else {
                break;
            }
        }
        println!("{:?}", &output);
        output
    }

    pub fn compile_class(tokens: &mut Tokens, output: &mut Vec<String>) {
        output.push("<class>".to_string());
        loop {
            if let Some(token) = tokens.advance() {
                match token {
                    Token::TIdentifier(identifier) => {
                        output.push(format!("<identifier>{}</identifier>", identifier));
                    },
                    Token::TSymbol(symbol) => {
                        output.push(format!("<symbol>{}</symbol>", symbol));
                    },
                    Token::TKeyword(keyword) => {
                        match keyword {
                            Keyword::Method => {}
                            Keyword::Function => {
                                CompilationEngine::compile_subroutine("function", tokens, output);
                            }
                            Keyword::Constructor => {}
                            Keyword::Static => {
                                CompilationEngine::compile_class_var_dec("static", tokens, output)
                            }
                            Keyword::Field => {}
                            _ => {},
                        }
                    }
                    _ => {},
                }
                // if let Token::TIdentifier(identifier) = token {
                //     output.push(format!("<identifier>{}</identifier>", identifier));
                // } else if 
            } else {
                break;
            }
        }
        output.push("</class>".to_string());
    }

    pub fn compile_class_var_dec(keyword: &str, tokens: &mut Tokens, output: &mut Vec<String>) {
        output.push("<classVarDec>".to_string());
        loop {
            let token = tokens.advance();
            if let Some(token) = token {
                match token {
                    Token::TKeyword(_) => {output.push(token.get_xml());}
                    Token::TSymbol(symbol) => {
                        output.push(token.get_xml());
                        if symbol == ";" {
                            break;
                        }
                    }
                    Token::TIdentifier(_) => {}
                    Token::TIntVal(_) => {}
                    Token::TStringVal(_) => {}
                    Token::TOther => {}
                }
            }
        }
        output.push("</classVarDec>".to_string());
    }

    pub fn compile_subroutine(keyword: &str, tokens: &mut Tokens, output: &mut Vec<String>) {
        output.push("<subroutineDec>".to_string());
        loop {
            let token = tokens.advance();
            if let Some(token) = token {
                match token {
                    Token::TKeyword(_) => {output.push(token.get_xml()); break;}
                    Token::TSymbol(_) => {}
                    Token::TIdentifier(_) => {}
                    Token::TIntVal(_) => {}
                    Token::TStringVal(_) => {}
                    Token::TOther => {}
                }
            }
        }
    }
}