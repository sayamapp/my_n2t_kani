use crate::{
    jack_tokenizer::{Keyword, TokenData, Tokenizer},
    symbol_table::{SymbolTable, VarKind},
    vm_writer::VMWriter,
};
pub struct CompilationEngine {
    tokenizer: Tokenizer,
    symbol_table: SymbolTable,
    vm_writer: VMWriter,
    class_name: String,
    label_index: usize,
    is_void: bool,
}

impl CompilationEngine {
    pub fn new(tokenizer: Tokenizer) -> Self {
        CompilationEngine {
            tokenizer: tokenizer,
            symbol_table: SymbolTable::new(),
            vm_writer: VMWriter::new(),
            class_name: String::new(),
            label_index: 0,
            is_void: false,
        }
    }

    pub fn start_compile(&mut self) {
        self.advance();
        self.compile_class();
    }

    fn compile_class(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_CLASS");
        // class className '{'
        self.advance();
        self.set_class_name();
        self.advance();
        self.advance();

        // classVarDec*
        while self.tokenizer.is_class_var_dec() {
            self.compile_class_var_dec();
        }
        // subroutineDec*
        while self.tokenizer.is_subroutine_dec() {
            self.compile_subroutine();
        }
        // '}'
        self.advance();
    }

    fn compile_class_var_dec(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_CLASS_VAR_DEC");
        // attribute type varName
        let v_attribute = self.get_attribute();
        self.advance();
        let v_type = self.get_var_type();
        self.advance();
        let v_name = self.get_identifier();
        self.symbol_table.define(&v_name, &v_type, &v_attribute);
        self.advance();

        // (',' varName)*
        while !self.tokenizer.is_semicolon() {
            self.skip_comma();
            let v_name = self.get_identifier();
            self.symbol_table.define(&v_name, &v_type, &v_attribute);
            self.advance();
        }
        // ';'
        self.advance();
    }

    fn compile_subroutine(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE SUBROUTINE");
        self.symbol_table.startSubroutine();

        // attribute type subroutineName
        let subroutine_attribute = self.get_token().clone();
        self.advance();
        self.set_type();
        self.advance();
        let subroutine_name = self.get_identifier();
        self.advance();

        // methodの場合
        // symbol table の arg 0 は this を設定。 :TODO

        // *** vm_writer
        match subroutine_attribute {
            Some(TokenData::TKeyword(Keyword::Constructor)) => {
                self.symbol_table.startSubroutine();
            }
            Some(TokenData::TKeyword(Keyword::Function)) => {
                self.symbol_table.startSubroutine();

            }
            Some(TokenData::TKeyword(Keyword::Method)) => {
                self.symbol_table.startSubroutine();
                self.symbol_table.define("this", &self.class_name, &VarKind::Argument);
            }
            _ => {}
        }

        // ( parameterList )
        self.advance();
        self.compile_parameter_list();


        self.advance();
        // '{'
        self.advance();

        

        // varDec*
        let mut n_args = 0;
        while self.tokenizer.is_var_dec() {
            n_args += self.compile_var_dec();
        }

        // *** vm_writer
        match subroutine_attribute {
            Some(TokenData::TKeyword(Keyword::Constructor)) => {
                self.vm_writer.write_function(&self.class_name, &subroutine_name, n_args);

                let n_size = self.symbol_table.var_count(&VarKind::Field)
                    + self.symbol_table.var_count(&VarKind::Static);
                self.vm_writer.write_push("constant", n_size as u16);
                self.vm_writer.write_call("Memory.alloc", 1);
                self.vm_writer.write_pop("pointer", 0);
            }
            Some(TokenData::TKeyword(Keyword::Function)) => {
                self.vm_writer.write_function(&self.class_name, &subroutine_name, n_args);
            }
            Some(TokenData::TKeyword(Keyword::Method)) => {
                // メソッドの場合、arg 0 に 何か を設定。
                // push argument 0
                // pop pointer 0 を書く
                self.vm_writer.write_function(&self.class_name, &subroutine_name, n_args);
                self.vm_writer.write_push("argument", 0);
                self.vm_writer.write_pop("pointer", 0);
            }
            _ => {}
        }

        // statements
        self.compile_statements();
        // '}'
        self.advance();

        self.debug_priint_symbol_table();
    }

    fn compile_parameter_list(&mut self) -> usize {
        // ((type varName) (',' type varName)* )?
        let mut n_args = 0;
        while !self.is_close_paren() {
            if self.is_comma() {
                self.advance();
            }

            // type varName
            let v_attribute = VarKind::Argument;
            let v_type = self.get_var_type();
            self.advance();
            let v_name = self.get_identifier();
            self.advance();
            self.symbol_table.define(&v_name, &v_type, &v_attribute);

            n_args += 1;
        }

        n_args
    }

    fn compile_var_dec(&mut self) -> usize {
        self.debug_print_this_token("FUNCTION: COMPILE_VAR_DEC");
        let mut n_arg = 1;
        // var type varName
        let v_attribute = VarKind::Var;
        self.advance();
        let v_type = self.get_var_type();
        self.advance();
        let v_name = self.get_identifier();
        self.advance();

        self.symbol_table.define(&v_name, &v_type, &v_attribute);

        while !self.is_semicolon() {
            if self.is_comma() {
                n_arg += 1;
                self.advance();
            }

            let v_name = self.get_identifier();
            self.advance();

            self.symbol_table.define(&v_name, &v_type, &v_attribute);
        }
        // ';'
        self.advance();

        n_arg
    }

    fn compile_statements(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_STATEMENTS");
        // statements*
        while self.is_statement() {
            if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
                match keyword {
                    Keyword::Let => {
                        self.compile_let();
                    }
                    Keyword::Do => {
                        self.compile_do();
                    }
                    Keyword::If => {
                        self.compile_if();
                    }
                    Keyword::While => {
                        self.compile_while();
                    }
                    Keyword::Return => {
                        self.compile_return();
                    }
                    _ => {
                        self.terminate();
                    }
                }
            }
        }
    }

    fn compile_let(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_LET");

        let mut is_array = false;

        // let varName
        self.advance();
        let v_name = self.get_identifier();
        self.advance();

        // ('[' expression ']' )?
        if self.is_open_sq() {
            is_array = true;
            self.advance();
            self.compile_expression();
            self.advance();

            self.write_push_to_vm(&v_name);
            self.vm_writer.push("add");
            self.vm_writer.push("pop pointer 1");
        }

        // =
        self.advance();

        // expression
        self.compile_expression();


        // ;
        self.advance();

        if is_array {
            self.vm_writer.push("pop that 0");
        } else {
            self.write_pop_to_vm(&v_name);
        }
        self.vm_writer.push("push constant 0");
        self.vm_writer.push("pop pointer 1");
    }

    fn compile_do(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_DO");
        // do
        self.advance();

        // className '.' subroutineName '(' expressionList ')' ';'
        if self.is_class_name() {
            let class_name = self.get_identifier();
            self.advance();
            self.advance();
            let subroutine_name = self.get_identifier();
            self.advance();
            self.advance();

            // self.vm_writer.write_push("pointer", 0);
            let n_args = self.compile_expression_list();

            self.advance();
            self.advance();

            let command = format!("{}.{}", class_name, subroutine_name);
            self.vm_writer.write_call(&command, n_args);
        }
        // varName '.' subroutineName '(' expressionList ')' ';'
        else if self.is_var_name() {
            let var_name = self.get_identifier();
            let v_idx = self.symbol_table.index_of(&var_name).unwrap();
            let v_kind = self.symbol_table.kind_of(&var_name).unwrap().to_string();
            let v_type = self.symbol_table.type_of(&var_name).unwrap();
            self.advance();
            self.advance();
            let subroutine_name = self.get_identifier();
            self.advance();
            self.advance();

            self.vm_writer.write_push(&v_kind, v_idx as u16);

            let n_args = self.compile_expression_list();
            self.advance();
            self.advance();

            let command = format!("{}.{}", &v_type, subroutine_name);
            self.vm_writer.write_call(&command, n_args + 1);
        }

        // subroutineName '(' expressionList ')' ';'
        else {
            let subroutine_name = self.get_identifier();
            self.advance();
            self.advance();
            self.vm_writer.write_push("pointer", 0);
            let n_args = self.compile_expression_list();
            self.advance();
            self.advance();

            let command = format!("{}.{}", self.class_name, subroutine_name);
            self.vm_writer.write_call(&command, n_args + 1);
        }

        // dispose return
        self.vm_writer.write_pop("temp", 0);
    }

    fn compile_if(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_IF");

        let label_1 = self.get_label();
        let label_2 = self.get_label();

        // if '(' expression ')'
        self.advance();
        self.advance();
        self.compile_expression();
        self.advance();

        self.vm_writer.write_if(&label_1);

        // '{' statements '}'
        self.advance();
        self.compile_statements();
        self.advance();

        self.vm_writer.write_goto(&label_2);
        self.vm_writer.write_label(&label_1);

        // ( else '{' statements '}' )?
        if self.is_else() {
            self.advance();
            self.advance();
            self.compile_statements();
            self.advance();
        }

        self.vm_writer.write_label(&label_2);
    }

    fn compile_while(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_WHILE");

        let label_1 = self.get_label();
        let label_2 = self.get_label();

        self.vm_writer.write_label(&label_1);

        // while '(' expression ')'
        self.advance();
        self.advance();
        self.compile_expression();
        self.advance();

        self.vm_writer.write_if(&label_2);

        // '{' statements '}'
        self.advance();
        self.compile_statements();
        self.advance();

        self.vm_writer.write_goto(&label_1);
        self.vm_writer.write_label(&label_2);
    }

    fn compile_return(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_RETURN");

        // return
        self.advance();

        // expression?
        while !self.is_semicolon() {
            self.compile_expression();
        }

        // ';'
        self.advance();

        if self.is_void {
            self.vm_writer.push("push constant 0");
        }

        self.vm_writer.write_return();
    }

    fn compile_expression_list(&mut self) -> usize {
        self.debug_print_this_token("FUNCTION: COMPILE_EXPRESSION_LIST");

        let mut n_args = 0;

        while !self.is_close_paren() {
            if self.is_comma() {
                self.advance();
            }
            self.compile_expression();
            n_args += 1;
        }

        n_args
    }

    fn compile_expression(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_EXPRESSION");
        let mut arithmetic = None;

        // term
        self.compile_term();

        // (op term)*
        while self.is_op() {
            arithmetic = self.get_token().clone();
            self.advance();

            self.compile_term();
        }

        if let Some(TokenData::TSymbol(symbol)) = arithmetic {
            self.vm_writer.write_arithmetic(&symbol);
        }
    }

    fn compile_term(&mut self) {
        self.debug_print_this_token("FUNCTION: COMPILE_TERM");

        // integerConst (FIX)
        if self.is_integer_const() {
            let n = self.get_integer_const();
            self.vm_writer.write_push("constant", n);
            self.advance();
        }
        // stringConst (FIX)
        else if self.is_string_constant() {
            let string_val = self.tokenizer.get_string_val().unwrap();
            let string_len = string_val.len();

            self.vm_writer.write_push("constant", string_len as u16);
            self.vm_writer.write_call("String.new", 1);

            for char in string_val.chars() {
                let b = char as u8;
                self.vm_writer.write_push("constant", b as u16);
                self.vm_writer.write_call("String.appendChar", 2);
            }

            self.advance();
        }
        // keywordConstant (FIX)
        else if self.is_keyword_constant() {
            if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
                match keyword {
                    Keyword::True => {
                        self.vm_writer.write_push("constant", 0);
                        self.vm_writer.push("not");
                    }
                    Keyword::False => {
                        self.vm_writer.write_push("constant", 0);
                    }
                    Keyword::Null => {
                        self.vm_writer.write_push("constant", 0);
                    }
                    Keyword::This => {
                        self.vm_writer.write_push("pointer", 0);
                    }
                    _ => {}
                }
            }
            self.advance();
        }
        // unaryOp term
        else if self.is_unary_op() {
            let unary_op = self.get_token().clone();
            self.advance();
            self.compile_term();

            if let Some(TokenData::TSymbol(op)) = unary_op {
                self.vm_writer.write_unary_op(&op);
            }
        }
        // '(' expression ')'
        else if self.is_open_paren() {
            self.advance();
            self.compile_expression();
            self.advance();


        } else {
            let next_token = self.peek_token().unwrap();

            // varName '[' expression ']'
            if next_token == TokenData::TSymbol("[".to_string()) {
                let var_name = self.get_identifier();
                self.write_push_to_vm(&var_name);
                self.advance();
                self.advance();
                self.compile_expression();
                self.vm_writer.push("add");
                self.vm_writer.write_pop("pointer", 1);
                self.vm_writer.write_push("that", 0);
                self.vm_writer.write_push("constant", 0);
                self.vm_writer.write_pop("pointer", 1);
                self.advance();
            }

            // name '.' subroutineName '(' expressionList ')'
            else if next_token == TokenData::TSymbol(".".to_string()) {
                // className '.' subroutineName '(' expresionList ')'
                if self.is_class_name() {
                    let class_name = self.get_identifier();
                    self.advance();
                    self.advance();

                    let subroutine_name = self.get_identifier();
                    self.advance();
                    self.advance();

                    let n_args = self.compile_expression_list();
                    self.advance();

                    let command = format!("{}.{}", class_name, subroutine_name);
                    self.vm_writer.write_call(&command, n_args);
                }

                // varName '.' subroutineName '(' expressionLIst ')'
                else if self.is_var_name() {
                    let var_name = self.get_identifier();
                    self.advance();
                    self.advance();
                    let subroutine_name = self.get_identifier();
                    self.advance();
                    self.advance();
                    
                    let v_idx = self.symbol_table.index_of(&var_name).unwrap();
                    let v_type = self.symbol_table.type_of(&var_name).unwrap();
                    let v_kind = self.symbol_table.kind_of(&var_name).unwrap().to_string();
                    self.vm_writer.write_push(&v_kind, v_idx as u16);

                    let n_args = self.compile_expression_list();
                    self.advance();

                    let command = format!("{}.{}", v_type, subroutine_name);
                    self.vm_writer.write_call(&command, n_args + 1);
                }

                // subroutineName '(' expressionLIst ')'
                else {
                    let subroutine_name = self.get_identifier();
                    self.advance();
                    self.advance();
                    let n_args = self.compile_expression_list();
                    self.advance();

                    let command = format!("{}.{}", self.class_name, subroutine_name);

                    self.vm_writer.write_push("argument", 0);
                    self.vm_writer.write_call(&command, n_args + 1);
                }
            }
            // varName
            else {
                let v_name = self.get_identifier();
                self.advance();

                self.write_push_to_vm(&v_name);
            }
        }
    }

    // helper functions
    fn advance(&mut self) {
        self.tokenizer.advance();
    }

    fn skip_comma(&mut self) {
        if self.tokenizer.is_comma() {
            self.advance();
        }
    }

    fn terminate(&mut self) {
        while self.tokenizer.has_more_tokens() {
            self.advance();
        }
        println!("*** TERMINATE ***"); // DEBUG
        self.push_vm("*** TERMINATE ***");
    }

    pub fn output_vm(&self) -> Vec<String> {
        self.vm_writer.output()
    }

    // push vm
    fn push_vm(&mut self, str: &str) {
        self.vm_writer.push(str);
    }

    // write vm helper
    fn write_push_to_vm(&mut self, name: &str) {
        let v_attribute = self.symbol_table.kind_of(name).unwrap();
        let v_idx = self.symbol_table.index_of(name).unwrap() as u16;

        match v_attribute {
            VarKind::Static => {
                self.vm_writer.write_push("static", v_idx);
            }
            VarKind::Field => {
                self.vm_writer.write_push("this", v_idx);
            }
            VarKind::Argument => {
                self.vm_writer.write_push("argument", v_idx);
            }
            VarKind::Var => {
                self.vm_writer.write_push("local", v_idx);
            }
        }
    }

    fn write_pop_to_vm(&mut self, name: &str) {
        let v_attribute = self.symbol_table.kind_of(name).unwrap();
        let v_idx = self.symbol_table.index_of(name).unwrap();

        match v_attribute {
            VarKind::Static => {
                self.vm_writer.write_pop("static", v_idx);
            }
            VarKind::Field => {
                self.vm_writer.write_pop("this", v_idx);
            }
            VarKind::Argument => {
                self.vm_writer.write_pop("argument", v_idx);
            }
            VarKind::Var => {
                self.vm_writer.write_pop("local", v_idx);
            }
        }
    }

    // flag check
    fn is_class_name(&self) -> bool {
        if self.tokenizer.next_is_dot() {
            let name = self.get_identifier();
            return !self.symbol_table.contains(&name);
        } 
        false 
    }

    fn is_var_name(&self) -> bool {
        if self.tokenizer.next_is_dot() {
            let name = self.get_identifier();
            return self.symbol_table.contains(&name);
        }
        false
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
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Let))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::Do))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::If))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::While))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::Return))
    }

    fn is_keyword_constant(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::True))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::False))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::Null))
            || self.get_token() == &Some(TokenData::TKeyword(Keyword::This))
    }

    fn is_integer_const(&self) -> bool {
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

    fn is_var_dec(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Var))
    }

    fn is_close_paren(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(")".to_string()))
    }

    fn is_open_paren(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("(".to_string()))
    }

    fn is_open_sq(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("[".to_string()))
    }

    fn is_comma(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(",".to_string()))
    }

    fn is_semicolon(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol(";".to_string()))
    }

    fn is_else(&self) -> bool {
        self.get_token() == &Some(TokenData::TKeyword(Keyword::Else))
    }

    fn is_class_method(&self) -> bool {
        let next_token = self.peek_token();
        next_token == Some(TokenData::TSymbol(".".to_string()))
    }

    fn is_op(&self) -> bool {
        if let &Some(TokenData::TSymbol(symbol)) = &self.get_token() {
            symbol == "+"
                || symbol == "-"
                || symbol == "*"
                || symbol == "/"
                || symbol == "&amp;"
                || symbol == "|"
                || symbol == "&lt;"
                || symbol == "&gt;"
                || symbol == "="
        } else {
            false
        }
    }

    fn is_unary_op(&self) -> bool {
        self.get_token() == &Some(TokenData::TSymbol("-".to_string()))
            || self.get_token() == &Some(TokenData::TSymbol("~".to_string()))
    }

    // get functions
    fn get_attribute(&self) -> VarKind {
        if self.tokenizer.is_class_static() {
            VarKind::Static
        } else if self.tokenizer.is_class_field() {
            VarKind::Field
        } else {
            VarKind::Var
        }
    }

    fn get_token(&self) -> &Option<TokenData> {
        self.tokenizer.get_token()
    }

    fn get_identifier(&self) -> String {
        if let Some(TokenData::TIdentifier(id)) = self.get_token() {
            id.to_string()
        } else {
            panic!("ERROR: CE.get_identifier()");
        }
    }

    fn get_integer_const(&self) -> u16 {
        if let Some(TokenData::TIntVal(n)) = self.get_token() {
            *n
        } else {
            panic!("ERROR: CE.get_integer_const()");
        }
    }

    fn get_var_type(&self) -> String {
        if let Some(token) = self.get_token() {
            match token {
                TokenData::TKeyword(keyword) => match keyword {
                    Keyword::Int => "int".to_string(),
                    Keyword::Boolean => "boolean".to_string(),
                    Keyword::Char => "char".to_string(),
                    _ => {
                        panic!("ERROR: get var name");
                    }
                },
                TokenData::TIdentifier(id) => id.to_string(),
                _ => {
                    panic!("ERROR: get var name");
                }
            }
        } else {
            panic!("ERROR: get var name");
        }
    }

    fn get_label(&mut self) -> String {
        let label = format!("L{}", self.label_index);
        self.label_index += 1;
        label
    }

    fn peek_token(&self) -> Option<TokenData> {
        self.tokenizer.peek_token()
    }

    // set functions
    fn set_class_name(&mut self) {
        self.class_name = self.get_identifier();
    }

    fn set_type(&mut self) {
        if let &Some(TokenData::TKeyword(keyword)) = &self.get_token() {
            self.is_void = keyword == &Keyword::Void;
        } else {
            self.is_void = false;
        }
    }

    // debugger
    fn debug_print_this_token(&self, message: &str) {
        println!("DEBUG TOKEN >> /* {} */ {:?}", message, self.get_token());
    }

    fn debug_priint_symbol_table(&self) {
        self.symbol_table.debug_print_class_table();
        self.symbol_table.debug_print_subroutine_table();
    }
}
