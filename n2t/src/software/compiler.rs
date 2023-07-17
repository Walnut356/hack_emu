use crate::{
    software::compiler_enums::{DType, Keyword, Token},
    utils::get_file_buffers,
};
use concat_string::concat_string;
use std::{
    error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Cursor, Lines, Read, Write},
    iter::Peekable,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::compiler_enums::Symbol;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const COMMENT: &str = "//";
const COMMENT_OPEN: &str = "/*";
const API_COMMENT_OPEN: &str = "/**";
const COMMENT_CLOSE: &str = "*/";
// ASCII encodings
const SPACE: u8 = 0x20;
const NEWLINE: u8 = 0x0a;
const C_RETURN: u8 = 0x0d; // thanks windows
const PAREN_OPEN: u8 = "(".as_bytes()[0];
const PAREN_CLOSE: u8 = ")".as_bytes()[0];

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

pub fn peek(stream: &mut Cursor<String>) -> Result<[u8; 1]> {
    let mut buff = [0];
    stream.read_exact(&mut buff)?;

    stream.set_position(stream.position() - 1);

    Ok(buff)
}

pub fn peek_eq(val: &str, stream: &mut Cursor<String>) -> Result<bool> {
    assert_eq!(val.len(), 1, "peek_eq requires a str input of length 1");
    let byte = peek(stream).unwrap();

    if byte[0] == val.bytes().next().unwrap() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn read_byte(stream: &mut Cursor<String>) -> Result<[u8; 1]> {
    let mut buff: [u8; 1] = [0];
    stream.read_exact(&mut buff)?;

    Ok(buff)
}

/// Accepts a stream, reads spaces until the first non-space character is found, returns a [u8; 1]
/// containing the non-space character
pub fn skip_spaces(stream: &mut Cursor<String>) -> Result<[u8; 1]> {
    let mut character = [SPACE];
    while character == [SPACE] {
        stream.read_exact(&mut character)?;
    }

    Ok(character)
}

/// Accepts a stream, reads and discards until a newline character is found
pub fn skip_to_newline(stream: &mut Cursor<String>) {
    let mut character = [0];
    while character != [NEWLINE] {
        stream.read_exact(&mut character).unwrap();
    }
}

/// Accepts a stream, reads and discards characters (at least 1) until the first non-newline, non-space character is
/// found. Returns that character.
pub fn skip_whitespace(stream: &mut Cursor<String>) -> Result<[u8; 1]> {
    let mut character = [SPACE];
    while character == [SPACE] || character == [NEWLINE] || character == [C_RETURN] {
        stream.read_exact(&mut character)?;
    }

    Ok(character)
}

pub fn skip_to_comment_end(stream: &mut Cursor<String>) {
    let mut character = [0];

    loop {
        while character != ["*".as_bytes()[0]] {
            stream.read_exact(&mut character).unwrap();
        }
        if peek_eq("/", stream).unwrap() {
            read_byte(stream).unwrap();
            break;
        }
    }
}

pub fn skip_comment(com_type: Keyword, stream: &mut Cursor<String>) {
    match com_type {
        Keyword::Comment => skip_to_newline(stream),
        Keyword::MComment | Keyword::APIComment => skip_to_comment_end(stream),
        _ => panic!("Invalid Comment Type"),
    };
}

/// Accepts a stream, reads characters until a space is found, returns a String containing the characters read WITHOUT
/// a trailing space
pub fn get_next_token(stream: &mut Cursor<String>) -> Result<String> {
    let mut character = skip_whitespace(stream)?;
    let mut token = Vec::new();

    while character != [SPACE] && character != [NEWLINE] && character != [C_RETURN] {
        let next_byte = peek(stream).unwrap();

        let next_res = Symbol::from_str(std::str::from_utf8(&next_byte).unwrap());

        // if we currently have a symbol
        if let Ok(curr_symbol) = Symbol::from_str(std::str::from_utf8(&character).unwrap()) {
            // and the next byte is a symbol
            if let Ok(next_symbol) = next_res {
                // and the next symbol is forward slash or astersik, return "//" or "/*"
                if curr_symbol == Symbol::FwdSlash
                    || curr_symbol == Symbol::Asterisk
                        && (next_symbol == Symbol::FwdSlash || next_symbol == Symbol::Asterisk)
                {
                    read_byte(stream).unwrap();
                    return Ok(concat_string!(
                        curr_symbol.to_string(),
                        next_symbol.to_string()
                    ));
                } else {
                    return Ok(curr_symbol.to_string());
                }
            }
            // if the next byte is not a symbol, return the current byte
            return Ok(curr_symbol.to_string());
        }

        token.push(character[0]);

        if let Ok(_) = next_res {
            break;
        }

        stream.read_exact(&mut character)?;
    }

    Ok(std::str::from_utf8(&token).unwrap().to_owned())
}

pub fn expect_bytes(expected: &str, got: &[u8]) {
    let got_str = std::str::from_utf8(&got).unwrap();

    assert_eq!(got_str, expected);
}

pub fn xml_token(token: &Token) -> String {
    match token {
        Token::Keyword(t) => concat_string!("<keyword> ", t, " </keyword>\n"),
        Token::Symbol(t) => concat_string!("<symbol> ", t, " </symbol>\n"),
        Token::Identifier(t) => concat_string!("<identifier> ", t, " </identifier>\n"),
        Token::ConstString(t) => concat_string!("<constant> ", t, " </constant>\n"),
        Token::ConstInt(t) => concat_string!("<constant> ", t.to_string(), " </constant>\n"),
    }
}

pub fn get_token_type(token: &str) -> Token {
    if let Ok(t) = Symbol::from_str(token) {
        return Token::Symbol(t.to_string());
    }
    if let Ok(t) = Keyword::from_str(token) {
        return Token::Keyword(t.to_string());
    }
    if token.chars().nth(0).unwrap().is_numeric() {
        return Token::ConstInt(token.parse().unwrap());
    }
    if token.starts_with('"') && token.ends_with('"') {
        return Token::ConstString(
            token
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_owned(),
        );
    }
    Token::Identifier(token.to_owned())
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
