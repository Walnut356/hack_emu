#![allow(non_camel_case_types)]

use concat_string::concat_string;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Cursor, Write};
use std::sync::Mutex;
use std::{error, io::Read, str::FromStr};

use strum_macros::{EnumString, IntoStaticStr};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// ugly but oh well
pub static INDENT_DEPTH: Mutex<usize> = Mutex::new(0);

fn incr_depth() {
    *INDENT_DEPTH.lock().unwrap() += 1;
}

fn decr_depth() {
    *INDENT_DEPTH.lock().unwrap() -= 1;
}

pub fn reset_depth() {
    *INDENT_DEPTH.lock().unwrap() == 0;
}

// const COMMENT: &str = "//";
// const COMMENT_OPEN: &str = "/*";
// const API_COMMENT_OPEN: &str = "/**";
// const COMMENT_CLOSE: &str = "*/";
// ASCII encodings
const SPACE: u8 = 0x20;
const TAB: u8 = 0x09;
const NEWLINE: u8 = 0x0a;
const C_RETURN: u8 = 0x0d; // thanks windows
const PAREN_OPEN: u8 = "(".as_bytes()[0];
const PAREN_CLOSE: u8 = ")".as_bytes()[0];
const DBL_QUOTE: u8 = "\"".as_bytes()[0];
lazy_static! {
    pub static ref DELIM_MAP: HashMap<Token, Token> = {
        use Symbol::*;
        let mut temp = HashMap::new();
        temp.insert(Token::Symbol(BracketOp), Token::Symbol(BracketCl));
        temp.insert(Token::Symbol(ParenOp), Token::Symbol(ParenCl));
        temp.insert(Token::Symbol(BraceOp), Token::Symbol(BraceCl));

        temp
    };
}

#[derive(Debug, Clone, Copy, EnumString, strum_macros::Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    Var,
    Int,
    Boolean,
    Char,
    #[strum(serialize = "//")]
    Comment,
    #[strum(serialize = "/*")]
    MComment,
    #[strum(serialize = "/**")]
    APIComment,
    #[strum(serialize = "*/")]
    CommentEnd,
}

#[derive(Debug, Clone, Copy, EnumString, strum_macros::Display, PartialEq, Eq, Hash)]
pub enum Symbol {
    #[strum(serialize = "{")]
    BracketOp,
    #[strum(serialize = "}")]
    BracketCl,
    #[strum(serialize = "(")]
    ParenOp,
    #[strum(serialize = ")")]
    ParenCl,
    #[strum(serialize = "[")]
    BraceOp,
    #[strum(serialize = "]")]
    BraceCl,
    #[strum(serialize = ".")]
    Period,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    SemiColon,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Asterisk,
    #[strum(serialize = "/")]
    FwdSlash,
    #[strum(to_string = "&amp;")] // & is reserved in XML
    #[strum(serialize = "&")]
    And,
    #[strum(serialize = "|")]
    Pipe,
    #[strum(to_string = "&lt;")] // < is reserved in XML
    #[strum(serialize = "<")]
    LessThan,
    #[strum(to_string = "&gt;")] // > is reserved in XML
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "=")]
    Equals,
    #[strum(serialize = "~")]
    Tilde,
    #[strum(to_string = "&quot;")] // Double quotes are reserved in XML
    #[strum(serialize = "\"")]
    DblQuote,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    ConstString(String),
    ConstInt(i16),
    None,
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Symbol(s) => matches!(
                s,
                Symbol::Plus
                    | Symbol::Minus
                    | Symbol::Asterisk
                    | Symbol::FwdSlash
                    | Symbol::And
                    | Symbol::Pipe
                    | Symbol::GreaterThan
                    | Symbol::LessThan
                    | Symbol::Equals
                    | Symbol::Tilde
            ),
            _ => false,
        }
    }
    pub fn is_unary_operator(&self) -> bool {
        match self {
            Token::Symbol(s) => matches!(s, Symbol::Minus | Symbol::Tilde),
            _ => false,
        }
    }

    pub fn is_keyword_const(&self) -> bool {
        use Keyword::*;
        match self {
            Token::Keyword(k) => matches!(k, True | False | Null | This),
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }

    pub fn is_comment(&self) -> bool {
        use Keyword::*;
        matches!(self, Token::Keyword(Comment))
    }

    pub fn is_long_comment(&self) -> bool {
        use Keyword::*;
        matches!(self, Token::Keyword(APIComment) | Token::Keyword(MComment))
    }

    pub fn is_type(&self) -> bool {
        use Keyword::*;
        matches!(
            self,
            Token::Identifier(_)
                | Token::Keyword(Int)
                | Token::Keyword(Char)
                | Token::Keyword(Boolean)
        )
    }

    pub fn is_statement(&self) -> bool {
        use Keyword::*;
        matches!(
            self,
            Token::Keyword(Let)
                | Token::Keyword(If)
                | Token::Keyword(While)
                | Token::Keyword(Do)
                | Token::Keyword(Return)
        )
    }

    /// true if Self is `)` | `}` | `]`
    pub fn is_closer(&self) -> bool {
        use Symbol::*;
        matches!(
            self,
            Token::Symbol(BracketCl) | Token::Symbol(BraceCl) | Token::Symbol(ParenCl)
        )
    }
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
    while character == [SPACE]
        || character == [NEWLINE]
        || character == [C_RETURN]
        || character == [TAB]
    {
        stream.read_exact(&mut character)?;
    }

    Ok(character)
}

pub fn skip_to_comment_end(stream: &mut Cursor<String>) {
    let mut character = [0];

    while !(character == ["*".as_bytes()[0]] && peek_eq("/", stream).unwrap()) {
        stream.read_exact(&mut character).unwrap();
    }
    read_byte(stream).unwrap(); // consume the peeked '/' character
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
pub fn get_next_token(stream: &mut Cursor<String>) -> Result<Token> {
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
                // this is kindof a hack due to ignoring API comments
                if curr_symbol == Symbol::FwdSlash && next_symbol == Symbol::FwdSlash {
                    skip_comment(Keyword::Comment, stream);
                    return get_next_token(stream);
                }
                if curr_symbol == Symbol::FwdSlash && next_symbol == Symbol::Asterisk {
                    skip_comment(Keyword::MComment, stream);
                    return get_next_token(stream);
                }
                // and it's not a comment, return the symbol
                return Ok(Token::Symbol(curr_symbol));
            }
            if curr_symbol == Symbol::DblQuote {
                // string constants, treats the whole constant as 1 token
                let mut buff = Vec::new();
                stream.read_until(DBL_QUOTE, &mut buff).unwrap();
                buff.pop(); // remove trailing quote

                let const_string = std::string::String::from_utf8(buff).unwrap();

                return Ok(Token::ConstString(const_string));
            }
            // if the next byte is not a symbol, return the current byte
            return Ok(Token::Symbol(curr_symbol));
        }

        token.push(character[0]);

        if next_res.is_ok() {
            break;
        }

        stream.read_exact(&mut character)?;
    }

    Ok(get_token_type(std::str::from_utf8(&token).unwrap()))
}

/// Returns a Result containing a Tuple of the next **non-comment** Token, and the read position
/// corresponding to the end of the peeked token. If the peeked token is used, calling
/// `stream.set_position()` with the returned u64 will update the stream to the proper location
/// without having to re-read the peeked token.
pub fn peek_next_token(stream: &mut Cursor<String>) -> Result<(Token, u64)> {
    let position = stream.position();

    let mut next_token = get_next_token(stream)?;

    use Keyword::*;
    while next_token == Token::Keyword(Comment)
        || next_token == Token::Keyword(MComment)
        || next_token == Token::Keyword(APIComment)
    {
        if next_token == Token::Keyword(Comment) {
            skip_to_newline(stream);
        } else {
            skip_to_comment_end(stream);
        }
        next_token = get_next_token(stream)?
    }

    let post_pos = stream.position();
    stream.set_position(position);

    Ok((next_token, post_pos))
}

/// Tries to match Token::Keyword or Token::ConstInt, then falls back to Token::Identifier
pub fn get_token_type(token: &str) -> Token {
    if let Ok(t) = Keyword::from_str(token) {
        return Token::Keyword(t);
    }
    if token.chars().next().unwrap().is_numeric() {
        return Token::ConstInt(token.parse().unwrap());
    }
    Token::Identifier(token.to_owned())
}

#[inline]
pub fn expect_bytes(expected: &str, got: &[u8]) {
    let got_str = std::str::from_utf8(got).unwrap();

    assert_eq!(got_str, expected);
}

// #[derive(Debug, Clone, Copy, EnumString, strum_macros::Display, PartialEq, Eq, Hash, IntoStaticStr)]
// pub enum XML {
//     class,
//     classVarDec,
//     varDec,
//     subroutineDec,
//     letStatement,
//     parameterList,
//     subroutineBody,
//     statements,
//     term,
//     expression,
//     expressionList,
// }

pub fn xml_token(token: &Token) -> String {
    let indent = "  ".repeat(*INDENT_DEPTH.lock().unwrap());
    match token {
        Token::Keyword(t) => concat_string!(indent, "<keyword> ", t.to_string(), " </keyword>\n"),
        Token::Symbol(t) => concat_string!(indent, "<symbol> ", t.to_string(), " </symbol>\n"),
        Token::Identifier(t) => concat_string!(indent, "<identifier> ", t, " </identifier>\n"),
        Token::ConstString(t) => {
            concat_string!(indent, "<stringConstant> ", t, " </stringConstant>\n")
        }
        Token::ConstInt(t) => {
            concat_string!(
                indent,
                "<integerConstant> ",
                t.to_string(),
                " </integerConstant>\n"
            )
        }
        Token::None => panic!("Cannot create xml token for Token::None"),
    }
}

/// returns a string opening an xml "group", also increments the indent depth
#[inline]
pub fn open_xml_group(group_name: &str) -> String {
    let temp = concat_string!(
        "  ".repeat(*INDENT_DEPTH.lock().unwrap()),
        "<",
        group_name,
        ">\n"
    );
    incr_depth();

    temp
}

/// returns a string closing an xml "group", also decrements the indent depth
#[inline]
pub fn close_xml_group(group_name: &str) -> String {
    decr_depth();
    concat_string!(
        "  ".repeat(*INDENT_DEPTH.lock().unwrap()),
        "</",
        group_name,
        ">\n"
    )
}

#[inline]
pub fn write_token(token: &Token, output: &mut BufWriter<File>) {
    write!(output, "{}", xml_token(token)).unwrap();
    #[cfg(debug_assertions)]
    output.flush().unwrap();
}

#[inline]
pub fn write_xml(name: &str, tag: bool, output: &mut BufWriter<File>) {
    if tag {
        write!(output, "{}", open_xml_group(name)).unwrap();
        #[cfg(debug_assertions)]
        output.flush().unwrap();
    } else {
        write!(output, "{}", close_xml_group(name)).unwrap();
        #[cfg(debug_assertions)]
        output.flush().unwrap();
    }
}
