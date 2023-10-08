use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use strum_macros::FromRepr;
use strum_macros::{Display, EnumString};

pub fn u16_from_i16(val: i16) -> u16 {
    u16::from_ne_bytes(val.to_ne_bytes())
}

fn get_file_buffer(path: &Path, ext: &str) -> BufReader<File> {
    assert_eq!(
        path.extension().unwrap(),
        ext,
        "Expected file extension '{:?}', got file extension {:?}",
        ext,
        path.extension().unwrap()
    );

    let file = File::open(path).unwrap();

    BufReader::new(file)
}

/// Accepts a path and an extension. Returns a tuple of the file reader and the file's name (no
/// file extension or full path) .If the path is a directory, the returned Vec will contain multiple
/// elements, if it is a file, it will contain 1.
///
/// Panics if there are no files of the given extension
pub fn get_file_buffers(path: &Path, ext: &str) -> Vec<(BufReader<File>, String)> {
    let mut files = Vec::new();

    if path.is_dir() {
        let mut file_list = path.read_dir().unwrap();
        while let Some(Ok(file)) = file_list.next() {
            let f_path = file.path();
            if f_path.extension() == Some(OsStr::new(ext)) {
                files.push((
                    get_file_buffer(&f_path, ext),
                    f_path.file_stem().unwrap().to_str().unwrap().to_owned(),
                ));
            }
        }
    } else {
        files.push((
            get_file_buffer(path, ext),
            path.file_stem() // there literally has to be a better way, right?
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        ));
    }
    if files.is_empty() {
        panic! {"No files with extension {} in directory '{:?}'", ext, path}
    }
    files
}

pub fn hack_to_vec(path: &Path) -> Vec<u16> {
    let buffer = get_file_buffer(path, "hack");
    let mut program = Vec::new();

    let mut lines = buffer.lines();

    while let Some(Ok(line)) = lines.next() {
        program.push(u16::from_str_radix(&line, 2).expect("Not a binary number"))
    }
    program
}

// kinda disgusting but it'll do.
pub fn bitvec_from_int(mut int: u16) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..16 {
        result.push((int & 0b0000_0000_0000_0001) as u8);
        int >>= 1;
    }
    result.into_iter().rev().collect()
}

pub fn int_from_bitvec(vec: &Vec<u8>) -> u16 {
    let mut result: u16 = 0;
    for (i, j) in vec.iter().enumerate() {
        result |= (*j) as u16;
        if i < vec.len() - 1 {
            result <<= 1;
        }
    }
    result
}

pub fn decode_bitvec_instr(instr: &Vec<u8>) {
    // form: [i, i, i, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3]
    let a_or_c = match instr[0] {
        0 => {
            println!(
                "a instr: load value {} into A register",
                int_from_bitvec(instr)
            );
            return;
        }
        1 => "c instr",
        _ => "Error",
    };

    let a_or_m = match instr[3] {
        0 => "using value of A as val",
        1 => "using RAM[A] as val",
        _ => "Error",
    };

    let cmp = match instr[4..10] {
        [1, 0, 1, 0, 1, 0] => "0",
        [1, 1, 1, 1, 1, 1] => "1",
        [1, 1, 1, 0, 1, 0] => "-1",
        [0, 0, 1, 1, 0, 0] => "D",
        [1, 1, 0, 0, 0, 0] => "val",
        [0, 0, 1, 1, 0, 1] => "!D",
        [1, 1, 0, 0, 0, 1] => "!val",
        [0, 0, 1, 1, 1, 1] => "minus D",
        [1, 1, 0, 0, 1, 1] => "minus val",
        [0, 1, 1, 1, 1, 1] => "D + 1",
        [1, 1, 0, 1, 1, 1] => "val + 1",
        [0, 0, 1, 1, 1, 0] => "D - 1",
        [1, 1, 0, 0, 1, 0] => "val - 1",
        [0, 0, 0, 0, 1, 0] => "D + val",
        [0, 1, 0, 0, 1, 1] => "D - val",
        [0, 0, 0, 1, 1, 1] => "val - D",
        [0, 0, 0, 0, 0, 0] => "D & val",
        [0, 1, 0, 1, 0, 1] => "D | val",
        _ => "Error",
    };

    let store_in = match instr[10..13] {
        [0, 0, 0] => "None",
        [0, 0, 1] => "RAM[A]",
        [0, 1, 0] => "D",
        [0, 1, 1] => "D and RAM[A]",
        [1, 0, 0] => "A",
        [1, 0, 1] => "A and RAM[A]",
        [1, 1, 0] => "A and D",
        [1, 1, 1] => "A, D, and RAM[A]",
        _ => "Error",
    };

    let jump = match instr[13..=15] {
        [0, 0, 0] => "Never",
        [0, 0, 1] => "If greater than",
        [0, 1, 0] => "If equal",
        [0, 1, 1] => "If greater than or equal",
        [1, 0, 0] => "If less than",
        [1, 0, 1] => "If not equal",
        [1, 1, 0] => "If less than or equal",
        [1, 1, 1] => "Always",
        _ => "Error",
    };

    println!("{a_or_c}: {a_or_m}, compute '{cmp}' and store the value in {store_in}. {jump} jump to ROM[A].")
}

pub fn decode_instr(instr: u16, vars: &[u16]) -> String {
    if instr & 0b1000_0000_0000_0000 == 0 {
        let val = instr & 0b0111_1111_1111_1111;
        return match val {
            0 => "@SP".to_owned(),
            1 => "@LCL".to_owned(),
            2 => "@ARG".to_owned(),
            3 => "@THIS".to_owned(),
            4 => "@THAT".to_owned(),
            5 => "@R5".to_owned(),
            6 => "@R6".to_owned(),
            7 => "@R7".to_owned(),
            8 => "@R8".to_owned(),
            9 => "@R9".to_owned(),
            10 => "@R10".to_owned(),
            11 => "@R11".to_owned(),
            12 => "@R12".to_owned(),
            13 => "@R13".to_owned(),
            14 => "@R14".to_owned(),
            15 => "@R15".to_owned(),
            _ => format!("@{val}"),
        };
    }
    if let Some(func) = BuiltInFunc::from_repr(instr) {
        return func.to_string();
    }

    let a_or_m = match instr & 0b0001_0000_0000_0000 > 0 {
        false => "A",
        true => "M",
    };

    let cmp = match (instr & 0b0000_1111_1100_0000) >> 6 {
        0b0000_0000_0010_1010 => "0".to_owned(),
        0b0000_0000_0011_1111 => "1".to_owned(),
        0b0000_0000_0011_1010 => "-1".to_owned(),
        0b0000_0000_0000_1100 => "D".to_owned(),
        0b0000_0000_0011_0000 => a_or_m.to_owned(),
        0b0000_0000_0000_1101 => "!D".to_owned(),
        0b0000_0000_0011_0001 => format!("!{a_or_m}"),
        0b0000_0000_0000_1111 => "-D".to_owned(),
        0b0000_0000_0011_0011 => format!("-{a_or_m}"),
        0b0000_0000_0001_1111 => "D+1".to_owned(),
        0b0000_0000_0011_0111 => format!("{a_or_m}+1"),
        0b0000_0000_0000_1110 => "D-1".to_owned(),
        0b0000_0000_0011_0010 => format!("{a_or_m}-1"),
        0b0000_0000_0000_0010 => format!("D+{a_or_m}"),
        0b0000_0000_0001_0011 => format!("D-{a_or_m}"),
        0b0000_0000_0000_0111 => format!("{a_or_m}-D"),
        0b0000_0000_0000_0000 => format!("D & {a_or_m}"),
        0b0000_0000_0001_0101 => format!("D | {a_or_m}"),
        _ => "Error".to_owned(),
    };

    let store_in = match (instr & 0b0000_0000_0011_1000) >> 3 {
        0 => "",
        1 => "M=",
        2 => "D=",
        3 => "DM=",
        4 => "A=",
        5 => "AM=",
        6 => "AD=",
        7 => "ADM=",
        _ => "Error",
    };

    let jump = match instr & 0b0000_0000_0000_0111 {
        0 => "",
        1 => "; Jump if greater than 0",
        2 => "; Jump if equal to 0",
        3 => "; Jump if greater than or equal to 0",
        4 => "; Jump if less than 0",
        5 => "; Jump if not equal to 0",
        6 => "; Jump if less than or equal to 0",
        7 => "; Unconditional jump",
        _ => "Error",
    };
    let out = format!("{store_in}{cmp}{jump}");
    let mut temp_out = cmp.replace('A', &vars[0].to_string());
    temp_out = temp_out.replace('D', &vars[1].to_string());
    temp_out = temp_out.replace('M', &vars[2].to_string());

    let out2 = format!("{out} | {store_in}{temp_out}");

    out2
}

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

#[derive(Debug, EnumString, FromRepr, strum_macros::Display)]
#[repr(u16)]
pub enum BuiltInFunc {
    #[strum(serialize = "Math.multiply")]
    /// Accepts 2 int args, returns a * b
    MathMul = 0b1100_0000_0000_0000,
    #[strum(serialize = "Math.divide")]
    /// Accepts 2 int args, returns a / b
    MathDiv = 0b1100_0000_0000_0001,
    #[strum(serialize = "Math.min")]
    /// Accepts 2 int args, returns whichever is less
    MathMin = 0b1100_0000_0000_0010,
    #[strum(serialize = "Math.max")]
    /// Accepts 2 int args, returns whichever is greater
    MathMax = 0b1100_0000_0000_0011,
    #[strum(serialize = "Math.sqrt")]
    /// Accepts 1 int arg, returns the square root
    MathSqrt = 0b1100_0000_0000_0100,

    #[strum(serialize = "String.new")]
    StringNew = 0b1100_0010_0000_0000,
    #[strum(serialize = "String.dispose")]
    StringDispose = 0b1100_0010_0000_0001,
    #[strum(serialize = "String.length")]
    StringLength = 0b1100_0010_0000_0010,
    #[strum(serialize = "String.charAt")]
    StringCharAt = 0b1100_0010_0000_0011,
    #[strum(serialize = "String.setCharAt")]
    StringSetChar = 0b1100_0010_0000_0100,
    #[strum(serialize = "String.appendChar")]
    StringAppendChar = 0b1100_0010_0000_0101,
    #[strum(serialize = "String.eraseLastChar")]
    StringEraseLast = 0b1100_0010_0000_0110,
    #[strum(serialize = "String.intValue")]
    StringIntVal = 0b1100_0010_0000_0111,
    #[strum(serialize = "String.setInt")]
    StringSetInt = 0b1100_0010_0000_1000,
    #[strum(serialize = "String.backSpace")]
    StringBackspace = 0b1100_0010_0000_1001,
    #[strum(serialize = "String.doubleQuote")]
    StringDblQuote = 0b1100_0010_0000_1010,
    #[strum(serialize = "String.newLine")]
    StringNewline = 0b1100_0010_0000_1011,

    #[strum(serialize = "Array.new")]
    ArrayNew = 0b1100_0100_0000_0000,
    #[strum(serialize = "Array.dispose")]
    ArrayDispose = 0b1100_0100_0000_0001,

    #[strum(serialize = "Output.moveCursor")]
    OutputMoveCursor = 0b1100_0110_0000_0000,
    #[strum(serialize = "Output.printChar")]
    OutputPrintChar = 0b1100_0110_0000_0001,
    #[strum(serialize = "Output.printString")]
    OutputPrintString = 0b1100_0110_0000_0010,
    #[strum(serialize = "Output.printInt")]
    OutputPrintInt = 0b1100_0110_0000_0011,
    #[strum(serialize = "Output.println")]
    OutputPrintLn = 0b1100_0110_0000_0100,
    #[strum(serialize = "Output.backSpace")]
    OutputBackspace = 0b1100_0110_0000_0101,

    #[strum(serialize = "Screen.clearScreen")]
    ScreenClear = 0b1100_1000_0000_0000,
    #[strum(serialize = "Screen.setColor")]
    ScreenSetColor = 0b1100_1000_0000_0001,
    #[strum(serialize = "Screen.drawPixel")]
    ScreenDrawPixel = 0b1100_1000_0000_0010,
    #[strum(serialize = "Screen.drawLine")]
    ScreenDrawLine = 0b1100_1000_0000_0011,
    #[strum(serialize = "Screen.drawRectangle")]
    ScreenDrawRectangle = 0b1100_1000_0000_0100,
    #[strum(serialize = "Screen.drawCircle")]
    ScreenDrawCircle = 0b1100_1000_0000_0101,

    #[strum(serialize = "Keyboard.keyPressed")]
    KeyboardPressed = 0b1100_1010_0000_0000,
    #[strum(serialize = "Keyboard.readChar")]
    KeyboardReadChar = 0b1100_1010_0000_0001,
    #[strum(serialize = "Keyboard.readLine")]
    KeyboardReadLine = 0b1100_1010_0000_0010,
    #[strum(serialize = "Keyboard.readInt")]
    KeyboardReadInt = 0b1100_1010_0000_0011,

    #[strum(serialize = "Memory.peek")]
    MemPeek = 0b1100_1100_0000_0000,
    #[strum(serialize = "Memory.poke")]
    MemPoke = 0b1100_1100_0000_0001,
    #[strum(serialize = "Memory.alloc")]
    MemAlloc = 0b1100_1100_0000_0010,
    #[strum(serialize = "Memory.deAlloc")]
    MemDealloc = 0b1100_1100_0000_0011,

    #[strum(serialize = "Sys.Init")]
    SysInit = 0b1100_1110_0000_0000,
    #[strum(serialize = "Sys.halt")]
    SysHalt = 0b1100_1110_0000_0001,
    #[strum(serialize = "Sys.error")]
    SysError = 0b1100_1110_0000_0010,
    #[strum(serialize = "Sys.wait")]
    SysWait = 0b1100_1110_0000_0011,
}
