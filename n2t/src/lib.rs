/// logical implementations using only NAND chip + manually constructed chips
pub mod hardware {
    pub mod logic_gate {
        pub mod alu;
        pub mod arithmetic;
        pub mod cpu;
        pub mod gates;
        pub mod memory;
    }

    /// shortcut implementations in native rust to speed up processing
    pub mod native {
        pub mod alu;
        pub mod cpu;
        pub mod gates;
        pub mod instructions;
        pub mod memory;
    }
}

pub mod software {
    pub mod assembler;
    pub mod compiler;
    pub mod compiler_utils;
    pub mod tokenizer;
    pub mod tokenizer_utils;
    pub mod vm;
    pub mod vm_instructions;
    pub mod writer_impl;
}

pub mod utils;

pub const STACK_START: usize = 256;
pub const STACK_MAX: usize = 2047;

pub const STACK_POINTER: usize = 0;
pub const LCL: usize = 1;
pub const ARG: usize = 2;
pub const THIS: usize = 3;
pub const THAT: usize = 4;
pub const TEMP_START: usize = 5;
pub const TEMP_MAX: usize = 12;
pub const VAR_START: usize = 13;
pub const VAR_MAX: usize = 15;
pub const STATIC_START: usize = 16;
pub const STATIC_MAX: usize = 255;

use strum_macros::FromRepr;
use strum_macros::{Display, EnumString};

#[allow(non_camel_case_types)]
#[derive(Debug, Display, EnumString, FromRepr)]
#[repr(u8)]
pub enum Key {
    // Non-extended ascii
    DEL = 127,
    NewLine = 128,
    BackSpace = 129,
    Left = 130,
    Up = 131,
    Right = 132,
    Down = 133,
    Home = 134,
    End = 135,
    PageUp = 136,
    PageDown = 137,
    Insert = 138,
    Delete = 139,
    Esc = 140,
    F1 = 141,
    F2 = 142,
    F3 = 143,
    F4 = 144,
    F5 = 145,
    F6 = 146,
    F7 = 147,
    F8 = 148,
    F9 = 149,
    F10 = 150,
    F11 = 151,
    F12 = 152,

    // Probably won't need these since it's just ASCII encodings
    #[strum(serialize = " ")]
    Space = 32,
    #[strum(serialize = "!")]
    Exclam = 33,
    #[strum(serialize = "\"")]
    DblQuote = 34,
    #[strum(serialize = "#")]
    Pound = 35,
    #[strum(serialize = "$")]
    Dollar = 36,
    #[strum(serialize = "%")]
    Mod = 37,
    #[strum(serialize = "&")]
    And = 38,
    #[strum(serialize = "'")]
    SglQuote = 39,
    #[strum(serialize = "(")]
    ParenOp = 40,
    #[strum(serialize = ")")]
    ParenCl = 41,
    #[strum(serialize = "*")]
    Asterisk = 42,
    #[strum(serialize = "+")]
    Plus = 43,
    #[strum(serialize = ",")]
    Comma = 44,
    #[strum(serialize = "-")]
    Minus = 45,
    #[strum(serialize = ".")]
    Period = 46,
    #[strum(serialize = "/")]
    FwdSlash = 47,
    #[strum(serialize = "0")]
    Zero = 48,
    #[strum(serialize = "1")]
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Colon = 58,
    SemiColon = 59,
    LessThan = 60,
    Equals = 61,
    GreaterThan = 62,
    Question = 63,
    At = 64,
    // A = 65,
    // B,
    // C,
    // D,
    // E,
    // F,
    // G,
    // H,
    // I,
    // J,
    // K,
    // L,
    // M,
    // N,
    // O,
    // P,
    // Q,
    // R,
    // S,
    // T,
    // U,
    // V,
    // W,
    // X,
    // Y,
    // Z,
    BracketOp,
    FwdSlash2,
    BracketCl,
    Carrat,
    Underscore,
    BackTick,
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    BraceOp,
    Pipe,
    BraceCl,
    Tilde,
}
