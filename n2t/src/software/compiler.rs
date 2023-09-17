//! jack -> vm code compiler. Divider comments are the lexical elements and program structure of the
//! jack language, mostly for my own sanity during debugging.

use crate::{
    software::compiler_utils::{
        Keyword::*, Symbol::*, Token,
    },
    utils::get_file_buffers,
};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use super::compiler_utils::{DELIM_MAP};
use concat_string::concat_string;

const OPEN: bool = true;
const CLOSE: bool = false;

#[derive(Debug, Default)]
pub struct SymbolTable {}

#[derive(Debug)]
pub struct JackCompiler {
    pub stream: Cursor<String>,
    pub output: BufWriter<File>,

    pub symbol_table: SymbolTable,
    pub indent_depth: usize,
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

        let files = get_file_buffers(&out_dir, "jack");

        for (mut file, file_name) in files {
            let mut output_path = out_dir.clone();
            output_path.push(file_name);
            output_path.set_extension("xml");

            let out_file = File::create(output_path).unwrap();
            let output = BufWriter::new(out_file);

            let mut stream = String::new();
            file.read_to_string(&mut stream).unwrap();

            let mut compiler = JackCompiler {
                stream: Cursor::new(stream),
                output,
                symbol_table: SymbolTable::default(),
                indent_depth: 0,
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

        self.write_xml("class", OPEN);

        // ----------------------------------------- 'class' ---------------------------------------- //
        while buff != Token::Keyword(Class) {
            buff = self.get_next_token().unwrap();
            err_counter += 1;
            if err_counter >= 10000 {
                // 10k seems like a good enough number of iterations to panic on just in case i mess
                // something up.
                panic!("unable to find identifier 'class'")
            }
        }
        self.write_token(&buff);

        // ---------------------------------------- className --------------------------------------- //
        let identifier = self.get_next_token().unwrap();
        assert!(matches!(identifier, Token::Identifier(_)));
        self.write_token(&identifier);

        // ------------------------------------------- '{' ------------------------------------------ //
        let bracket = self.get_next_token().unwrap();
        assert_eq!(bracket, Token::Symbol(BracketOp));
        self.write_token(&bracket);

        // ------------------------------- classVarDec* subroutineDec* ------------------------------ //
        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                self.write_token(&token);
                break;
            }
            self.keyword_dispatch(token);
        }

        // ------------------------------------------- '}' ------------------------------------------ //
        self.write_xml("class", CLOSE);
    }

    pub fn keyword_dispatch(&mut self, token: Token) {
        // let output = &mut self.output;
        // let stream = &mut self.stream;
        if let Token::Keyword(keyword) = token {
            match keyword {
                Comment | MComment | APIComment => self.skip_comment(keyword),
                Static | Field => {
                    self.write_xml("classVarDec", OPEN);
                    self.write_token(&token);

                    self.compile_decl();

                    self.write_xml("classVarDec", CLOSE);
                }
                Var => {
                    self.write_xml("varDec", OPEN);
                    self.write_token(&token);
                    self.compile_decl();
                    self.write_xml("varDec", CLOSE);
                }
                Function | Method | Constructor => {
                    self.write_xml("subroutineDec", OPEN);
                    self.write_token(&token);

                    self.compile_func();

                    self.write_xml("subroutineDec", CLOSE);
                }
                Let => {
                    self.write_xml("letStatement", OPEN);
                    self.write_token(&token);

                    self.compile_let();
                    self.write_xml("letStatement", CLOSE);
                }
                Do => {
                    self.write_xml("doStatement", OPEN);
                    self.write_token(&token);
                    self.compile_func_call();
                    let semicolon = self.get_next_token().unwrap();
                    assert_eq!(semicolon, Token::Symbol(SemiColon));
                    self.write_token(&semicolon);
                    self.write_xml("doStatement", CLOSE);
                }
                Return => {
                    self.write_xml("returnStatement", OPEN);
                    self.write_token(&token);

                    self.compile_return();
                    self.write_xml("returnStatement", CLOSE);
                }
                If => {
                    self.write_xml("ifStatement", OPEN);
                    self.write_token(&token);

                    self.compile_if();
                    self.write_xml("ifStatement", CLOSE);
                }
                While => {
                    self.write_xml("whileStatement", OPEN);
                    self.write_token(&token);

                    self.compile_while();
                    self.write_xml("whileStatement", CLOSE);
                }
                Else => panic!("Dangling Else"),
                t => panic!("Invalid statement keyword: {:?}", t),
            }
        } else {
            panic!("Invalid token: {:?}", token);
        }
    }

    pub fn compile_return(&mut self) {
        // ---------------------------------------- 'return' ---------------------------------------- //

        let (next_token, read_pos) =  self.peek_next_token().unwrap();

        if next_token != Token::Symbol(SemiColon) {
            // ------------------------------------- expression? ------------------------------------ //
            self.compile_expression(&Token::Symbol(SemiColon));
            self.write_token(&Token::Symbol(SemiColon));
        } else {
            self.write_token(&next_token);
            self.stream.set_position(read_pos);
        }
    }

    pub fn compile_decl(&mut self) {
        // ------------------------------ ('static' | 'field' | 'var') ------------------------------ //

        // ------------------------------------------ type ------------------------------------------ //
        let dtype = self.get_next_token().unwrap();
        assert!(dtype.is_type());
        self.write_token(&dtype);

        // --------------------------------- varName (',' varName)* --------------------------------- //
        while let Ok(token) = self.get_next_token() {
            self.write_token(&token);

            if token == Token::Symbol(SemiColon) {
                break;
            }

            // covers patterns `var int i` and `var int i, j, k;
            assert!(matches!(token, Token::Identifier(_) | Token::Symbol(Comma)));
        }
    }

    pub fn compile_func(&mut self) {
        // ------------------------- ('constructor' | 'function' | 'method') ------------------------ //

        // ------------------------------------- ('void' | type) ------------------------------------ //
        let ret_type = self.get_next_token().unwrap();
        assert!(ret_type == Token::Keyword(Void) || ret_type.is_type());
        self.write_token(&ret_type);

        // ------------------------------------- subroutineName ------------------------------------- //
        let func_name = self.get_next_token().unwrap();
        assert!(func_name.is_identifier());
        self.write_token(&func_name);

        // ------------------------------------------- '(' ------------------------------------------ //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));
        self.write_token(&open_paren);

        // -------------------------------------- parameterList ------------------------------------- //
        self.write_xml("parameterList", OPEN);

        // -------------------------- ((type varName) (',' type varName)*)? ------------------------- //
        while let Ok(token) = self.get_next_token() {
            // if the first token is a ParenCl, the paramlist is empty, but still writes tags
            if token == Token::Symbol(ParenCl) {
                break;
            }

            // token is a dtype or a comma
            self.write_token(&token);

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

            // if we're past the conditionals, the token was a dtype, so the next token must be the
            // arg's name
            let name = self.get_next_token().unwrap();
            assert!(matches!(name, Token::Identifier(_)));
            self.write_token(&name);
        }

        // ------------------------------------------- ')' ------------------------------------------ //
        self.write_xml("parameterList", CLOSE);
        self.write_token(&Token::Symbol(ParenCl));

        // ------------------------------------- subroutineBody ------------------------------------- //
        self.compile_func_body();
    }

    pub fn compile_func_body(&mut self) {
        self.write_xml("subroutineBody", OPEN);

        // ------------------------------------------- '{' ------------------------------------------ //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));
        self.write_token(&open_brac);

        // ----------------------------------------- varDec* ---------------------------------------- //
        while let Ok((next_token, read_pos)) =  self.peek_next_token() {
            if next_token != Token::Keyword(Var) {
                break;
            }
            self.stream.set_position(read_pos);
            self.keyword_dispatch(next_token);
        }

        // --------------------------------------- statements --------------------------------------- //
        self.write_xml("statements", OPEN);

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }

            assert!(token.is_statement(), "got: {token:?}");
            self.keyword_dispatch(token);
        }

        // ------------------------------------------- '}' ------------------------------------------ //
        self.write_xml("statements", CLOSE);
        self.write_token(&Token::Symbol(BracketCl));

        self.write_xml("subroutineBody", CLOSE);
    }

    pub fn compile_func_call(&mut self) {
        // ------------------------------------ 'do' or from term ----------------------------------- //

        // ------------------------------------- subroutineCall ------------------------------------- //
        let identifier = self.get_next_token().unwrap();
        self.compile_term(&identifier);
    }

    pub fn compile_let(&mut self) {
        // ------------------------------------------ 'let' ----------------------------------------- //

        // ----------------------------------------- varName ---------------------------------------- //
        let id = self.get_next_token().unwrap();
        assert!(matches!(id, Token::Identifier(_)));
        self.write_token(&id);

        let (next_token, read_pos) =  self.peek_next_token().unwrap();

        // ------------------------------------------ ('[' ------------------------------------------ //
        if next_token == Token::Symbol(BraceOp) {
            self.write_token(&next_token);
            self.stream.set_position(read_pos);

            // ------------------------------------- expression ------------------------------------- //
            self.compile_expression(&Token::Symbol(BraceCl));
            // ---------------------------------------- ']')? --------------------------------------- //
            self.write_token(&Token::Symbol(BraceCl));
        }

        // ------------------------------------------- '=' ------------------------------------------ //
        let token = self.get_next_token().unwrap();
        assert_eq!(token, Token::Symbol(Equals));
        self.write_token(&token);

        // --------------------------------------- expression --------------------------------------- //
        self.compile_expression(&Token::Symbol(SemiColon));

        // ------------------------------------------- ';' ------------------------------------------ //
        self.write_token(&Token::Symbol(SemiColon));
    }

    pub fn compile_if(&mut self) {
        // ------------------------------------------ 'if' ------------------------------------------ //

        // ------------------------------------------- '(' ------------------------------------------ //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));
        self.write_token(&open_paren);

        // --------------------------------------- expression --------------------------------------- //
        self.compile_expression(&Token::Symbol(ParenCl));

        // ------------------------------------------- ')' ------------------------------------------ //
        self.write_token(&Token::Symbol(ParenCl));

        // ------------------------------------------- '{' ------------------------------------------ //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));
        self.write_token(&open_brac);

        // --------------------------------------- statements --------------------------------------- //
        self.write_xml("statements", OPEN);

        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }

            assert!(token.is_statement());
            self.keyword_dispatch(token)
        }

        self.write_xml("statements", CLOSE);

        // ------------------------------------------- '}' ------------------------------------------ //
        self.write_token(&Token::Symbol(BracketCl));

        // ----------------------------------------- ('else' ---------------------------------------- //
        let (maybe_else, read_pos) =  self.peek_next_token().unwrap();

        if maybe_else == Token::Keyword(Else) {
            self.stream.set_position(read_pos);
            self.write_token(&maybe_else);

            // ---------------------------------- [rest of else])? ---------------------------------- //
            self.compile_else()
        }
    }

    pub fn compile_else(&mut self) {
        // ----------------------------------------- ('else' ---------------------------------------- //

        // ------------------------------------------- '{' ------------------------------------------ //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));
        self.write_token(&open_brac);

        // --------------------------------------- statements --------------------------------------- //
        self.write_xml("statements", OPEN);
        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }
            self.keyword_dispatch(token)
        }

        self.write_xml("statements", CLOSE);

        // ------------------------------------------- '}' ------------------------------------------ //
        self.write_token(&Token::Symbol(BracketCl));

        // ------------------------------------------- )? ------------------------------------------- //
    }

    pub fn compile_while(&mut self) {
        // ----------------------------------------- 'while' ---------------------------------------- //

        // ------------------------------------------- '(' ------------------------------------------ //
        let open_paren = self.get_next_token().unwrap();
        assert_eq!(open_paren, Token::Symbol(ParenOp));
        self.write_token(&open_paren);

        // --------------------------------------- expression --------------------------------------- //
        self.compile_expression(&Token::Symbol(ParenCl));

        self.write_token(&Token::Symbol(ParenCl));

        // ------------------------------------------- '{' ------------------------------------------ //
        let open_brac = self.get_next_token().unwrap();
        assert_eq!(open_brac, Token::Symbol(BracketOp));
        self.write_token(&open_brac);

        // --------------------------------------- statements --------------------------------------- //
        self.write_xml("statements", OPEN);
        while let Ok(token) = self.get_next_token() {
            if token == Token::Symbol(BracketCl) {
                break;
            }
            self.keyword_dispatch(token)
        }
        self.write_xml("statements", CLOSE);

        // ------------------------------------------- '}' ------------------------------------------ //
        self.write_token(&Token::Symbol(BracketCl));
    }

    pub fn compile_expression(
        &mut self,
        delim: &Token,
    ) -> Token {
        self.write_xml("expression", OPEN);
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
            self.write_xml("expression", CLOSE);
            return token;
        }
        // ---------------------------------------- term ---------------------------------------- //
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

        self.write_xml("term", OPEN);
        self.compile_term(&token);
        self.write_xml("term", CLOSE);

        let mut op = self.get_next_token().unwrap();
        // --------------------------------------- (op term)* --------------------------------------- //
        while op.is_operator() {
            self.write_token(&op);

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

            self.write_xml("term", OPEN);
            let token = self.get_next_token().unwrap();
            self.compile_term(&token);
            self.write_xml("term", CLOSE);

            op = self.get_next_token().unwrap();
        }

        self.write_xml("expression", CLOSE);
        op
    }

    pub fn compile_term(&mut self, token: &Token) {
        let (look_ahead, read_pos) =  self.peek_next_token().unwrap();

        match token {
            // --------------------------------- '(' expression ')' --------------------------------- //
            Token::Symbol(ParenOp) => {
                self.write_token(token);
                self.compile_expression(&Token::Symbol(ParenCl));
                self.write_token(&Token::Symbol(ParenCl));
            }
            // ----------------------------- varName '[' expression ']' ----------------------------- //
            Token::Identifier(_) if look_ahead == Token::Symbol(BraceOp) => {
                self.write_token(token);
                self.write_token(&look_ahead);
                self.stream.set_position(read_pos);

                self.compile_expression(&Token::Symbol(BraceCl));
                self.write_token(&Token::Symbol(BraceCl));
            }
            // ------------------------- subroutineName'('expressionList')' ------------------------- //
            Token::Identifier(_) if look_ahead == Token::Symbol(ParenOp) => {
                self.write_token(token);
                self.write_token(&look_ahead);
                self.stream.set_position(read_pos);

                self.compile_expr_list();
            }
            // -------------- (className|varName)'.'subroutineName'('expressionList')' -------------- //
            Token::Identifier(_) if look_ahead == Token::Symbol(Period) => {
                self.write_token(token);
                self.write_token(&look_ahead);
                self.stream.set_position(read_pos);

                let func_name = self.get_next_token().unwrap();
                assert!(matches!(func_name, Token::Identifier(_)));
                self.write_token(&func_name);

                let paren_open = self.get_next_token().unwrap();
                assert_eq!(paren_open, Token::Symbol(ParenOp));
                self.write_token(&paren_open);

                self.compile_expr_list();
            }
            // ----------------------------------- (unaryOp term) ----------------------------------- //
            Token::Symbol(Tilde) | Token::Symbol(Minus) => {
                self.write_token(token);
                self.write_xml("term", OPEN);
                self.stream.set_position(read_pos);

                self.compile_term(&look_ahead);
                self.write_xml("term", CLOSE);
            }
            x => self.write_token(token),
        }
    }

    pub fn compile_expr_list(&mut self) {
        self.write_xml("expressionList", OPEN);
        let mut delim;

        while let Ok((token, read_pos)) =  self.peek_next_token() {
            if token == Token::Symbol(ParenCl) {
                self.stream.set_position(read_pos);
                break;
            }

            delim = self.compile_expression(&Token::Symbol(Comma));
            if delim == Token::Symbol(ParenCl) {
                break;
            } else {
                self.write_token(&delim);
            }
        }

        self.write_xml("expressionList", CLOSE);
        // all expression lists are wrapped in parens
        self.write_token(&Token::Symbol(ParenCl));
    }
}
