use std::path::PathBuf;

use crate::jack_tokenizer::Keyword;
use crate::jack_tokenizer::TokenData;
use crate::jack_tokenizer::Tokenizer;

pub struct CompilationEngine {
    tokenizer: Tokenizer,
    xml: Vec<String>,
    depth: usize,
}
impl CompilationEngine {
    pub fn new(tokenizer: Tokenizer) -> Self {
        let xml: Vec<String> = Vec::new();
        CompilationEngine {
            tokenizer,
            xml,
            depth: 0,
        }
    }
    
    fn push_xml_this_token(&mut self) {
        self.push_xml(&self.get_xml());
        self.advance();
    }

    fn push_xml(&mut self, str: &str) {
        let mut space = String::new();
        for _ in 0..self.depth {
            space += "  ";
        }
        self.xml.push(format!("{}{}", space, str));
    }
    fn advance(&mut self) {
        self.tokenizer.advance();
    }
    fn get_xml(&self) -> String {
        self.tokenizer.get_xml()
    }
    fn get_token(&self) -> &Option<TokenData> {
        self.tokenizer.get_token()
    }
    fn peek_token(&self) -> Option<TokenData> {
        self.tokenizer.peek_token()
    }
    fn inc_tab(&mut self) {
        self.depth += 1;
    }
    fn dec_tab(&mut self) {
        self.depth -= 1;
    }

    pub fn output_xml(&self) -> Vec<String> {
        self.xml.clone()
    }

    pub fn debug_xml(&self) {
        for xml in &self.xml {
            println!("{}", xml);
        }
    }
    
    pub fn start_compile(&mut self) {
        self.tokenizer.advance();
        self.compile_class();
    }

    fn compile_class(&mut self) {
        self.push_xml("<class>");
        self.inc_tab();
        // class className {
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.push_xml_this_token();
        // classVarDec*
        while self.is_class_var_dec() { self.compile_class_var_dec(); }
        // subroutineDec*
        while self.is_subroutine_dec() {self.compile_subroutine(); }
        // }
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</class>");
        self.push_xml("");
    }

    fn compile_class_var_dec(&mut self) {
        self.push_xml("<classVarDec>");
        self.inc_tab();


        // attribute type varname 
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.push_xml_this_token();

        // (',' type varName)*
        while !self.is_semicolon() {
            self.push_xml_this_token();
        }

        // ;
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</classVarDec>")
    }

    fn compile_var_dec(&mut self) {
        self.push_xml("<varDec>");
        self.inc_tab();

        // var type varname
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.push_xml_this_token();

        while !self.is_semicolon() {
            self.push_xml_this_token();
        }

        // ;
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</varDec>");
    }

    fn compile_subroutine(&mut self) {
        self.push_xml("<subroutineDec>");
        self.inc_tab();

        // attribute type subroutineName
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.push_xml_this_token();

        //( parameterList )
        self.push_xml_this_token();
        self.compile_parameter_list();
        self.push_xml_this_token();

        // *** subroutineBody
        self.push_xml("<subroutineBody>");
        self.inc_tab();

        // {
        self.push_xml_this_token();

        // varDec*
        while self.is_var_dec() { self.compile_var_dec(); }

        // statements
        self.compile_statements();

        // }
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</subroutineBody>");

        self.dec_tab();
        self.push_xml("</subroutineDec>");
    }

    fn compile_parameter_list(&mut self) {
        self.push_xml("<parameterList>");
        self.inc_tab();

        // ((type varname) (',' type varname)*)?
        while !self.is_close_paren() {
            self.push_xml_this_token();
        }

        self.dec_tab();
        self.push_xml("</parameterList>");
    }

    fn compile_statements(&mut self) {
        self.push_xml("<statements>");
        self.inc_tab();

        // statement*
        while self.is_statement() {
            if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
                match keyword {
                    Keyword::Let => { self.compile_let(); }
                    Keyword::Do => { self.compile_do(); }
                    Keyword::If => { self.compile_if(); }
                    Keyword::While => { self.compile_while(); }
                    Keyword::Return => { self.compile_return(); }
                    _ => {}
                }
            }
        }

        self.dec_tab();
        self.push_xml("</statements>");
    }

    fn compile_let(&mut self) {
        self.push_xml("<letStatement>");
        self.inc_tab();

        // let varName
        self.push_xml_this_token();
        self.push_xml_this_token();

        // ('[' expression ']')?
        if self.is_open_sq() {
            self.push_xml_this_token();
            self.compile_expression();
            self.push_xml_this_token();
        }

        // =
        self.push_xml_this_token();

        // expression
        self.compile_expression();

        // ;
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</letStatement>");
    }

    fn compile_expression(&mut self) {
        self.push_xml("<expression>");
        self.inc_tab();

        // term 
        self.compile_term();

        // (op term)*
        while self.is_op() {
            self.push_xml_this_token();
            self.compile_term();
        }

        self.dec_tab();
        self.push_xml("</expression>");
    }

    fn compile_term(&mut self) {
        self.push_xml("<term>");
        self.inc_tab();

        // integerConstant
        if self.is_integer_constant() { 
            self.push_xml_this_token(); 
        }

        // stringConstant
        else if self.is_string_constant() {
            self.push_xml_this_token(); 
        }

        // keywordConstant
        else if self.is_keyword_constant() {
            self.push_xml_this_token(); 
        }

        // unaryOp term
        else if self.is_unary_op() {
            self.push_xml_this_token();
            self.compile_term();
        }

        // '(' expression ')'
        else if self.is_open_paren() {
            self.push_xml_this_token();
            self.compile_expression();
            self.push_xml_this_token();
        }


        else {
            let next_token = self.peek_token().unwrap();

            // varName[ expression ]
            if next_token == TokenData::TSymbol("[".to_string()) {
                self.push_xml_this_token();
                self.push_xml_this_token();
                self.compile_expression();
                self.push_xml_this_token();
            }

            // subroutineCall 1
            // name '.' subroutineName '(' expressionList ')'
            else if next_token == TokenData::TSymbol(".".to_string()) {
                self.push_xml_this_token();
                self.push_xml_this_token();
                self.push_xml_this_token();
                self.push_xml_this_token();
                self.compile_expression_list();
                self.push_xml_this_token();
            }

            // subroutineCall 2 
            // name '(' expressionList ')'
            else if next_token == TokenData::TSymbol("(".to_string()) {
                self.push_xml_this_token();
                self.push_xml_this_token();
                self.compile_expression_list();
                self.push_xml_this_token();
            }

            // varName
            else {
                self.push_xml_this_token();
            }
        }

        self.dec_tab();
        self.push_xml("</term>");
    }

    fn compile_expression_list(&mut self) {
        self.push_xml("<expressionList>");
        self.inc_tab();

        while !self.is_close_paren() {
            if self.is_comma() {
                self.push_xml_this_token();
            }
            self.compile_expression();
        }

        self.dec_tab();
        self.push_xml("</expressionList>");
    }

    fn compile_do(&mut self) {
        self.push_xml("<doStatement>");
        self.inc_tab();

        // do
        self.push_xml_this_token();

        let next_token = self.peek_token();

        // subroutineName '(' expressionList ')'
        if next_token == Some(TokenData::TSymbol("(".to_string())) {
            self.push_xml_this_token();
            self.push_xml_this_token();
            self.compile_expression_list();
            self.push_xml_this_token();
        }

        // name '.' subroutineName '(' expressionList ')'
        else {
            self.push_xml_this_token();
            self.push_xml_this_token();
            self.push_xml_this_token();
            self.push_xml_this_token();
            self.compile_expression_list();
            self.push_xml_this_token();
        }

        // ';'
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</doStatement>");
    }

    fn compile_if(&mut self) {
        self.push_xml("<ifStatement>");
        self.inc_tab();

        // if '(' expression ')' '{' statements '}'
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.compile_expression();
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.compile_statements();
        self.push_xml_this_token();

        // ( else '{' statemetns '}' )?
        if self.is_else() {
            self.push_xml_this_token();
            self.push_xml_this_token();
            self.compile_statements();
            self.push_xml_this_token();
        }

        self.dec_tab();
        self.push_xml("</ifStatement>");
    }

    fn compile_while(&mut self) {
        self.push_xml("<whileStatement>");
        self.inc_tab();

        // while '(' expression ')' '{' statements '}'
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.compile_expression();
        self.push_xml_this_token();
        self.push_xml_this_token();
        self.compile_statements();
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</whileStatement>");
    }
    
    fn compile_return(&mut self) {
        self.push_xml("<returnStatement>");
        self.inc_tab();

        // return
        self.push_xml_this_token();

        // expression?
        while !self.is_semicolon() {
            self.compile_expression();
        }

        // ';'
        self.push_xml_this_token();

        self.dec_tab();
        self.push_xml("</returnStatement>");
    }

    fn is_class_var_dec(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Static))
        || self.get_token() == &Some(TokenData::TKeyword(Keyword::Field))
    }

    fn is_subroutine_dec(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Constructor))
        || self.get_token() == &Some(TokenData::TKeyword(Keyword::Function))
        || self.get_token() == &Some(TokenData::TKeyword(Keyword::Method))
    }

    fn is_statement(&self) -> bool {
        if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
            keyword == &Keyword::Let 
            || keyword == &Keyword::If
            || keyword == &Keyword::While
            || keyword == &Keyword::Do
            || keyword == &Keyword::Return
        } else {
            false
        }
    }

    fn is_semicolon(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(";".to_string()))
    }

    fn is_var_dec(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Var))
    }

    fn is_else(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Else))
    }

    fn is_integer_constant(&self) -> bool {
        if let &Some(TokenData::TIntVal(_)) = self.get_token() {
            true
        } else {
            false
        }
    }

    fn is_string_constant(&self) -> bool {
        if let &Some(TokenData::TStringVal(_)) = self.get_token() {
            true
        } else {
            false
        }
    }

    fn is_keyword_constant(&self) -> bool {
        if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
            keyword == &Keyword::True || keyword == &Keyword::False 
            || keyword == &Keyword::Null || keyword == &Keyword::This
        } else {
            false
        }
    }

    fn is_comma(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(",".to_string()))
    }

    fn is_open_paren(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("(".to_string()))
    }
    fn is_close_paren(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(")".to_string()))
    }

    fn is_open_sq(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("[".to_string()))
    }

    fn is_close_sq(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("]".to_string()))
    }

    fn is_op(&self) -> bool {
        if let &Some(TokenData::TSymbol(symbol)) = &self.get_token() {
            symbol == "+" || symbol == "-" || symbol == "*" || symbol == "/"
            || symbol == "&amp;" || symbol == "|" || symbol == "&lt;" 
            || symbol == "&gt;" || symbol == "="
        } else {
            false
        }
    }

    fn is_unary_op(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("-".to_string()))
        || self.get_token() == &Some(TokenData::TSymbol("~".to_string()))
    }

}



#[test]
fn test() {
    let test_tokenizer = Tokenizer::new(PathBuf::from("./testcase/test/test.jack"));
    let mut compile_engine = CompilationEngine::new(test_tokenizer);

    compile_engine.start_compile();
    println!("//////////////////////");
    println!();
    compile_engine.debug_xml();
    println!();
    println!("//////////////////////");
}

// use std::{char::ToLowercase, env::var, fmt::format, io::SeekFrom};

// use crate::jack_tokenizer::Keyword;
// use crate::jack_tokenizer::Token;
// use crate::jack_tokenizer::Tokens;

// pub struct CompilationEngine {
//     xml: Vec<String>,
//     depth: usize,
// }
// impl CompilationEngine {
//     pub fn new() -> Self {
//         let xml: Vec<String> = Vec::new();
//         CompilationEngine { xml, depth: 0 }
//     }

//     pub fn compile(&mut self, tokens: &mut Tokens) -> Vec<String> {
//         if let Some(token) = tokens.peek() {
//             if token == &Token::TKeyword(Keyword::Class) {
//                 self.compile_class(tokens);
//             } else {
//                 panic!("class not found");
//             }
//         }
//         self.xml.clone()
//     }

//     fn push_tag(&mut self, tag: &str) {
//         let line = format!("{}{}", self.get_tab(), tag);
//         self.xml.push(line);
//     }
//     fn get_tab(&self) -> String {
//         let mut indent = String::new();
//         for _ in 0..self.depth {
//             indent += "  ";
//         }
//         indent
//     }
//     fn inc_tab(&mut self) {
//         self.depth += 1;
//     }
//     fn dec_tab(&mut self) {
//         self.depth -= 1;
//     }

//     fn compile_class(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<class>");
//         self.inc_tab();
//         let statement = tokens.advance().unwrap();
//         let class_name = tokens.advance().unwrap();
//         let open_bracket = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());
//         self.push_tag(&class_name.get_xml());
//         self.push_tag(&open_bracket.get_xml());

//         // class var dec
//         loop {
//             if let Some(token) = tokens.peek() {
//                 match &token {
//                     Token::TKeyword(keyword) => match keyword {
//                         Keyword::Method | Keyword::Function | Keyword::Constructor => {
//                             self.compile_subroutine(tokens);
//                         }
//                         Keyword::Static | Keyword::Field => {
//                             self.compile_class_var_dec(tokens);
//                         }
//                         _ => {
//                             panic!("ERROR: compile_class 1");
//                         }
//                     },
//                     // close bracket
//                     Token::TSymbol(symbol) if symbol == "}" => {
//                         self.push_tag(&tokens.advance().unwrap().get_xml());
//                         break;
//                     }
//                     _ => {
//                         panic!("ERROR: compile_class 2");
//                     }
//                 }
//             }
//         }

//         self.dec_tab();
//         self.push_tag("</class>\n");
//     }

//     fn compile_class_var_dec(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<classVarDec>");
//         self.inc_tab();
//         loop {
//             if let Some(token) = tokens.advance() {
//                 self.push_tag(&token.get_xml());

//                 if token == Token::TSymbol(";".to_string()) {
//                     break;
//                 }
//             }
//         }
//         self.dec_tab();
//         self.push_tag("</classVarDec>");
//     }

//     fn compile_subroutine(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<subroutineDec>");
//         self.inc_tab();

//         let statement = tokens.advance().unwrap();
//         let return_type = tokens.advance().unwrap();
//         let subroutine_name = tokens.advance().unwrap();
//         let open_paren = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());
//         self.push_tag(&return_type.get_xml());
//         self.push_tag(&subroutine_name.get_xml());
//         self.push_tag(&open_paren.get_xml());

//         self.compile_parameter_list(tokens);

//         let end_paren = tokens.advance().unwrap();
//         self.push_tag(&end_paren.get_xml());

//         // subroutine BODY
//         self.push_tag("<subroutineBody>");
//         self.inc_tab();
//         let open_bracket = tokens.advance().unwrap();
//         self.push_tag(&open_bracket.get_xml());

//         loop {
//             if let Some(p_token) = tokens.peek() {
//                 match p_token {
//                     Token::TSymbol(s) if s == "}" => {
//                         break;
//                     }
//                     Token::TKeyword(key) => match key {
//                         Keyword::Var => {
//                             self.compile_var_dec(tokens);
//                         }
//                         Keyword::Let
//                         | Keyword::Do
//                         | Keyword::If
//                         | Keyword::While
//                         | Keyword::Return => {
//                             self.compile_statements(tokens);
//                         }
//                         _ => {
//                             panic!("ERROR: compile_subroutine body 1");
//                         }
//                     },
//                     _ => {
//                         panic!("ERROR: compile_subroutine body 2");
//                     }
//                 }
//             }
//         }

//         let close_bracket = tokens.advance().unwrap();
//         self.push_tag(&close_bracket.get_xml());
//         self.dec_tab();
//         self.push_tag("</subroutineBody>");
//         self.dec_tab();
//         self.push_tag("</subroutineDec>");
//     }

//     fn compile_parameter_list(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<parameterList>");
//         self.inc_tab();
//         loop {
//             if let Some(p_token) = tokens.peek() {
//                 if p_token == &Token::TSymbol(")".to_string()) {
//                     break;
//                 }
//             }
//             if let Some(token) = tokens.advance() {
//                 self.push_tag(&token.get_xml());
//             }
//         }
//         self.dec_tab();
//         self.push_tag("</parameterList>");
//     }

//     fn compile_var_dec(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<varDec>");
//         self.inc_tab();
//         loop {
//             if let Some(token) = tokens.advance() {
//                 self.push_tag(&token.get_xml());
//                 if token == Token::TSymbol(";".to_string()) {
//                     break;
//                 }
//             }
//         }

//         self.dec_tab();
//         self.push_tag("</varDec>");
//     }

//     fn compile_statements(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<statements>");
//         self.inc_tab();

//         loop {
//             if let Some(p_token) = tokens.peek() {
//                 match p_token {
//                     Token::TKeyword(key) => match key {
//                         Keyword::Let => {
//                             self.compile_let(tokens);
//                         }
//                         Keyword::Do => {
//                             self.compile_do(tokens);
//                         }
//                         Keyword::If => {
//                             self.compile_if(tokens);
//                         }
//                         Keyword::While => {
//                             self.compile_while(tokens);
//                         }
//                         Keyword::Return => {
//                             self.compile_return(tokens);
//                         }
//                         _ => {
//                             panic!("ERROR: compile statements 1");
//                         }
//                     },
//                     Token::TSymbol(s) if s == "}" => {
//                         break;
//                     }
//                     _ => {
//                         println!("{:?}", p_token);
//                         tokens.advance();
//                         // panic!("ERROR: compile statements 2");
//                     }
//                 }
//             }
//         }

//         self.dec_tab();
//         self.push_tag("</statements>");
//     }

//     fn compile_let(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<letStatement>");
//         self.inc_tab();
//         let statement = tokens.advance().unwrap();
//         let var_name = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());
//         self.push_tag(&var_name.get_xml());

//         if let Some(p_token) = tokens.peek() {
//             if p_token == &Token::TSymbol("[".to_string()) {
//                 let open_sqbracket = tokens.advance().unwrap();
//                 self.push_tag(&open_sqbracket.get_xml());

//                 self.compile_expression(true,"]", tokens);

//                 let close_sqbracket = tokens.advance().unwrap();
//                 self.push_tag(&close_sqbracket.get_xml());
//             }
//         }

//         let eq = tokens.advance().unwrap();
//         self.push_tag(&eq.get_xml());

//         self.compile_expression(true, ";", tokens);

//         let end_line = tokens.advance().unwrap();
//         self.push_tag(&end_line.get_xml());

//         self.dec_tab();
//         self.push_tag("</letStatement>");
//     }

//     fn compile_do(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<doStatement>");
//         self.inc_tab();

//         let statement = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());

//         self.subroutine_call(tokens);

//         let end_line = tokens.advance().unwrap();
//         self.push_tag(&end_line.get_xml());

//         self.dec_tab();
//         self.push_tag("</doStatement>");
//     }

//     fn compile_if(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<ifStatement>");
//         self.inc_tab();

//         let statement = tokens.advance().unwrap();
//         let open_paren = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());
//         self.push_tag(&open_paren.get_xml());

//         self.compile_expression(true, ")", tokens);

//         let close_paren = tokens.advance().unwrap();
//         let open_bracket = tokens.advance().unwrap();
//         self.push_tag(&close_paren.get_xml());
//         self.push_tag(&open_bracket.get_xml());

//         self.compile_statements(tokens);

//         let close_bracket = tokens.advance().unwrap();
//         self.push_tag(&close_bracket.get_xml());

//         if let Some(p_token) = tokens.peek() {
//             if p_token == &Token::TKeyword(Keyword::Else) {
//                 let statement = tokens.advance().unwrap();
//                 let open_bracket = tokens.advance().unwrap();
//                 self.push_tag(&statement.get_xml());
//                 self.push_tag(&open_bracket.get_xml());

//                 self.compile_statements(tokens);

//                 let end_bracket = tokens.advance().unwrap();
//                 self.push_tag(&end_bracket.get_xml());
//             }
//         }

//         self.dec_tab();
//         self.push_tag("</ifStatement>");
//     }

//     fn compile_while(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<whileStatement>");
//         self.inc_tab();
//         let statement = tokens.advance().unwrap();
//         let open_paren = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());
//         self.push_tag(&open_paren.get_xml());

//         self.compile_expression(true, ")", tokens);

//         let close_paren = tokens.advance().unwrap();
//         let open_bracket = tokens.advance().unwrap();
//         self.push_tag(&close_paren.get_xml());
//         self.push_tag(&open_bracket.get_xml());

//         self.compile_statements(tokens);

//         let close_bracket = tokens.advance().unwrap();
//         self.push_tag(&close_bracket.get_xml());

//         self.dec_tab();
//         self.push_tag("</whileStatement>");
//     }

//     fn compile_return(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<returnStatement>");
//         self.inc_tab();

//         let statement = tokens.advance().unwrap();
//         self.push_tag(&statement.get_xml());

//         if let Some(p_token) = tokens.peek() {
//             if p_token != &Token::TSymbol(";".to_string()) {
//                 self.compile_expression(true,";", tokens);
//             }
//         }

//         let end_line = tokens.advance().unwrap();
//         self.push_tag(&end_line.get_xml());

//         self.dec_tab();
//         self.push_tag("</returnStatement>");
//     }

//     fn compile_expression(&mut self, flag: bool, end_symbol: &str, tokens: &mut Tokens) {
//         if flag {
//             self.push_tag("<expression>");
//             self.inc_tab();
//         }

//         self.compile_term(tokens);

//         let ops: Vec<Token> = vec![
//             Token::TSymbol("+".to_string()),
//             Token::TSymbol("-".to_string()),
//             Token::TSymbol("*".to_string()),
//             Token::TSymbol("/".to_string()),
//             Token::TSymbol("&amp;".to_string()),
//             Token::TSymbol("|".to_string()),
//             Token::TSymbol("&lt;".to_string()),
//             Token::TSymbol("&gt;".to_string()),
//             Token::TSymbol("=".to_string()),
//         ];

//         if let Some(p_token) = tokens.peek() {
//             if ops.contains(&p_token) {
//                 let op = tokens.advance().unwrap();
//                 self.push_tag(&op.get_xml());
//             }
//         }
//         if let Some(p_token) = tokens.peek() {
//             if end_symbol == "FROM_EXP_LIST" && (p_token == &Token::TSymbol(",".to_string()) || p_token == &Token::TSymbol(")".to_string())) {

//             } else if p_token == &Token::TSymbol(end_symbol.to_string()) {

//             } else {
//                 self.compile_expression(false, end_symbol, tokens);
//             }
//         }

//         if flag {
//             self.dec_tab();
//             self.push_tag("</expression>");
//         }
//     }

//     fn compile_expression_list(&mut self, flag: bool, tokens: &mut Tokens) {
//         if flag {
//             self.push_tag("<expressionList>");
//             self.inc_tab();
//         }

//         if let Some(p_token) = tokens.peek() {
//             if p_token != &Token::TSymbol(")".to_string()) {
//                 self.compile_expression(true, "FROM_EXP_LIST", tokens);

//                 if let Some(p_token) = tokens.peek() {
//                     if p_token == &Token::TSymbol(",".to_string()) {
//                         let comma = tokens.advance().unwrap();
//                         self.push_tag(&comma.get_xml());
//                         self.compile_expression_list(false, tokens);
//                     }
//                 }
//             }
//         }

//         if flag {
//             self.dec_tab();
//             self.push_tag("</expressionList>");
//         }
//     }

//     fn subroutine_call(&mut self, tokens: &mut Tokens) {
//         if let Some(p_token) = tokens.peek() {
//             if p_token != &Token::TSymbol(".".to_string()) {
//                 let name = tokens.advance().unwrap();
//                 self.push_tag(&name.get_xml());
//             }
//         }

//         if let Some(p_token) = tokens.peek() {
//             if p_token == &Token::TSymbol(".".to_string()) {
//                 let dot = tokens.advance().unwrap();
//                 let sub_name = tokens.advance().unwrap();

//                 self.push_tag(&dot.get_xml());
//                 self.push_tag(&sub_name.get_xml());
//             }
//         }

//         let open_paren = tokens.advance().unwrap();
//         self.push_tag(&open_paren.get_xml());

//         self.compile_expression_list(true, tokens);

//         let close_paren = tokens.advance().unwrap();
//         self.push_tag(&close_paren.get_xml());
//     }

//     fn compile_term(&mut self, tokens: &mut Tokens) {
//         self.push_tag("<term>");
//         self.inc_tab();

//         if let Some(token) = tokens.advance() {
//             match token {
//                 // 3 keyword constant
//                 Token::TKeyword(Keyword::True) => {
//                     self.push_tag(&token.get_xml());
//                 }
//                 Token::TKeyword(Keyword::False) => {
//                     self.push_tag(&token.get_xml());
//                 }
//                 Token::TKeyword(Keyword::Null) => {
//                     self.push_tag(&token.get_xml());
//                 }
//                 Token::TKeyword(Keyword::This) => {
//                     self.push_tag(&token.get_xml());
//                 }

//                 // 8 or 7
//                 Token::TSymbol(_) => {
//                     // 8 unaryOp term
//                     if token == Token::TSymbol("-".to_string())
//                         || token == Token::TSymbol("~".to_string())
//                     {
//                         self.push_tag(&token.get_xml());
//                         self.compile_term(tokens);
//                     }
//                     // 7 ( expression )
//                     else if token == Token::TSymbol("(".to_string()) {
//                         self.push_tag(&token.get_xml());
//                         self.compile_expression(true, ")", tokens);
//                         let close_paren = tokens.advance().unwrap();
//                         self.push_tag(&close_paren.get_xml());
//                     }
//                 }

//                 // 4 or 5 or 6
//                 Token::TIdentifier(_) => {
//                     if let Some(next_p_token) = tokens.peek() {
//                         // 5 varName[ expression ]
//                         if next_p_token == &Token::TSymbol("[".to_string()) {
//                             self.push_tag(&token.get_xml());
//                             let open_sqparen = tokens.advance().unwrap();
//                             self.push_tag(&open_sqparen.get_xml());

//                             self.compile_expression(true, "]", tokens);

//                             let close_sqparen = tokens.advance().unwrap();
//                             self.push_tag(&close_sqparen.get_xml());
//                         }
//                         // 6 subroutineCall
//                         else if next_p_token == &Token::TSymbol("(".to_string())
//                             || next_p_token == &Token::TSymbol(".".to_string())
//                         {
//                             self.push_tag(&token.get_xml());
//                             self.subroutine_call(tokens);
//                         }
//                         // 4 varName
//                         else {
//                             self.push_tag(&token.get_xml());
//                         }
//                     }
//                 }

//                 // 1 integer constant
//                 Token::TIntVal(_) => {
//                     self.push_tag(&token.get_xml());
//                 }

//                 // 2 string constant
//                 Token::TStringVal(_) => {
//                     self.push_tag(&token.get_xml());
//                 }

//                 _ => {}
//             }
//         }

//         self.dec_tab();
//         self.push_tag("</term>");
//     }
// }
