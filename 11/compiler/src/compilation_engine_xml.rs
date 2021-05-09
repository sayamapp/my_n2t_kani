use crate::jack_tokenizer::{Keyword, TokenData, Tokenizer};

pub struct CompilationEngineXml {
    tokenizer: Tokenizer,
    xml: Vec<String>,
    depth: usize,
}
impl CompilationEngineXml {
    pub fn new(tokenizer: Tokenizer) -> Self {
        let xml: Vec<String> = Vec::new();
        CompilationEngineXml {
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

        // (',' varName)*
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
