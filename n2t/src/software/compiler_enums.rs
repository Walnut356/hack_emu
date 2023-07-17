use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, PartialEq)]
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
}

#[derive(Debug, EnumString, Display, PartialEq)]
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

#[derive(Debug, EnumString, Display, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum DType {
    Int,
    Boolean,
    Char,
    Void,
    #[strum(default)]
    UserDef(String),
}

#[derive(Debug, EnumString, Display, PartialEq)]
pub enum Token {
    Keyword(String),
    Symbol(String),
    Identifier(String),
    ConstString(String),
    ConstInt(i16),
}
