use concat_string::concat_string;
use std::fmt::Display;
use std::io::{BufRead, Cursor};
use std::{error, io::Read, str::FromStr};

use strum_macros::EnumString;

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
const DBL_QUOTE: u8 = "\"".as_bytes()[0];

#[derive(Debug, Clone, Copy, EnumString, strum_macros::Display, PartialEq)]
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

#[derive(Debug, Clone, Copy, EnumString, strum_macros::Display, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    ConstString(String),
    ConstInt(i16),
    None,
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
                if curr_symbol == Symbol::FwdSlash
                    || curr_symbol == Symbol::Asterisk
                        && (next_symbol == Symbol::FwdSlash || next_symbol == Symbol::Asterisk)
                {
                    read_byte(stream).unwrap();
                    return Ok(Token::Keyword(
                        Keyword::from_str(&concat_string!(
                            curr_symbol.to_string(),
                            next_symbol.to_string()
                        ))
                        .unwrap(),
                    ));
                } else {
                    return Ok(Token::Symbol(curr_symbol));
                }
            }
            if curr_symbol == Symbol::DblQuote {
                // string constants, treats the whole constant as 1 token
                let mut buff = Vec::new();
                buff.push(DBL_QUOTE);
                stream.read_until(DBL_QUOTE, &mut buff).unwrap();

                let const_string = std::string::String::from_utf8(buff).unwrap();

                return Ok(Token::ConstString(const_string));
            }
            // if the next byte is not a symbol, return the current byte
            return Ok(Token::Symbol(curr_symbol));
        }

        token.push(character[0]);

        if let Ok(_) = next_res {
            break;
        }

        stream.read_exact(&mut character)?;
    }

    Ok(get_token_type(std::str::from_utf8(&token).unwrap()))
}

/// Tries to match Token::Keyword or Token::ConstInt, then falls back to Token::Identifier
pub fn get_token_type(token: &str) -> Token {
    if let Ok(t) = Keyword::from_str(token) {
        return Token::Keyword(t);
    }
    if token.chars().nth(0).unwrap().is_numeric() {
        return Token::ConstInt(token.parse().unwrap());
    }
    Token::Identifier(token.to_owned())
}

pub fn expect_bytes(expected: &str, got: &[u8]) {
    let got_str = std::str::from_utf8(&got).unwrap();

    assert_eq!(got_str, expected);
}

pub fn xml_token(token: &Token) -> String {
    match token {
        Token::Keyword(t) => concat_string!("<keyword> ", t.to_string(), " </keyword>\n"),
        Token::Symbol(t) => concat_string!("<symbol> ", t.to_string(), " </symbol>\n"),
        Token::Identifier(t) => concat_string!("<identifier> ", t, " </identifier>\n"),
        Token::ConstString(t) => concat_string!("<stringConstant> ", t, " </stringConstant>\n"),
        Token::ConstInt(t) => {
            concat_string!("<integerConstant> ", t.to_string(), " </integerConstant>\n")
        }
        Token::None => panic!("Cannot create xml token for Token::None"),
    }
}
