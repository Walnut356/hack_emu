//! jack -> vm code compiler. Divider comments are the lexical elements and program structure of the
//! jack language, mostly for my own sanity during debugging.

use crate::{
    software::{
        compiler_utils::{
            Keyword::{self, *},
            Symbol::*,
            Token,
        },
        writer_impl::Segment,
    },
    utils::get_file_buffers,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
};

use maplit::hashmap;

use super::compiler_utils::Symbol;

#[derive(Debug, Clone)]
pub struct SymbolDef {
    pub segment: Segment,
    pub dtype: Token,
    pub index: usize,
}

impl SymbolDef {
    pub fn new(category: Segment, dtype: Token, index: usize) -> Self {
        Self {
            segment: category,
            dtype,
            index,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    pub cls: HashMap<String, SymbolDef>,
    pub func: HashMap<String, SymbolDef>,
    pub counts: HashMap<Segment, usize>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            cls: HashMap::default(),
            func: HashMap::default(),
            counts: hashmap! {
                Segment::Static => 0,
                Segment::This => 0,
                Segment::Local => 0,
                Segment::Argument => 0,
            },
        }
    }
}

impl SymbolTable {
    pub fn get(&self, name: &str) -> Option<&SymbolDef> {
        let mut result = self.func.get(name);
        if result.is_none() {
            result = self.cls.get(name);
        }

        result
    }

    pub fn has(&self, name: &str) -> bool {
        self.func.get(name).is_some() || self.cls.get(name).is_some()
    }

    pub fn insert(&mut self, name: &str, dtype: Token, segment: Segment) {
        assert!(dtype.is_type(), "Token '{dtype:?}' is not a Data Type");

        let index = *self.counts.get(&segment).unwrap();

        let result = match segment {
            Segment::Local | Segment::Argument => self
                .func
                .insert(name.to_owned(), SymbolDef::new(segment, dtype, index)),

            Segment::This | Segment::Static => self
                .cls
                .insert(name.to_owned(), SymbolDef::new(segment, dtype, index)),
            _ => panic!("Invalid keyword for symbol table: {segment:?}"),
        };

        assert!(result.is_none(), "key '{name}' existed before insert");

        *self.counts.get_mut(&segment).unwrap() += 1;
    }

    pub fn clear(&mut self) {
        *self.counts.get_mut(&Segment::Local).unwrap() = 0;
        *self.counts.get_mut(&Segment::Argument).unwrap() = 0;

        self.func.clear()
    }
}

#[derive(Debug)]
pub struct JackCompiler {
    pub stream: Cursor<String>,
    pub output: BufWriter<File>,

    pub class_name: String,
    pub symbol_table: SymbolTable,
    pub label_count: usize,
}

impl JackCompiler {
    /// Takes a path to a .jack file or a folder containing .jack files, compiles those files into
    /// .vm files, and returns the path to the file(s).
    pub fn compile(path: &Path) -> PathBuf {
        let in_path = PathBuf::from(path);

        let out_dir = if in_path.is_file() {
            Path::new(path.parent().unwrap()).into()
        } else {
            in_path.clone()
        };

        let files = get_file_buffers(path, "jack");

        for (mut file, file_name) in files {
            let mut output_path = out_dir.clone();
            output_path.push(file_name.clone());
            output_path.set_extension("vm");

            let out_file = File::create(output_path).unwrap();
            let output = BufWriter::new(out_file);

            let mut stream = String::new();
            file.read_to_string(&mut stream).unwrap();

            let mut compiler = JackCompiler {
                stream: Cursor::new(stream),
                output,
                class_name: file_name,
                symbol_table: SymbolTable::default(),
                label_count: 0,
            };

            compiler.tokenize();

            compiler.output.flush().unwrap();
        }

        out_dir
    }

    /// Tokenizes the given file buffer. Acts as an entrypoint, looking only for one `class`
    /// declaration, it then proceeds to recursively parse the contents of the class.
    pub fn tokenize(&mut self) {
        // i attempted to parse by .lines() and by .split_whitespace(), but both lacked a bit of
        // granularity that i felt i needed so i don't mind doing it byte-by-byte
        let mut buff = Token::None;
        let mut err_counter = 0;

        // --------------------------------------- 'class' -------------------------------------- //
        while buff != Token::Keyword(Class) {
            buff = self.get_next_token().unwrap();
            err_counter += 1;
            if err_counter >= 10000 {
                // 10k seems like a good enough number of iterations to panic on just in case i mess
                // something up.
                panic!("unable to find identifier 'class'")
            }
        }

        // -------------------------------------- className ------------------------------------- //
        let identifier = self.get_next_token().unwrap();
        assert!(matches!(identifier, Token::Identifier(_)));
        assert_eq!(identifier, Token::Identifier(self.class_name.clone()));

        // ----------------------------------------- '{' ---------------------------------------- //
        let bracket = self.get_next_token().unwrap();
        assert_eq!(bracket, Token::Symbol(BracketOp));

        // ----------------------------- classVarDec* subroutineDec* ---------------------------- //
        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }
            self.keyword_dispatch(token);
        }

        // ----------------------------------------- '}' ---------------------------------------- //
    }

    pub fn keyword_dispatch(&mut self, token: Token) {
        // let output = &mut self.output;
        // let stream = &mut self.stream;
        if let Token::Keyword(keyword) = token {
            match keyword {
                Comment | MComment | APIComment => self.skip_comment(keyword),
                Static | Field | Var => {
                    self.compile_decl(keyword);
                }
                Function => {
                    self.symbol_table.clear();
                    self.compile_func(keyword);
                }
                Method => {
                    self.symbol_table.clear();
                    self.compile_func(keyword);
                }
                Constructor => {
                    self.symbol_table.clear();
                    self.compile_func(keyword);
                }
                Let => {
                    self.compile_let();
                }
                Do => {
                    self.compile_func_call();
                    let semicolon = self.get_next_token().unwrap();
                    assert_eq!(semicolon, Token::Symbol(SemiColon));
                    self.pop_seg(Segment::Temp, 0);
                }
                Return => {
                    self.compile_return();
                }
                If => {
                    self.compile_if();
                }
                While => {
                    self.compile_while();
                }
                Else => panic!("Dangling Else"),
                t => panic!("Invalid statement keyword: {:?}", t),
            }
        } else {
            panic!("Invalid token: {:?}", token);
        }
    }

    pub fn compile_return(&mut self) {
        // -------------------------------------- 'return' -------------------------------------- //

        let (next_token, read_pos) = self.peek_next_token().unwrap();

        if next_token != Token::Symbol(SemiColon) {
            // ----------------------------------- expression? ---------------------------------- //
            self.compile_expression(&Token::Symbol(SemiColon));
        } else {
            self.stream.set_position(read_pos);
            self.push_seg(Segment::Constant, 0);
        }

        self.write_return();
    }

    pub fn compile_decl(&mut self, decl_type: Keyword) {
        // ---------------------------- ('static' | 'field' | 'var') ---------------------------- //

        // ---------------------------------------- type ---------------------------------------- //
        let dtype = self.get_next_token().unwrap();
        assert!(dtype.is_type());

        // ------------------------------- varName (',' varName)* ------------------------------- //
        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(Comma) {
                continue;
            }

            if token == Token::Symbol(SemiColon) {
                break;
            }
            // covers patterns `var int i` and `var int i, j, k;
            assert!(matches!(token, Token::Identifier(_)));

            self.symbol_table
                .insert(&token.to_string(), dtype.clone(), decl_type.into());
        }
    }

    pub fn compile_func(&mut self, func_type: Keyword) {
        // ----------------------- ('constructor' | 'function' | 'method') ---------------------- //
        dbg!(&func_type);

        let is_method = func_type == Method;
        if is_method {
            self.symbol_table.insert(
                "this",
                Token::Identifier(self.class_name.clone()),
                Segment::Argument,
            )
        }
        // ----------------------------------- ('void' | type) ---------------------------------- //
        let ret_type = self.get_next_token().unwrap();
        assert!(ret_type.is_type());

        // ----------------------------------- subroutineName ----------------------------------- //
        let func_name = self.get_next_token().unwrap();
        assert!(func_name.is_identifier());

        dbg!(&func_name);

        // ----------------------------------------- '(' ---------------------------------------- //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));

        // ------------------------------------ parameterList ----------------------------------- //
        // ------------------------ ((type varName) (',' type varName)*)? ----------------------- //
        while let Ok(token) = self.get_next_token() {
            // if the first token is a ParenCl, the paramlist is empty, but still writes tags
            if token == Token::Symbol(ParenCl) {
                break;
            }

            if token == Token::Symbol(Comma) {
                continue;
            }

            assert!(matches!(
                token,
                Token::Identifier(_)
                    | Token::Keyword(Int)
                    | Token::Keyword(Char)
                    | Token::Keyword(Boolean)
            ));

            let name = self.get_next_token().unwrap();
            assert!(matches!(name, Token::Identifier(_)));
            dbg!(&name);
            self.symbol_table
                .insert(&name.to_string(), token, Segment::Argument);
            dbg!(&self.symbol_table.func);
        }

        // ----------------------------------------- ')' ---------------------------------------- //
        // ----------------------------------- subroutineBody ----------------------------------- //
        self.compile_func_body(func_name, func_type);
    }

    pub fn compile_func_body(&mut self, name: Token, func_type: Keyword) {
        // ----------------------------------------- '{' ---------------------------------------- //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));

        // --------------------------------------- varDec* -------------------------------------- //
        while let Ok((next_token, read_pos)) = self.peek_next_token() {
            if next_token != Token::Keyword(Var) {
                break;
            }
            self.stream.set_position(read_pos);
            self.keyword_dispatch(next_token);
        }

        let arg_count = *self.symbol_table.counts.get(&Segment::Local).unwrap();
        self.write_function(&name.to_string(), arg_count);

        if func_type == Method {
            self.push_seg(Segment::Argument, 0);
            self.pop_seg(Segment::Pointer, 0);
        }
        if func_type == Constructor {
            self.push_seg(
                Segment::Constant,
                *self.symbol_table.counts.get(&Segment::This).unwrap(),
            );
            self.write_function_call("Memory", "alloc", 1);
            self.pop_seg(Segment::Pointer, 0)
        }

        // ------------------------------------- statements ------------------------------------- //

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }

            assert!(token.is_statement(), "got: {token:?}");
            self.keyword_dispatch(token);
        }

        // ----------------------------------------- '}' ---------------------------------------- //
    }

    pub fn compile_func_call(&mut self) {
        // ---------------------------------- 'do' or from term --------------------------------- //

        // ----------------------------------- subroutineCall ----------------------------------- //
        let identifier = self.get_next_token().unwrap();
        self.compile_term(&identifier);
    }

    pub fn compile_let(&mut self) {
        // ---------------------------------------- 'let' --------------------------------------- //

        let mut array_handling = false;
        // --------------------------------------- varName -------------------------------------- //
        let id = self.get_next_token().unwrap();
        assert!(matches!(id, Token::Identifier(_)));

        let (next_token, read_pos) = self.peek_next_token().unwrap();

        // ---------------------------------------- ('[' ---------------------------------------- //
        if next_token == Token::Symbol(BraceOp) {
            array_handling = true;
            self.stream.set_position(read_pos);
            // ----------------------------------- expression ----------------------------------- //
            self.compile_expression(&Token::Symbol(BraceCl));
            // -------------------------------------- ']')? ------------------------------------- //
            self.push_name(&id.to_string());
            self.write_operators(&[Token::Symbol(Plus)]);
        }

        // ----------------------------------------- '=' ---------------------------------------- //
        let token = self.get_next_token().unwrap();
        assert_eq!(token, Token::Symbol(Equals));

        // ------------------------------------- expression ------------------------------------- //
        self.compile_expression(&Token::Symbol(SemiColon));

        // ----------------------------------------- ';' ---------------------------------------- //
        let var = self
            .symbol_table
            .get(&id.to_string())
            .unwrap_or_else(|| panic!("Undefined symbol name: {id}"));

        if array_handling {
            self.pop_seg(Segment::Temp, 0);
            self.pop_seg(Segment::Pointer, 1);
            self.push_seg(Segment::Temp, 0);
            self.pop_seg(Segment::That, 0);
        } else {
            self.pop_seg(var.segment, var.index)
        }
    }

    pub fn compile_if(&mut self) {
        // ---------------------------------------- 'if' ---------------------------------------- //

        // ----------------------------------------- '(' ---------------------------------------- //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));

        // ------------------------------------- expression ------------------------------------- //
        self.compile_expression(&Token::Symbol(ParenCl));

        let if_label = self.label_count;
        let else_label = self.label_count + 1;
        self.label_count += 2;

        self.write_not();
        self.write_if(if_label);

        // ----------------------------------------- ')' ---------------------------------------- //

        // ----------------------------------------- '{' ---------------------------------------- //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));

        // ------------------------------------- statements ------------------------------------- //

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }

            assert!(token.is_statement());
            self.keyword_dispatch(token)
        }

        // ----------------------------------------- '}' ---------------------------------------- //

        // --------------------------------------- ('else' -------------------------------------- //
        let (maybe_else, read_pos) = self.peek_next_token().unwrap();

        if maybe_else == Token::Keyword(Else) {
            self.stream.set_position(read_pos);

            // -------------------------------- [rest of else])? -------------------------------- //
            self.write_else(else_label);
            self.write_label(if_label);
            self.compile_else();
            self.write_label(else_label);
        } else {
            self.write_label(if_label);
        }
    }

    pub fn compile_else(&mut self) {
        // --------------------------------------- ('else' -------------------------------------- //

        // ----------------------------------------- '{' ---------------------------------------- //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));

        // ------------------------------------- statements ------------------------------------- //

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }
            self.keyword_dispatch(token)
        }

        // ----------------------------------------- '}' ---------------------------------------- //

        // ----------------------------------------- )? ----------------------------------------- //
    }

    pub fn compile_while(&mut self) {
        // --------------------------------------- 'while' -------------------------------------- //

        // ----------------------------------------- '(' ---------------------------------------- //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));

        // ------------------------------------- expression ------------------------------------- //
        let if_label = self.label_count;
        let else_label = self.label_count + 1;
        self.label_count += 2;

        self.write_label(else_label);
        self.compile_expression(&Token::Symbol(ParenCl));
        self.write_not();
        self.write_if(if_label);

        // ----------------------------------------- '{' ---------------------------------------- //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));

        // ------------------------------------- statements ------------------------------------- //

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }
            self.keyword_dispatch(token)
        }

        // ----------------------------------------- '}' ---------------------------------------- //

        self.write_else(else_label);
        self.write_label(if_label);
    }

    pub fn compile_expression(&mut self, delim: &Token) -> Token {
        /*
        Brain's not working so i'll leave a note for later: i'm going to assume that there's always 1
        term, followed by [something]. [Something] can either be the delimeter or an operator, but the
        the sequence must *start* with a term, and *end* with a term (just before the delimiter). That
        means i can loop through "token" pairs, which should allow me to be a bit more strict with
        asserts and narrow the match statements.
        */

        let token = self.get_next_token().unwrap();
        if token == *delim || token == Token::Symbol(ParenCl) || token == Token::Symbol(BraceCl)
        // || token == Token::Symbol(SemiColon)
        {
            return token;
        }
        // -------------------------------------- term -------------------------------------- //
        assert!(
            matches!(
                token,
                Token::ConstInt(_)
                    | Token::ConstString(_)
                    | Token::Keyword(True)
                    | Token::Keyword(False)
                    | Token::Keyword(Null)
                    | Token::Keyword(This)
                    | Token::Identifier(_)
                    | Token::Symbol(Minus)
                    | Token::Symbol(Tilde)
                    | Token::Symbol(ParenOp)
            ),
            "got {token:?}"
        );

        let mut ops = Vec::new();

        self.compile_term(&token);

        let mut op = self.get_next_token().unwrap();
        // ------------------------------------- (op term)* ------------------------------------- //
        while op.is_operator() {
            ops.push(op);

            let token = self.get_next_token().unwrap();
            assert!(
                matches!(
                    token,
                    Token::ConstInt(_)
                        | Token::ConstString(_)
                        | Token::Keyword(True)
                        | Token::Keyword(False)
                        | Token::Keyword(Null)
                        | Token::Keyword(This)
                        | Token::Identifier(_)
                        | Token::Symbol(Minus)
                        | Token::Symbol(Tilde)
                        | Token::Symbol(ParenOp)
                ),
                "got {token:?}"
            );

            self.compile_term(&token);

            op = self.get_next_token().unwrap();
        }

        self.write_operators(&ops);

        op
    }

    pub fn compile_term(&mut self, token: &Token) {
        let (look_ahead, read_pos) = self.peek_next_token().unwrap();

        match token {
            Token::ConstInt(x) => self.push_seg(Segment::Constant, *x as usize),
            Token::ConstString(x) => {
                self.push_seg(Segment::Constant, x.len());
                self.write_function_call("String", "new", 1);
                for char in x.chars() {
                    self.push_seg(Segment::Constant, char as usize);
                    self.write_function_call("String", "appendChar", 2);
                }
            }
            // ------------------------------- '(' expression ')' ------------------------------- //
            Token::Symbol(ParenOp) => {
                self.compile_expression(&Token::Symbol(ParenCl));
            }
            // --------------------------- varName '[' expression ']' --------------------------- //
            Token::Identifier(x) if look_ahead == Token::Symbol(BraceOp) => {
                self.stream.set_position(read_pos);

                self.compile_expression(&Token::Symbol(BraceCl));
                self.push_name(x);
                self.write_operators(&[Token::Symbol(Symbol::Plus)]);
                self.pop_seg(Segment::Pointer, 1);
                self.push_seg(Segment::That, 0);
            }
            // ----------------------- subroutineName'('expressionList')' ----------------------- //
            Token::Identifier(func_name) if look_ahead == Token::Symbol(ParenOp) => {
                // all non-identifier subroutine names are treated as **method** calls in the
                // current class.
                self.stream.set_position(read_pos);

                let arg_count = self.compile_expr_list();

                self.push_seg(Segment::Pointer, 0);
                self.write_function_call(&self.class_name.clone(), func_name, arg_count + 1);
            }
            // ------------ (className|varName)'.'subroutineName'('expressionList')' ------------ //
            Token::Identifier(x) if look_ahead == Token::Symbol(Period) => {
                self.stream.set_position(read_pos);

                let func_name = self.get_next_token().unwrap();
                assert!(matches!(func_name, Token::Identifier(_)));

                let paren_open = self.get_next_token().unwrap();
                assert_eq!(paren_open, Token::Symbol(ParenOp));

                let mut id_or_type = x.clone();
                let is_method = self.symbol_table.has(x);
                if is_method {
                    self.push_name(x);
                    id_or_type = self.symbol_table.get(x).unwrap().dtype.to_string();
                }

                let arg_count = self.compile_expr_list() + is_method as usize;

                self.write_function_call(&id_or_type, &func_name.to_string(), arg_count);
            }
            // --------------------------------- (unaryOp term) --------------------------------- //
            Token::Symbol(Minus) => {
                self.stream.set_position(read_pos);

                self.compile_term(&look_ahead);
                self.write_negate();
            }
            Token::Symbol(Tilde) => {
                self.stream.set_position(read_pos);

                self.compile_term(&look_ahead);
                self.write_not();
            }
            Token::Identifier(_)
            | Token::Keyword(False)
            | Token::Keyword(True)
            | Token::Keyword(This) => {
                self.push_name(&token.to_string());
            }
            _ => {}
        }
    }

    pub fn compile_expr_list(&mut self) -> usize {
        let mut arg_count = 0;
        let mut delim;

        while let Ok((token, read_pos)) = self.peek_next_token() {
            if token == Token::Symbol(ParenCl) {
                self.stream.set_position(read_pos);
                break;
            }

            arg_count += 1;
            delim = self.compile_expression(&Token::Symbol(Comma));
            if delim == Token::Symbol(ParenCl) {
                break;
            }
        }

        arg_count
    }
}
