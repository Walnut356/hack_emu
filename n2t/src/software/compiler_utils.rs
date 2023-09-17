#![allow(non_camel_case_types)]

use concat_string::concat_string;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Cursor, Write};
use std::sync::Mutex;
use std::{error, io::Read, str::FromStr};

use strum_macros::{EnumString, IntoStaticStr};

use crate::software::compiler::JackCompiler;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

impl JackCompiler {
    pub fn peek(&mut self) -> Result<[u8; 1]> {
        let mut buff = [0];
        self.stream.read_exact(&mut buff)?;

        self.stream.set_position(self.stream.position() - 1);

        Ok(buff)
    }

    pub fn peek_eq(&mut self, val: &str) -> Result<bool> {
        assert_eq!(val.len(), 1, "peek_eq requires a str input of length 1");
        let byte = self.peek().unwrap();

        if byte[0] == val.bytes().next().unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn read_byte(&mut self) -> Result<[u8; 1]> {
        let mut buff: [u8; 1] = [0];
        self.stream.read_exact(&mut buff)?;

        Ok(buff)
    }

    /// Accepts a stream, reads spaces until the first non-space character is found, returns a [u8; 1]
    /// containing the non-space character
    pub fn skip_spaces(&mut self) -> Result<[u8; 1]> {
        let mut character = [SPACE];
        while character == [SPACE] {
            self.stream.read_exact(&mut character)?;
        }

        Ok(character)
    }

    /// Accepts a stream, reads and discards until a newline character is found
    pub fn skip_to_newline(&mut self) {
        let mut character = [0];
        while character != [NEWLINE] {
            self.stream.read_exact(&mut character).unwrap();
        }
    }

    /// Accepts a stream, reads and discards characters (at least 1) until the first non-newline, non-space character is
    /// found. Returns that character.
    pub fn skip_whitespace(&mut self) -> Result<[u8; 1]> {
        let mut character = [SPACE];
        while character == [SPACE]
            || character == [NEWLINE]
            || character == [C_RETURN]
            || character == [TAB]
        {
            self.stream.read_exact(&mut character)?;
        }

        Ok(character)
    }

    pub fn skip_to_comment_end(&mut self) {
        let mut character = [0];

        while !(character == ["*".as_bytes()[0]] && self.peek_eq("/").unwrap()) {
            self.stream.read_exact(&mut character).unwrap();
        }
        self.read_byte().unwrap(); // consume the peeked '/' character
    }

    pub fn skip_comment(&mut self, com_type: Keyword) {
        match com_type {
            Keyword::Comment => self.skip_to_newline(),
            Keyword::MComment | Keyword::APIComment => self.skip_to_comment_end(),
            _ => panic!("Invalid Comment Type"),
        };
    }

    /// Reads characters until a space is found, returns a String containing the characters read
    /// WITHOUT a trailing space. Skips comments entirely.
    pub fn get_next_token(&mut self) -> Result<Token> {
        let mut character = self.skip_whitespace()?;
        let mut token = Vec::new();

        while character != [SPACE] && character != [NEWLINE] && character != [C_RETURN] {
            let next_byte = self.peek().unwrap();

            let next_res = Symbol::from_str(std::str::from_utf8(&next_byte).unwrap());

            // if we currently have a symbol
            if let Ok(curr_symbol) = Symbol::from_str(std::str::from_utf8(&character).unwrap()) {
                // and the next byte is a symbol
                if let Ok(next_symbol) = next_res {
                    // and the next symbol is forward slash or astersik, return "//" or "/*"
                    // this is kindof a hack due to ignoring API comments
                    if curr_symbol == Symbol::FwdSlash && next_symbol == Symbol::FwdSlash {
                        self.skip_comment(Keyword::Comment);
                        return self.get_next_token();
                    }
                    if curr_symbol == Symbol::FwdSlash && next_symbol == Symbol::Asterisk {
                        self.skip_comment(Keyword::MComment);
                        return self.get_next_token();
                    }
                    // and it's not a comment, return the symbol
                    return Ok(Token::Symbol(curr_symbol));
                }
                if curr_symbol == Symbol::DblQuote {
                    // string constants, treats the whole constant as 1 token
                    let mut buff = Vec::new();
                    self.stream.read_until(DBL_QUOTE, &mut buff).unwrap();
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

            self.stream.read_exact(&mut character)?;
        }

        Ok(get_token_type(std::str::from_utf8(&token).unwrap()))
    }

    /// Returns a Result containing a Tuple of the next **non-comment** Token, and the read position
    /// corresponding to the end of the peeked token. If the peeked token is used, calling
    /// `stream.set_position()` with the returned u64 will update the stream to the proper location
    /// without having to re-read the peeked token.
    pub fn peek_next_token(&mut self) -> Result<(Token, u64)> {
        let position = self.stream.position();

        let mut next_token = self.get_next_token()?;

        use Keyword::*;
        while next_token == Token::Keyword(Comment)
            || next_token == Token::Keyword(MComment)
            || next_token == Token::Keyword(APIComment)
        {
            if next_token == Token::Keyword(Comment) {
                self.skip_to_newline();
            } else {
                self.skip_to_comment_end();
            }
            next_token = self.get_next_token()?
        }

        let post_pos = self.stream.position();
        self.stream.set_position(position);

        Ok((next_token, post_pos))
    }

    pub fn xml_token(&self, token: &Token) -> String {
        let indent = "  ".repeat(self.indent_depth);
        match token {
            Token::Keyword(t) => {
                concat_string!(indent, "<keyword> ", t.to_string(), " </keyword>\n")
            }
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
    pub fn open_xml_group(&mut self, group_name: &str) -> String {
        let temp = concat_string!(
            "  ".repeat(self.indent_depth),
            "<",
            group_name,
            ">\n"
        );
        self.indent_depth += 1;

        temp
    }

    /// returns a string closing an xml "group", also decrements the indent depth
    #[inline]
    pub fn close_xml_group(&mut self, group_name: &str) -> String {
        self.indent_depth -= 1;
        concat_string!(
            "  ".repeat(self.indent_depth),
            "</",
            group_name,
            ">\n"
        )
    }

    #[inline]
    pub fn write_token(&mut self, token: &Token) {
        write!(self.output, "{}", self.xml_token(token)).unwrap();
    }

    #[inline]
    pub fn write_xml(&mut self, name: &str, tag: bool) {
        if tag {
            let group = self.open_xml_group(name);
            write!(self.output, "{}", group).unwrap();
        } else {
            let group = self.close_xml_group(name);
            write!(self.output, "{}", group).unwrap();
        }
    }
}
