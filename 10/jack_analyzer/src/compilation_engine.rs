use std::{char::ToLowercase, env::var, fmt::format, io::SeekFrom};

use crate::jack_tokenizer::Keyword;
use crate::jack_tokenizer::Token;
use crate::jack_tokenizer::Tokens;

pub struct CompilationEngine {
    xml: Vec<String>,
    depth: usize,
}
impl CompilationEngine {
    pub fn new() -> Self {
        let xml: Vec<String> = Vec::new();
        CompilationEngine { xml, depth: 0 }
    }

    pub fn compile(&mut self, tokens: &mut Tokens) -> Vec<String> {
        if let Some(token) = tokens.peek() {
            if token == &Token::TKeyword(Keyword::Class) {
                self.compile_class(tokens);
            } else {
                panic!("class not found");
            }
        }
        self.xml.clone()
    }

    fn push_tag(&mut self, tag: &str) {
        let line = format!("{}{}", self.get_tab(), tag);
        self.xml.push(line);
    }
    fn get_tab(&self) -> String {
        let mut indent = String::new();
        for _ in 0..self.depth {
            indent += "  ";
        }
        indent
    }
    fn inc_tab(&mut self) {
        self.depth += 1;
    }
    fn dec_tab(&mut self) {
        self.depth -= 1;
    }

    fn compile_class(&mut self, tokens: &mut Tokens) {
        self.push_tag("<class>");
        self.inc_tab();
        let statement = tokens.advance().unwrap();
        let class_name = tokens.advance().unwrap();
        let open_bracket = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());
        self.push_tag(&class_name.get_xml());
        self.push_tag(&open_bracket.get_xml());

        // class var dec
        loop {
            if let Some(token) = tokens.peek() {
                match &token {
                    Token::TKeyword(keyword) => match keyword {
                        Keyword::Method | Keyword::Function | Keyword::Constructor => {
                            self.compile_subroutine(tokens);
                        }
                        Keyword::Static | Keyword::Field => {
                            self.compile_class_var_dec(tokens);
                        }
                        _ => {
                            panic!("ERROR: compile_class 1");
                        }
                    },
                    // close bracket
                    Token::TSymbol(symbol) if symbol == "}" => {
                        self.push_tag(&tokens.advance().unwrap().get_xml());
                        break;
                    }
                    _ => {
                        panic!("ERROR: compile_class 2");
                    }
                }
            }
        }

        self.dec_tab();
        self.push_tag("</class>\n");
    }

    fn compile_class_var_dec(&mut self, tokens: &mut Tokens) {
        self.push_tag("<classVarDec>");
        self.inc_tab();
        loop {
            if let Some(token) = tokens.advance() {
                self.push_tag(&token.get_xml());

                if token == Token::TSymbol(";".to_string()) {
                    break;
                }
            }
        }
        self.dec_tab();
        self.push_tag("</classVarDec>");
    }

    fn compile_subroutine(&mut self, tokens: &mut Tokens) {
        self.push_tag("<subroutineDec>");
        self.inc_tab();

        let statement = tokens.advance().unwrap();
        let return_type = tokens.advance().unwrap();
        let subroutine_name = tokens.advance().unwrap();
        let open_paren = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());
        self.push_tag(&return_type.get_xml());
        self.push_tag(&subroutine_name.get_xml());
        self.push_tag(&open_paren.get_xml());

        self.compile_parameter_list(tokens);

        let end_paren = tokens.advance().unwrap();
        self.push_tag(&end_paren.get_xml());

        // subroutine BODY
        self.push_tag("<subroutineBody>");
        self.inc_tab();
        let open_bracket = tokens.advance().unwrap();
        self.push_tag(&open_bracket.get_xml());

        loop {
            if let Some(p_token) = tokens.peek() {
                match p_token {
                    Token::TSymbol(s) if s == "}" => {
                        break;
                    }
                    Token::TKeyword(key) => match key {
                        Keyword::Var => {
                            self.compile_var_dec(tokens);
                        }
                        Keyword::Let
                        | Keyword::Do
                        | Keyword::If
                        | Keyword::While
                        | Keyword::Return => {
                            self.compile_statements(tokens);
                        }
                        _ => {
                            panic!("ERROR: compile_subroutine body 1");
                        }
                    },
                    _ => {
                        panic!("ERROR: compile_subroutine body 2");
                    }
                }
            }
        }

        let close_bracket = tokens.advance().unwrap();
        self.push_tag(&close_bracket.get_xml());
        self.dec_tab();
        self.push_tag("</subroutineBody>");
        self.dec_tab();
        self.push_tag("</subroutineDec>");
    }

    fn compile_parameter_list(&mut self, tokens: &mut Tokens) {
        self.push_tag("<parameterList>");
        self.inc_tab();
        loop {
            if let Some(p_token) = tokens.peek() {
                if p_token == &Token::TSymbol(")".to_string()) {
                    break;
                }
            }
            if let Some(token) = tokens.advance() {
                self.push_tag(&token.get_xml());
            }
        }
        self.dec_tab();
        self.push_tag("</parameterList>");
    }

    fn compile_var_dec(&mut self, tokens: &mut Tokens) {
        self.push_tag("<varDec>");
        self.inc_tab();
        loop {
            if let Some(token) = tokens.advance() {
                self.push_tag(&token.get_xml());
                if token == Token::TSymbol(";".to_string()) {
                    break;
                }
            }
        }

        self.dec_tab();
        self.push_tag("</varDec>");
    }

    fn compile_statements(&mut self, tokens: &mut Tokens) {
        self.push_tag("<statements>");
        self.inc_tab();

        loop {
            if let Some(p_token) = tokens.peek() {
                match p_token {
                    Token::TKeyword(key) => match key {
                        Keyword::Let => {
                            self.compile_let(tokens);
                        }
                        Keyword::Do => {
                            self.compile_do(tokens);
                        }
                        Keyword::If => {
                            self.compile_if(tokens);
                        }
                        Keyword::While => {
                            self.compile_while(tokens);
                        }
                        Keyword::Return => {
                            self.compile_return(tokens);
                        }
                        _ => {
                            panic!("ERROR: compile statements 1");
                        }
                    },
                    Token::TSymbol(s) if s == "}" => {
                        break;
                    }
                    _ => {
                        println!("{:?}", p_token);
                        tokens.advance();
                        // panic!("ERROR: compile statements 2");
                    }
                }
            }
        }

        self.dec_tab();
        self.push_tag("</statements>");
    }

    fn compile_let(&mut self, tokens: &mut Tokens) {
        self.push_tag("<letStatement>");
        self.inc_tab();
        let statement = tokens.advance().unwrap();
        let var_name = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());
        self.push_tag(&var_name.get_xml());

        if let Some(p_token) = tokens.peek() {
            if p_token == &Token::TSymbol("[".to_string()) {
                let open_sqbracket = tokens.advance().unwrap();
                self.push_tag(&open_sqbracket.get_xml());

                self.compile_expression(true,"]", tokens);

                let close_sqbracket = tokens.advance().unwrap();
                self.push_tag(&close_sqbracket.get_xml());
            }
        }

        let eq = tokens.advance().unwrap();
        self.push_tag(&eq.get_xml());

        self.compile_expression(true, ";", tokens);

        let end_line = tokens.advance().unwrap();
        self.push_tag(&end_line.get_xml());

        self.dec_tab();
        self.push_tag("</letStatement>");
    }

    fn compile_do(&mut self, tokens: &mut Tokens) {
        self.push_tag("<doStatement>");
        self.inc_tab();

        let statement = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());

        self.subroutine_call(tokens);

        let end_line = tokens.advance().unwrap();
        self.push_tag(&end_line.get_xml());

        self.dec_tab();
        self.push_tag("</doStatement>");
    }

    fn compile_if(&mut self, tokens: &mut Tokens) {
        self.push_tag("<ifStatement>");
        self.inc_tab();

        let statement = tokens.advance().unwrap();
        let open_paren = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());
        self.push_tag(&open_paren.get_xml());

        self.compile_expression(true, ")", tokens);

        let close_paren = tokens.advance().unwrap();
        let open_bracket = tokens.advance().unwrap();
        self.push_tag(&close_paren.get_xml());
        self.push_tag(&open_bracket.get_xml());

        self.compile_statements(tokens);

        let close_bracket = tokens.advance().unwrap();
        self.push_tag(&close_bracket.get_xml());

        if let Some(p_token) = tokens.peek() {
            if p_token == &Token::TKeyword(Keyword::Else) {
                let statement = tokens.advance().unwrap();
                let open_bracket = tokens.advance().unwrap();
                self.push_tag(&statement.get_xml());
                self.push_tag(&open_bracket.get_xml());

                self.compile_statements(tokens);

                let end_bracket = tokens.advance().unwrap();
                self.push_tag(&end_bracket.get_xml());
            }
        }

        self.dec_tab();
        self.push_tag("</ifStatement>");
    }

    fn compile_while(&mut self, tokens: &mut Tokens) {
        self.push_tag("<whileStatement>");
        self.inc_tab();
        let statement = tokens.advance().unwrap();
        let open_paren = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());
        self.push_tag(&open_paren.get_xml());

        self.compile_expression(true, ")", tokens);

        let close_paren = tokens.advance().unwrap();
        let open_bracket = tokens.advance().unwrap();
        self.push_tag(&close_paren.get_xml());
        self.push_tag(&open_bracket.get_xml());

        self.compile_statements(tokens);

        let close_bracket = tokens.advance().unwrap();
        self.push_tag(&close_bracket.get_xml());

        self.dec_tab();
        self.push_tag("</whileStatement>");
    }

    fn compile_return(&mut self, tokens: &mut Tokens) {
        self.push_tag("<returnStatement>");
        self.inc_tab();

        let statement = tokens.advance().unwrap();
        self.push_tag(&statement.get_xml());

        if let Some(p_token) = tokens.peek() {
            if p_token != &Token::TSymbol(";".to_string()) {
                self.compile_expression(true,";", tokens);
            }
        }

        let end_line = tokens.advance().unwrap();
        self.push_tag(&end_line.get_xml());

        self.dec_tab();
        self.push_tag("</returnStatement>");
    }

    fn compile_expression(&mut self, flag: bool, end_symbol: &str, tokens: &mut Tokens) {
        if flag {
            self.push_tag("<expression>");
            self.inc_tab();
        }

        self.compile_term(tokens);

        let ops: Vec<Token> = vec![
            Token::TSymbol("+".to_string()),
            Token::TSymbol("-".to_string()),
            Token::TSymbol("*".to_string()),
            Token::TSymbol("/".to_string()),
            Token::TSymbol("&amp;".to_string()),
            Token::TSymbol("|".to_string()),
            Token::TSymbol("&lt;".to_string()),
            Token::TSymbol("&gt;".to_string()),
            Token::TSymbol("=".to_string()),
        ];

        if let Some(p_token) = tokens.peek() {
            if ops.contains(&p_token) {
                let op = tokens.advance().unwrap();
                self.push_tag(&op.get_xml());
            }
        }
        if let Some(p_token) = tokens.peek() {
            if end_symbol == "FROM_EXP_LIST" && (p_token == &Token::TSymbol(",".to_string()) || p_token == &Token::TSymbol(")".to_string())) {

            } else if p_token == &Token::TSymbol(end_symbol.to_string()) {

            } else {
                self.compile_expression(false, end_symbol, tokens);
            }
        }

        if flag {
            self.dec_tab();
            self.push_tag("</expression>");
        }
    }

    fn compile_expression_list(&mut self, flag: bool, tokens: &mut Tokens) {
        if flag {
            self.push_tag("<expressionList>");
            self.inc_tab();
        }

        if let Some(p_token) = tokens.peek() {
            if p_token != &Token::TSymbol(")".to_string()) {
                self.compile_expression(true, "FROM_EXP_LIST", tokens);

                if let Some(p_token) = tokens.peek() {
                    if p_token == &Token::TSymbol(",".to_string()) {
                        let comma = tokens.advance().unwrap();
                        self.push_tag(&comma.get_xml());
                        self.compile_expression_list(false, tokens);
                    } 
                }
            }
        }

        if flag {
            self.dec_tab();
            self.push_tag("</expressionList>");
        }
    }

    fn subroutine_call(&mut self, tokens: &mut Tokens) {
        if let Some(p_token) = tokens.peek() {
            if p_token != &Token::TSymbol(".".to_string()) {
                let name = tokens.advance().unwrap();
                self.push_tag(&name.get_xml());
            }
        }

        if let Some(p_token) = tokens.peek() {
            if p_token == &Token::TSymbol(".".to_string()) {
                let dot = tokens.advance().unwrap();
                let sub_name = tokens.advance().unwrap();

                self.push_tag(&dot.get_xml());
                self.push_tag(&sub_name.get_xml());
            }
        }

        let open_paren = tokens.advance().unwrap();
        self.push_tag(&open_paren.get_xml());

        self.compile_expression_list(true, tokens);

        let close_paren = tokens.advance().unwrap();
        self.push_tag(&close_paren.get_xml());
    }

    fn compile_term(&mut self, tokens: &mut Tokens) {
        self.push_tag("<term>");
        self.inc_tab();

        if let Some(token) = tokens.advance() {
            match token {
                // 3 keyword constant
                Token::TKeyword(Keyword::True) => {
                    self.push_tag(&token.get_xml());
                }
                Token::TKeyword(Keyword::False) => {
                    self.push_tag(&token.get_xml());
                }
                Token::TKeyword(Keyword::Null) => {
                    self.push_tag(&token.get_xml());
                }
                Token::TKeyword(Keyword::This) => {
                    self.push_tag(&token.get_xml());
                }

                // 8 or 7
                Token::TSymbol(_) => {
                    // 8 unaryOp term
                    if token == Token::TSymbol("-".to_string())
                        || token == Token::TSymbol("~".to_string())
                    {
                        self.push_tag(&token.get_xml());
                        self.compile_term(tokens);
                    }
                    // 7 ( expression )
                    else if token == Token::TSymbol("(".to_string()) {
                        self.push_tag(&token.get_xml());
                        self.compile_expression(true, ")", tokens);
                        let close_paren = tokens.advance().unwrap();
                        self.push_tag(&close_paren.get_xml());
                    }
                }

                // 4 or 5 or 6
                Token::TIdentifier(_) => {
                    if let Some(next_p_token) = tokens.peek() {
                        // 5 varName[ expression ]
                        if next_p_token == &Token::TSymbol("[".to_string()) {
                            self.push_tag(&token.get_xml());
                            let open_sqparen = tokens.advance().unwrap();
                            self.push_tag(&open_sqparen.get_xml());

                            self.compile_expression(true, "]", tokens);

                            let close_sqparen = tokens.advance().unwrap();
                            self.push_tag(&close_sqparen.get_xml());
                        }
                        // 6 subroutineCall
                        else if next_p_token == &Token::TSymbol("(".to_string())
                            || next_p_token == &Token::TSymbol(".".to_string())
                        {
                            self.push_tag(&token.get_xml());
                            self.subroutine_call(tokens);
                        }
                        // 4 varName
                        else {
                            self.push_tag(&token.get_xml());
                        }
                    }
                }

                // 1 integer constant
                Token::TIntVal(_) => {
                    self.push_tag(&token.get_xml());
                }

                // 2 string constant
                Token::TStringVal(_) => {
                    self.push_tag(&token.get_xml());
                }

                _ => {}
            }
        }

        self.dec_tab();
        self.push_tag("</term>");
    }
}
