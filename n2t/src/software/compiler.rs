use crate::{software::compiler_utils::*, utils::get_file_buffers};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

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
    let mut buff = String::new();
    let mut err_counter = 0;

    while buff != "class" {
        buff = get_next_token(&mut stream).unwrap();
        err_counter += 1;
        if err_counter >= 10000 || buff.is_empty() {
            // 10k seems like a good enough number of iterations to panic on just in case i mess something up.
            panic!("unable to find identifier 'class'")
        }
    }

    write!(output, "{}", xml_token(&Token::Keyword("class".to_owned()))).unwrap();

    let identifier = get_next_token(&mut stream).unwrap();
    let bracket = get_next_token(&mut stream).unwrap();

    write!(output, "{}", xml_token(&Token::Identifier(identifier))).unwrap();
    write!(output, "{}", xml_token(&Token::Symbol(bracket))).unwrap();

    while let Ok(token) = get_next_token(&mut stream) {
        if token == "}" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        keyword_dispatch(token, &mut stream, output);
    }
}

pub fn keyword_dispatch(token: String, stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let keyword = Keyword::from_str(&token).expect(&format!("Invalid keyword '{}'", token));

    if keyword != Keyword::Comment && keyword != Keyword::MComment && keyword != Keyword::APIComment
    {
        write!(output, "{}", xml_token(&Token::Keyword(token))).unwrap();
    }

    match keyword {
        Keyword::Comment | Keyword::MComment | Keyword::APIComment => skip_comment(keyword, stream),
        Keyword::Static | Keyword::Var | Keyword::Field => {
            compile_decl(stream, output);
        }
        Keyword::Function | Keyword::Method | Keyword::Constructor => {
            compile_func(stream, output);
        }
        Keyword::Let => {
            compile_let(stream, output);
        }
        Keyword::Do => compile_do(stream, output),
        Keyword::Return => {
            compile_return(stream, output);
        }
        Keyword::If => compile_if(stream, output),
        Keyword::Else => compile_else(stream, output),
        Keyword::While => compile_while(stream, output),
        _ => todo!(),
    }
}

pub fn compile_return(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();

    let token = get_next_token(stream).unwrap();

    if token == ";" {
        let semicolon = Token::Symbol(token);
        result.push_str(&xml_token(&semicolon));
    } else {
        let token = Token::Identifier(token);
        result.push_str(&xml_token(&token));

        let semicolon = get_next_token(stream).unwrap();
        assert_eq!(semicolon, ";");
        let semicolon = Token::Symbol(semicolon);
        result.push_str(&xml_token(&semicolon));
    }

    let close_brac = get_next_token(stream).unwrap();
    assert_eq!(close_brac, "}");
    let close_brac = Token::Symbol(close_brac);
    result.push_str(&xml_token(&close_brac));

    write!(output, "{}", result).unwrap();
}

pub fn compile_decl(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let result = String::new();

    let dtype = DType::from_str(&get_next_token(stream).unwrap()).unwrap(); // can't fail due to strum default

    match dtype {
        DType::UserDef(user_type) => {
            write!(
                output,
                "{}",
                xml_token(&Token::Identifier(user_type.clone()))
            )
            .unwrap();
        }
        _ => write!(output, "{}", xml_token(&Token::Keyword(dtype.to_string()))).unwrap(),
    }

    while let Ok(token) = get_next_token(stream) {
        if token == ";" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        let token = get_token_type(&token);
        write!(output, "{}", xml_token(&token)).unwrap();
    }
}

pub fn compile_func(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();
    let ret_type = DType::from_str(&get_next_token(stream).unwrap()).unwrap();

    // can beeither an identifier or a keyword
    match ret_type {
        DType::UserDef(t) => result.push_str(&xml_token(&Token::Identifier(t))),
        _ => result.push_str(&xml_token(&Token::Keyword(ret_type.to_string()))),
    }

    let func_name = Token::Identifier(get_next_token(stream).unwrap());
    result.push_str(&xml_token(&func_name));

    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, "(");
    let open_paren = Token::Symbol(open_paren);
    result.push_str(&xml_token(&open_paren));

    while let Ok(token) = get_next_token(stream) {
        if token == ")" {
            result.push_str(&xml_token(&Token::Symbol(token)));
            break;
        }
        if token == "," {
            result.push_str(&xml_token(&Token::Symbol(token)));
            continue;
        }
        let dtype = get_token_type(&token);
        result.push_str(&xml_token(&dtype));

        let name = Token::Identifier(get_next_token(stream).unwrap());
        result.push_str(&xml_token(&name));
    }

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, "{");
    let open_brac = Token::Symbol(open_brac);
    result.push_str(&xml_token(&open_brac));

    write!(output, "{}", result).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == "}" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output);
    }
}

pub fn compile_let(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();
    let id = Token::Identifier(get_next_token(stream).unwrap());
    result.push_str(&xml_token(&id));

    let eq = get_next_token(stream).unwrap();
    assert_eq!(eq, "=");
    let eq = Token::Symbol(eq);

    result.push_str(&xml_token(&eq));

    // Eventually: compile_expression()
    // which will loop until hitting a semicolon
    let expr = get_token_type(&get_next_token(stream).unwrap());

    result.push_str(&xml_token(&expr));

    let delim = get_next_token(stream).unwrap();
    assert_eq!(delim, ";");
    let delim = Token::Symbol(delim);
    result.push_str(&xml_token(&delim));

    write!(output, "{}", result).unwrap();
}

pub fn compile_do(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();

    let class_name = Token::Identifier(get_next_token(stream).unwrap());
    result.push_str(&xml_token(&class_name));

    let mut token = get_next_token(stream).unwrap();

    if token == "." {
        let dot = Token::Symbol(token.clone());
        result.push_str(&xml_token(&dot));

        let func_name = Token::Identifier(get_next_token(stream).unwrap());
        result.push_str(&xml_token(&func_name));

        token = get_next_token(stream).unwrap();
    }

    assert_eq!(token, "(");
    let open_paren = Token::Symbol(token);
    result.push_str(&xml_token(&open_paren));

    while let Ok(token) = get_next_token(stream) {
        if token == ")" {
            result.push_str(&xml_token(&Token::Symbol(token)));
            break;
        }
        if token == "," {
            result.push_str(&xml_token(&Token::Symbol(token)));
            continue;
        }
        let arg = get_token_type(&token);
        result.push_str(&xml_token(&arg));
    }

    let delim = get_next_token(stream).unwrap();
    assert_eq!(delim, ";");
    let delim = Token::Symbol(delim);
    result.push_str(&xml_token(&delim));

    write!(output, "{}", result).unwrap();
}

pub fn compile_if(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();

    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, "(");
    let open_paren = Token::Symbol(open_paren);
    result.push_str(&xml_token(&open_paren));

    // todo compile_expression

    let token = get_token_type(&get_next_token(stream).unwrap());

    result.push_str(&xml_token(&token));

    let close_paren = get_next_token(stream).unwrap();
    assert_eq!(close_paren, ")");
    let close_paren = Token::Symbol(close_paren);
    result.push_str(&xml_token(&close_paren));

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, "{");
    let open_brac = Token::Symbol(open_brac);
    result.push_str(&xml_token(&open_brac));

    write!(output, "{}", result).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == "}" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_else(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, "{");
    let open_brac = Token::Symbol(open_brac);
    result.push_str(&xml_token(&open_brac));

    write!(output, "{}", result).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == "}" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_while(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {
    let mut result = String::new();

    let open_paren = get_next_token(stream).unwrap();
    assert_eq!(open_paren, "(");
    let open_paren = Token::Symbol(open_paren);
    result.push_str(&xml_token(&open_paren));

    // TODO compile_expression

    let token = get_next_token(stream).unwrap();
    let token = get_token_type(&token);
    result.push_str(&xml_token(&token));

    let close_paren = get_next_token(stream).unwrap();
    assert_eq!(close_paren, ")");
    let close_paren = Token::Symbol(close_paren);
    result.push_str(&xml_token(&close_paren));

    let open_brac = get_next_token(stream).unwrap();
    assert_eq!(open_brac, "{");
    let open_brac = Token::Symbol(open_brac);
    result.push_str(&xml_token(&open_brac));

    write!(output, "{}", result).unwrap();

    while let Ok(token) = get_next_token(stream) {
        if token == "}" {
            write!(output, "{}", xml_token(&Token::Symbol(token))).unwrap();
            break;
        }
        keyword_dispatch(token, stream, output)
    }
}

pub fn compile_expression(stream: &mut Cursor<String>, output: &mut BufWriter<File>) {}
