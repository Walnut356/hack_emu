use crate::{
    software::compiler_utils::{get_next_token, xml_token, Keyword::*, Symbol::*, Token},
    utils::get_file_buffers,
};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use super::compiler_utils::{skip_comment, DELIM_MAP};

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

    write!(token_out, "<tokens>\n").unwrap();

    for (mut file, _f_name) in files {
        let mut stream = String::new();
        file.read_to_string(&mut stream).unwrap();
        tokenize(Cursor::new(stream), &mut token_out);
    }

    write!(token_out, "</tokens>\n").unwrap();
    token_out.flush().unwrap();

    // let out_file = File::create(out_path.clone()).unwrap();
    // let mut output = BufWriter::new(out_file);

    // TODO everything else =)

    // output.flush().unwrap();

    out_path
}

/// Tokenizes the given file buffer. Acts as an entrypoint, looking only for one `class` declaration, it then proceeds
/// to recursively parse the contents of the class.
pub fn tokenize(mut stream: Cursor<String>, output: &mut BufWriter<File>) {
    // i attempted to parse by .lines() and by .split_whitespace(), but both lacked a bit of granularity that i felt i needed
    // so i don't mind doing it byte-by-byte
    let mut buff = Token::None;
    let mut err_counter = 0;

    while buff != Token::Keyword(Class) {
        buff = get_next_token(&mut stream).unwrap();
        err_counter += 1;
        if err_counter >= 10000 {
            // 10k seems like a good enough number of iterations to panic on just in case i mess something up.
            panic!("unable to find identifier 'class'")
        }
    }

    write!(output, "{}", xml_token(&buff)).unwrap();

    let identifier = get_next_token(&mut stream).unwrap();
    let bracket = get_next_token(&mut stream).unwrap();

    write!(output, "{}", xml_token(&identifier)).unwrap();

    assert_eq!(bracket, Token::Symbol(BracketOp));
    write!(output, "{}", xml_token(&bracket)).unwrap();

    while let Ok(token) = get_next_token(&mut stream) {
        if token == Token::Symbol(BracketCl) {
            write!(output, "{}", xml_token(&token)).unwrap();
            break;
        }
        keyword_dispatch(token, &mut stream, output);
    }
}

pub fn keyword_dispatch(token: Token, stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    if let Token::Keyword(keyword) = token {
        if keyword != Comment && keyword != MComment && keyword != APIComment {
            write!(output, "{}", xml_token(&Token::Keyword(keyword))).unwrap();
        }

        match keyword {
            Comment | MComment | APIComment => skip_comment(keyword, stream),
            Static | Var | Field => {
                compile_decl(stream, output);
            }
            Function | Method | Constructor => {
                compile_func(stream, output);
            }
            Let => {
                compile_let(stream, output);
            }
            Do => compile_do(stream, output),
            Return => {
                compile_return(stream, output);
            }
            If => compile_if(stream, output),
            Else => compile_else(stream, output),
            While => compile_while(stream, output),
            t => panic!("Invalid statement keyword: {:?}", t),
        }
    } else {
        panic!("Invalid token: {:?}", token);
    }
}

pub fn compile_return(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    compile_expression(&Token::Symbol(SemiColon), stream, output);

    // let close_brac = get_next_token(stream).unwrap();
    // assert_eq!(close_brac, Token::Symbol(BracketCl));

    // write!(output, "{}", &xml_token(&close_brac)).unwrap();
}

pub fn compile_decl(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let dtype = get_next_token(stream).unwrap();

    write!(output, "{}", &xml_token(&dtype)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        write!(output, "{}", xml_token(&token)).unwrap();
        if token == Token::Symbol(SemiColon) {
            break;
        }
    }
}

pub fn compile_func(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let ret_type = get_next_token(stream).unwrap();

    write!(output, "{}", xml_token(&ret_type)).unwrap();

    let func_name = get_next_token(stream).unwrap();
    write!(output, "{}", xml_token(&func_name)).unwrap();

    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write!(output, "{}", xml_token(&open_paren)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        write!(output, "{}", xml_token(&token)).unwrap();

        if token == Token::Symbol(ParenCl) {
            break;
        }

        if token == Token::Symbol(Comma) {
            continue;
        }

        // if we're past the conditionals, the token was a dtype, so the next token must be the arg's name
        let name = get_next_token(stream).unwrap();
        write!(output, "{}", xml_token(&name)).unwrap();
    }

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write!(output, "{}", xml_token(&open_brac)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            write!(output, "{}", xml_token(&(token))).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output);
    }
}

pub fn compile_let(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let id = get_next_token(stream).unwrap();
    write!(output, "{}", xml_token(&id)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        write!(output, "{}", xml_token(&token)).unwrap();
        if token == Token::Symbol(Equals) {
            break;
        }
        if token == Token::Symbol(BracketOp) {
            compile_expression(&Token::Symbol(BracketCl), stream, output);
        }
    }

    compile_expression(&Token::Symbol(SemiColon), stream, output);
}

pub fn compile_do(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let class_name = get_next_token(stream).unwrap();
    write!(output, "{}", xml_token(&class_name)).unwrap();

    let mut token = get_next_token(stream).unwrap();

    if token == Token::Symbol(Period) {
        write!(output, "{}", xml_token(&token)).unwrap();

        let func_name = get_next_token(stream).unwrap();
        write!(output, "{}", xml_token(&func_name)).unwrap();

        token = get_next_token(stream).unwrap();
    }

    assert_eq!(token, Token::Symbol(ParenOp));
    write!(output, "{}", xml_token(&token)).unwrap();

    compile_expression(&Token::Symbol(ParenCl), stream, output);
    // while let Ok(token) = get_next_token(stream) {
    //     write!(output, "{}", xml_token(&token)).unwrap();
    //     if token == Token::Symbol(ParenCl) {
    //         break;
    //     }
    //     if token == Token::Symbol(Comma) {
    //         continue;
    //     }

    // }

    let delim = get_next_token(stream).unwrap();
    assert_eq!(delim, Token::Symbol(SemiColon));
    write!(output, "{}", xml_token(&delim)).unwrap();
}

pub fn compile_if(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write!(output, "{}", xml_token(&open_paren)).unwrap();

    compile_expression(&Token::Symbol(ParenCl), stream, output);

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write!(output, "{}", xml_token(&open_brac)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            write!(output, "{}", xml_token(&token)).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_else(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write!(output, "{}", xml_token(&open_brac)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            write!(output, "{}", xml_token(&token)).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_while(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, Token::Symbol(ParenOp));
    write!(output, "{}", xml_token(&open_paren)).unwrap();

    compile_expression(&Token::Symbol(ParenCl), stream, output);

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, Token::Symbol(BracketOp));
    write!(output, "{}", xml_token(&open_brac)).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == Token::Symbol(BracketCl) {
            write!(output, "{}", xml_token(&token)).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_expression(
    delim: &Token,
    stream: &mut Cursor<String>,
    output: &mut BufWriter<File>,
) {
    while let Ok(token) = get_next_token(stream) {
        write!(output, "{}", xml_token(&token)).unwrap();
        if token == *delim {
            break;
        }
        // if we have some sort of delimiter (e.g. '(', '[') we've got a sub-expression so we can do a recursive call
        // with the matching closing delimiter
        if DELIM_MAP.contains_key(&token) {
            compile_expression(DELIM_MAP.get(&token).unwrap(), stream, output);
            continue;
        }
    }
}
