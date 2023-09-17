//! jack -> vm code compiler. Divider comments are the lexical elements and program structure of the
//! jack language, mostly for my own sanity during debugging.

use crate::{
    software::compiler_utils::{
        close_xml_group, get_next_token, open_xml_group, peek_next_token, reset_depth, write_token,
        write_xml, xml_token, Keyword::*, Symbol::*, Token, INDENT_DEPTH,
    },
    utils::get_file_buffers,
};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use super::compiler_utils::{skip_comment, DELIM_MAP};

const OPEN: bool = true;
const CLOSE: bool = false;

/// Takes a path to a .jack file or a folder containing .jack files, compiles those files into
/// .vm files, and returns the path to the file(s).
///
/// NOTE: this function relies on global mutable state. Do not run in parallel.
pub fn jack_to_vm(path: &Path) -> PathBuf {
    let mut out_path;

    if path.is_file() {
        out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    } else {
        out_path = Path::new(path).join(path.file_stem().unwrap());
    }

    let files = get_file_buffers(path, "jack");

    // Init output .asm file
    out_path.set_extension("xml");

    let token_file = File::create(out_path.clone()).unwrap();
    let mut token_out = BufWriter::new(token_file);

    for (mut file, _f_name) in files {
        let mut stream = String::new();
        file.read_to_string(&mut stream).unwrap();
        tokenize(Cursor::new(stream), &mut token_out);
    }

    token_out.flush().unwrap();

    // let out_file = File::create(out_path.clone()).unwrap();
    // let mut output = BufWriter::new(out_file);

    // TODO everything else =)

    // output.flush().unwrap();

    out_path
}

/// Tokenizes the given file buffer. Acts as an entrypoint, looking only for one `class`
/// declaration, it then proceeds to recursively parse the contents of the class.
pub fn tokenize(mut stream: Cursor<String>, output: &mut BufWriter<File>) {
    // i attempted to parse by .lines() and by .split_whitespace(), but both lacked a bit of
    // granularity that i felt i needed so i don't mind doing it byte-by-byte
    let mut buff = Token::None;
    let mut err_counter = 0;

    reset_depth();

    write_xml("class", OPEN, output);

    // ----------------------------------------- 'class' ---------------------------------------- //
    while buff != Token::Keyword(Class) {
        buff = get_next_token(&mut stream).unwrap();
        err_counter += 1;
        if err_counter >= 10000 {
            // 10k seems like a good enough number of iterations to panic on just in case i mess
            // something up.
            panic!("unable to find identifier 'class'")
        }
    }
    write_token(&buff, output);

    // ---------------------------------------- className --------------------------------------- //
    let identifier = get_next_token(&mut stream).unwrap();
    assert!(matches!(identifier, Token::Identifier(_)));
    write_token(&identifier, output);

    // ------------------------------------------- '{' ------------------------------------------ //
    let bracket = get_next_token(&mut stream).unwrap();
    assert_eq!(bracket, Token::Symbol(BracketOp));
    write_token(&bracket, output);

    // ------------------------------- classVarDec* subroutineDec* ------------------------------ //
    while let Ok(token) = get_next_token(&mut stream) {
        if token == Token::Symbol(BracketCl) {
            write_token(&token, output);
            break;
        }
        keyword_dispatch(token, &mut stream, output);
    }

    // ------------------------------------------- '}' ------------------------------------------ //
    write_xml("class", CLOSE, output);
}

pub fn keyword_dispatch(token: Token, stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    if let Token::Keyword(keyword) = token {
        // if keyword != Comment && keyword != MComment && keyword != APIComment {
        //     write!(output, "{}", xml_token(&Token::Keyword(keyword))).unwrap();
        //     dbg!(output.flush().unwrap());
        // }

        match keyword {
            Comment | MComment | APIComment => skip_comment(keyword, stream),
            Static | Field => {
                write_xml("classVarDec", OPEN, output);
                write_token(&token, output);

                compile_decl(stream, output);

                write_xml("classVarDec", CLOSE, output);
            }
            Var => {
                write_xml("varDec", OPEN, output);
                write_token(&token, output);
                compile_decl(stream, output);
                write_xml("varDec", CLOSE, output);
            }
            Function | Method | Constructor => {
                write_xml("subroutineDec", OPEN, output);
                write_token(&token, output);

                compile_func(stream, output);

                write_xml("subroutineDec", CLOSE, output);
            }
            Let => {
                write_xml("letStatement", OPEN, output);
                write_token(&token, output);

                compile_let(stream, output);
                write_xml("letStatement", CLOSE, output);
            }
            Do => {
                write_xml("doStatement", OPEN, output);
                write_token(&token, output);
                compile_func_call(stream, output);
                let semicolon = get_next_token(stream).unwrap();
                assert_eq!(semicolon, Token::Symbol(SemiColon));
                write_token(&semicolon, output);
                write_xml("doStatement", CLOSE, output);
            }
            Return => {
                write_xml("returnStatement", OPEN, output);
                write_token(&token, output);

                compile_return(stream, output);
                write_xml("returnStatement", CLOSE, output);
            }
            If => {
                write_xml("ifStatement", OPEN, output);
                write_token(&token, output);

                compile_if(stream, output);
                write_xml("ifStatement", CLOSE, output);
            }
            While => {
                write_xml("whileStatement", OPEN, output);
                write_token(&token, output);

                compile_while(stream, output);
                write_xml("whileStatement", CLOSE, output);
            }
            Else => panic!("Dangling Else"),
            t => panic!("Invalid statement keyword: {:?}", t),
        }
    } else {
        panic!("Invalid token: {:?}", token);
    }
}

pub fn compile_return(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ---------------------------------------- 'return' ---------------------------------------- //

    let (next_token, read_pos) = peek_next_token(stream).unwrap();

    if next_token != Token::Symbol(SemiColon) {
        // ------------------------------------- expression? ------------------------------------ //
        compile_expression(&Token::Symbol(SemiColon), stream, output);
        write_token(&Token::Symbol(SemiColon), output);
    } else {
        write_token(&next_token, output);
        stream.set_position(read_pos);
    }
}

pub fn compile_decl(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ------------------------------ ('static' | 'field' | 'var') ------------------------------ //

    // ------------------------------------------ type ------------------------------------------ //
    let dtype = get_next_token(stream).unwrap();
    assert!(dtype.is_type());
    write_token(&dtype, output);

    // --------------------------------- varName (',' varName)* --------------------------------- //
    while let Ok(token) = get_next_token(stream) {
        write_token(&token, output);

        if token == Token::Symbol(SemiColon) {
            break;
        }

        // covers patterns `var int i` and `var int i, j, k;
        assert!(matches!(token, Token::Identifier(_) | Token::Symbol(Comma)));
    }
}

pub fn compile_func(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ------------------------- ('constructor' | 'function' | 'method') ------------------------ //

    // ------------------------------------- ('void' | type) ------------------------------------ //
    let ret_type = get_next_token(stream).unwrap();
    assert!(ret_type == Token::Keyword(Void) || ret_type.is_type());
    write_token(&ret_type, output);

    // ------------------------------------- subroutineName ------------------------------------- //
    let func_name = get_next_token(stream).unwrap();
    assert!(func_name.is_identifier());
    write_token(&func_name, output);

    // ------------------------------------------- '(' ------------------------------------------ //
    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write_token(&open_paren, output);

    // -------------------------------------- parameterList ------------------------------------- //
    write_xml("parameterList", OPEN, output);

    // -------------------------- ((type varName) (',' type varName)*)? ------------------------- //
    while let Ok(token) = get_next_token(stream) {
        // if the first token is a ParenCl, the paramlist is empty, but still writes tags
        if token == Token::Symbol(ParenCl) {
            break;
        }

        // token is a dtype or a comma
        write_token(&token, output);

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
        let name = get_next_token(stream).unwrap();
        assert!(matches!(name, Token::Identifier(_)));
        write_token(&name, output);
    }

    // ------------------------------------------- ')' ------------------------------------------ //
    write_xml("parameterList", CLOSE, output);
    write_token(&Token::Symbol(ParenCl), output);

    // ------------------------------------- subroutineBody ------------------------------------- //
    compile_func_body(stream, output);
}

pub fn compile_func_body(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    write_xml("subroutineBody", OPEN, output);

    // ------------------------------------------- '{' ------------------------------------------ //
    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write_token(&open_brac, output);

    // ----------------------------------------- varDec* ---------------------------------------- //
    while let Ok((next_token, read_pos)) = peek_next_token(stream) {
        if next_token != Token::Keyword(Var) {
            break;
        }
        stream.set_position(read_pos);
        keyword_dispatch(next_token, stream, output);
    }

    // --------------------------------------- statements --------------------------------------- //
    write_xml("statements", OPEN, output);

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            break;
        }

        assert!(token.is_statement(), "got: {token:?}");
        keyword_dispatch(token, stream, output);
    }

    // ------------------------------------------- '}' ------------------------------------------ //
    write_xml("statements", CLOSE, output);
    write_token(&Token::Symbol(BracketCl), output);

    write_xml("subroutineBody", CLOSE, output);
}

pub fn compile_func_call(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ------------------------------------ 'do' or from term ----------------------------------- //

    // ------------------------------------- subroutineCall ------------------------------------- //
    let identifier = get_next_token(stream).unwrap();
    compile_term(&identifier, stream, output);
}

pub fn compile_let(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ------------------------------------------ 'let' ----------------------------------------- //

    // ----------------------------------------- varName ---------------------------------------- //
    let id = get_next_token(stream).unwrap();
    assert!(matches!(id, Token::Identifier(_)));
    write_token(&id, output);

    let (next_token, read_pos) = peek_next_token(stream).unwrap();

    // ------------------------------------------ ('[' ------------------------------------------ //
    if next_token == Token::Symbol(BraceOp) {
        write_token(&next_token, output);
        stream.set_position(read_pos);

        // ------------------------------------- expression ------------------------------------- //
        compile_expression(&Token::Symbol(BraceCl), stream, output);
        // ---------------------------------------- ']')? --------------------------------------- //
        write_token(&Token::Symbol(BraceCl), output);
    }

    // ------------------------------------------- '=' ------------------------------------------ //
    let token = get_next_token(stream).unwrap();
    assert_eq!(token, Token::Symbol(Equals));
    write_token(&token, output);

    // --------------------------------------- expression --------------------------------------- //
    compile_expression(&Token::Symbol(SemiColon), stream, output);

    // ------------------------------------------- ';' ------------------------------------------ //
    write_token(&Token::Symbol(SemiColon), output);
}

pub fn compile_if(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ------------------------------------------ 'if' ------------------------------------------ //

    // ------------------------------------------- '(' ------------------------------------------ //
    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write_token(&open_paren, output);

    // --------------------------------------- expression --------------------------------------- //
    compile_expression(&Token::Symbol(ParenCl), stream, output);

    // ------------------------------------------- ')' ------------------------------------------ //
    write_token(&Token::Symbol(ParenCl), output);

    // ------------------------------------------- '{' ------------------------------------------ //
    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write_token(&open_brac, output);

    // --------------------------------------- statements --------------------------------------- //
    write_xml("statements", OPEN, output);

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            break;
        }

        assert!(token.is_statement());
        keyword_dispatch(token, stream, output)
    }

    write_xml("statements", CLOSE, output);

    // ------------------------------------------- '}' ------------------------------------------ //
    write_token(&Token::Symbol(BracketCl), output);

    // ----------------------------------------- ('else' ---------------------------------------- //
    let (maybe_else, read_pos) = peek_next_token(stream).unwrap();

    if maybe_else == Token::Keyword(Else) {
        stream.set_position(read_pos);
        write_token(&maybe_else, output);

        // ---------------------------------- [rest of else])? ---------------------------------- //
        compile_else(stream, output)
    }
}

pub fn compile_else(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ----------------------------------------- ('else' ---------------------------------------- //

    // ------------------------------------------- '{' ------------------------------------------ //
    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write_token(&open_brac, output);

    // --------------------------------------- statements --------------------------------------- //
    write_xml("statements", OPEN, output);
    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            break;
        }
        keyword_dispatch(token, stream, output)
    }

    write_xml("statements", CLOSE, output);

    // ------------------------------------------- '}' ------------------------------------------ //
    write_token(&Token::Symbol(BracketCl), output);

    // ------------------------------------------- )? ------------------------------------------- //
}

pub fn compile_while(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    // ----------------------------------------- 'while' ---------------------------------------- //

    // ------------------------------------------- '(' ------------------------------------------ //
    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write_token(&open_paren, output);

    // --------------------------------------- expression --------------------------------------- //
    compile_expression(&Token::Symbol(ParenCl), stream, output);

    write_token(&Token::Symbol(ParenCl), output);

    // ------------------------------------------- '{' ------------------------------------------ //
    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write_token(&open_brac, output);

    // --------------------------------------- statements --------------------------------------- //
    write_xml("statements", OPEN, output);
    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            break;
        }
        keyword_dispatch(token, stream, output)
    }
    write_xml("statements", CLOSE, output);

    // ------------------------------------------- '}' ------------------------------------------ //
    write_token(&Token::Symbol(BracketCl), output);
}

pub fn compile_expression(
    delim: &Token,
    stream: &mut Cursor<String>,
    output: &mut BufWriter<File>,
) -> Token {
    write_xml("expression", OPEN, output);
    /*
    Brain's not working so i'll leave a note for later: i'm going to assume that there's always 1
    term, followed by [something]. [Something] can either be the delimeter or an operator, but the
    the sequence must *start* with a term, and *end* with a term (just before the delimiter). That
    means i can loop through "token" pairs, which should allow me to be a bit more strict with
    asserts and narrow the match statements.
    */

    let token = get_next_token(stream).unwrap();
    if token == *delim || token == Token::Symbol(ParenCl) || token == Token::Symbol(BraceCl)
    // || token == Token::Symbol(SemiColon)
    {
        write_xml("expression", CLOSE, output);
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

    write_xml("term", OPEN, output);
    compile_term(&token, stream, output);
    write_xml("term", CLOSE, output);

    let mut op = get_next_token(stream).unwrap();
    // --------------------------------------- (op term)* --------------------------------------- //
    while op.is_operator() {
        write_token(&op, output);

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

        write_xml("term", OPEN, output);
        let token = get_next_token(stream).unwrap();
        compile_term(&token, stream, output);
        write_xml("term", CLOSE, output);

        op = get_next_token(stream).unwrap();
    }

    write_xml("expression", CLOSE, output);
    op
}

pub fn compile_term(token: &Token, stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let (look_ahead, read_pos) = peek_next_token(stream).unwrap();

    match token {
        // --------------------------------- '(' expression ')' --------------------------------- //
        Token::Symbol(ParenOp) => {
            write_token(token, output);
            compile_expression(&Token::Symbol(ParenCl), stream, output);
            write_token(&Token::Symbol(ParenCl), output);
        }
        // ----------------------------- varName '[' expression ']' ----------------------------- //
        Token::Identifier(_) if look_ahead == Token::Symbol(BraceOp) => {
            write_token(token, output);
            write_token(&look_ahead, output);
            stream.set_position(read_pos);

            compile_expression(&Token::Symbol(BraceCl), stream, output);
            write_token(&Token::Symbol(BraceCl), output);
        }
        // ------------------------- subroutineName'('expressionList')' ------------------------- //
        Token::Identifier(_) if look_ahead == Token::Symbol(ParenOp) => {
            write_token(token, output);
            write_token(&look_ahead, output);
            stream.set_position(read_pos);

            compile_expr_list(stream, output);
        }
        // -------------- (className|varName)'.'subroutineName'('expressionList')' -------------- //
        Token::Identifier(_) if look_ahead == Token::Symbol(Period) => {
            write_token(token, output);
            write_token(&look_ahead, output);
            stream.set_position(read_pos);

            let func_name = get_next_token(stream).unwrap();
            assert!(matches!(func_name, Token::Identifier(_)));
            write_token(&func_name, output);

            let paren_open = get_next_token(stream).unwrap();
            assert_eq!(paren_open, Token::Symbol(ParenOp));
            write_token(&paren_open, output);

            compile_expr_list(stream, output);
        }
        // ----------------------------------- (unaryOp term) ----------------------------------- //
        Token::Symbol(Tilde) | Token::Symbol(Minus) => {
            write_token(token, output);
            write_xml("term", OPEN, output);
            stream.set_position(read_pos);

            compile_term(&look_ahead, stream, output);
            write_xml("term", CLOSE, output);
        }
        x => write_token(token, output),
    }
}

pub fn compile_expr_list(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    write_xml("expressionList", OPEN, output);
    let mut delim;

    while let Ok((token, read_pos)) = peek_next_token(stream) {
        if token == Token::Symbol(ParenCl) {
            stream.set_position(read_pos);
            break;
        }

        delim = compile_expression(&Token::Symbol(Comma), stream, output);
        if delim == Token::Symbol(ParenCl) {
            break;
        } else {
            write_token(&delim, output);
        }
    }

    write_xml("expressionList", CLOSE, output);
    // all expression lists are wrapped in parens
    write_token(&Token::Symbol(ParenCl), output);
}
