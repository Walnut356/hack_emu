use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::utils::{get_file_buffers, BuiltInFunc};

#[derive(Debug, Clone, Copy)]
pub enum Offset {
    Label(u16),
    Var(u16),
    BuiltIn(u16),
    OS,
}

impl Into<u16> for Offset {
    fn into(self) -> u16 {
        match self {
            Offset::Label(x) => x,
            Offset::Var(x) => x,
            Offset::BuiltIn(x) => x,
            _ => panic!("Cannot turn OS calls into u16's"),
        }
    }
}

/// Accepts a Path to a ".asm" file, returns a Path to the generated machine code file
/// with the ".hack" extension
pub fn asm_to_hack(path: &Path) -> PathBuf {
    let mut files = get_file_buffers(path, "asm");

    let buffer = files.pop().unwrap().0;

    let mut out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    out_path.set_extension("hack");
    let out_file = File::create(out_path.clone()).unwrap();
    let mut output = BufWriter::new(out_file);

    let mut symbol_table: HashMap<String, Offset> = HashMap::new();

    symbol_table.insert("SP".to_string(), Offset::BuiltIn(0));
    symbol_table.insert("LCL".to_string(), Offset::BuiltIn(1));
    symbol_table.insert("ARG".to_string(), Offset::BuiltIn(2));
    symbol_table.insert("THIS".to_string(), Offset::BuiltIn(3));
    symbol_table.insert("THAT".to_string(), Offset::BuiltIn(4));
    symbol_table.insert("R0".to_string(), Offset::BuiltIn(0));
    symbol_table.insert("R1".to_string(), Offset::BuiltIn(1));
    symbol_table.insert("R2".to_string(), Offset::BuiltIn(2));
    symbol_table.insert("R3".to_string(), Offset::BuiltIn(3));
    symbol_table.insert("R4".to_string(), Offset::BuiltIn(4));
    symbol_table.insert("R5".to_string(), Offset::BuiltIn(5));
    symbol_table.insert("R6".to_string(), Offset::BuiltIn(6));
    symbol_table.insert("R7".to_string(), Offset::BuiltIn(7));
    symbol_table.insert("R8".to_string(), Offset::BuiltIn(8));
    symbol_table.insert("R9".to_string(), Offset::BuiltIn(9));
    symbol_table.insert("R10".to_string(), Offset::BuiltIn(10));
    symbol_table.insert("R11".to_string(), Offset::BuiltIn(11));
    symbol_table.insert("R12".to_string(), Offset::BuiltIn(12));
    symbol_table.insert("R13".to_string(), Offset::BuiltIn(13));
    symbol_table.insert("R14".to_string(), Offset::BuiltIn(14));
    symbol_table.insert("R15".to_string(), Offset::BuiltIn(15));
    symbol_table.insert("SCREEN".to_string(), Offset::BuiltIn(16384));
    symbol_table.insert("KBD".to_string(), Offset::BuiltIn(24576));

    // ------------------------------- add labels to symbol table ------------------------------- //
    let mut first_pass: Vec<String> = Vec::new();
    let mut lines = buffer.lines();
    let mut executable_count: u32 = 0;

    while let Some(Ok(line)) = lines.next() {
        first_pass.push(parse_labels(
            line.clone(),
            &mut symbol_table,
            executable_count,
        ));
        if !(line.starts_with('(') || line.starts_with("//") || line.is_empty()) {
            executable_count += 1;
        }
    }

    // ----------------------------------------- codegen ---------------------------------------- //
    let mut second_pass = Vec::new();
    let mut var_counter = 16u16;

    let mut counter = 0;
    for line in first_pass {
        // dbg!(&line);
        // dbg!(second_pass.len());
        if let Some(instr) = parse_symbols(
            line,
            second_pass.len(),
            &mut var_counter,
            &mut symbol_table,
            &mut counter,
        ) {
            second_pass.push(instr);
        }
    }

    if second_pass.len() >= u16::MAX as usize {
        panic!("Program is longer than 64k and cannot be run on the hack cpu")
    }

    for instr in second_pass {
        write!(output, "{}", translate_instruction(instr, &symbol_table)).unwrap();
    }

    output.flush().unwrap();

    out_path
}

/// First pass of the assembler. Takes a single line of Hack VM code, trims it, and adds any labels
/// - e.g. "(xxx)" - to the symbol table
pub fn parse_labels(
    line: String,
    symbol_table: &mut HashMap<String, Offset>,
    line_count: u32,
) -> String {
    let mut trimmed = line.trim().to_owned();
    trimmed = trimmed
        .split_whitespace()
        .next()
        .unwrap_or(&trimmed)
        .to_owned();

    if trimmed.starts_with('(') {
        symbol_table.insert(
            trimmed[1..trimmed.len() - 1].to_string(),
            Offset::Label((line_count) as u16),
        );
    }
    trimmed
}

/// Parses a single line of Hack VM code, populates symbol table with non-label symbols. If the
/// line is not a comment or empty, returns the line.
pub fn parse_symbols(
    line: String,
    line_count: usize,
    var_counter: &mut u16,
    symbol_table: &mut HashMap<String, Offset>,
    counter: &mut usize,
) -> Option<String> {
    // I could probably use regex but it seems a bit excessive for something so constrained
    if line.starts_with("//") | line.is_empty() {
        return None;
    }

    if line.starts_with('(') {
        // symbol_table.insert(line[1..line.len() - 1].to_string(), line_count as u16);
        return None;
    }
    if line.starts_with('@') {
        let key = line.strip_prefix('@').unwrap();
        // if symbol isn't just a number
        if key.parse::<u16>().is_err() {
            match symbol_table.get(&key.to_string()) {
                // symbol is a label that occurs in the top 32K of rom
                Some(Offset::Label(x)) if *x >= 32768 => {
                    *counter += 1;
                    // see giant comment below for details.
                    for (k, v) in symbol_table.iter_mut() {
                        if let Offset::Label(val) = v {
                            if *val >= (line_count + *counter) as u16 {
                                if *val == 32767 {
                                    panic!("Label {k:?} crossed ROM boundary during handling of ASM -> machine code instruction insertion");
                                }
                                *val += 1;
                            }
                        }
                    }
                }
                // symbol has already been added
                Some(_) => (),
                // symbol is something else
                None => {
                    symbol_table.insert(
                        line.strip_prefix('@').unwrap().to_string(),
                        Offset::Var(*var_counter),
                    );
                    *var_counter += 1;
                    if *var_counter > 255 {
                        panic!("Too many static variables, overflowing into stack")
                    }
                }
            };
        }
        return Some(line.to_string());
    }
    Some(line.to_string())
}

/// Translates a single line of Hack VM code into its machine instruction counterpart (represented
/// as a string of 1's and 0's rather than a u16)
pub fn translate_instruction(instr: String, symbol_table: &HashMap<String, Offset>) -> Box<str> {
    let mut code: u16;

    // a instruction
    if instr.starts_with('@') {
        if let Some(&val) = symbol_table.get(instr.strip_prefix('@').unwrap()) {
            /* HACK this increases the addressable ROM range from 32k to 64k. Because we're
            inserting instructions into the generated machine code, our generated label:line
            mappings will drift out of sync with their actual loctions. The solution isn't entirely
            straightforward, and there's probably better ways to do it, but to change as little as
            possible i'll do the following:
            during the second pass (parse_symbols()), any time an @ instruction would reference a
            label whose value is higher than 32767, check every label, and if it occurs after the
            currently parsed line, increment it. Kinda gross and inefficient but oh well. Also
            definitely breaks if something starts out under the threshold, but is incremented over
            it.

            Definitely didn't take a day and a half of hairpulling to
            figure that one out =)
            */
            let num: u16 = val.into();
            if num < 32768 {
                return format!("{num:016b}\n").into();
            } else {
                let inverse = !num;

                return format!(
                    "{inverse:016b}\n{}",
                    translate_instruction("A=!A".into(), symbol_table),
                )
                .into();
            }
        } else {
            code = instr.strip_prefix('@').unwrap().parse::<u16>().unwrap();
            return format!("{code:016b}\n").into();
        }
    }

    // if instr.starts_with('B') {
    //     let func = instr.strip_prefix('B').unwrap();
    //     let func = BuiltInFunc::from_str(func)
    //         .unwrap_or_else(|_| panic!("{func} not a valid built in function name"));
    //     code = func as u16;

    //     return format!("{code:016b}\n").into();
    // }

    // c instruction
    code = 0b1110_0000_0000_0000;

    // jump
    if instr.contains(';') {
        match &instr[instr.len() - 3..] {
            "JGT" => code |= 0b0000_0000_0000_0001,
            "JEQ" => code |= 0b0000_0000_0000_0010,
            "JGE" => code |= 0b0000_0000_0000_0011,
            "JLT" => code |= 0b0000_0000_0000_0100,
            "JNE" => code |= 0b0000_0000_0000_0101,
            "JLE" => code |= 0b0000_0000_0000_0110,
            "JMP" => code |= 0b0000_0000_0000_0111,
            val => panic!("Invalid jump instruction: {val}"),
        }

        match instr.as_str().chars().next().unwrap() {
            '0' => code |= 0b0000_1010_1000_0000,
            'A' => code |= 0b0000_1100_0000_0000,
            'M' => code |= 0b0001_1100_0000_0000,
            'D' => code |= 0b0000_0011_0000_0000,
            val => panic!("Invalid jump comparitor: {val}"),
        }

        return format!("{code:016b}\n").into();
    }

    // everything else
    let (dest, src) = instr.split_once('=').unwrap();
    // determines whether to use A as a value or a pointer
    if src.contains('M') {
        code |= 0b0001_0000_0000_0000;
    }

    assert!(
        src.len() <= 3,
        "Comparison {} longer than 3 characters",
        src
    );
    assert!(
        dest.len() <= 3,
        "Destination {} longer than 3 characters",
        dest
    );

    match src {
        "0" => code |= 0b0000_1010_1000_0000,
        "1" => code |= 0b0000_1111_1100_0000,
        "-1" => code |= 0b0000_1110_1000_0000,
        "D" => code |= 0b0000_0011_0000_0000,
        "A" | "M" => code |= 0b0000_1100_0000_0000,
        "!D" => code |= 0b0000_0011_0100_0000,
        "!A" | "!M" => code |= 0b0000_1100_0100_0000,
        "-D" => code |= 0b0000_0011_1100_0000,
        "-A" | "-M" => code |= 0b0000_1100_1100_0000,
        "D+1" => code |= 0b0000_0111_1100_0000,
        "A+1" | "M+1" => code |= 0b0000_1101_1100_0000,
        "D-1" => code |= 0b0000_0011_1000_0000,
        "A-1" | "M-1" => code |= 0b0000_1100_1000_0000,
        "D+A" | "D+M" | "A+D" | "M+D" => code |= 0b0000_0000_1000_0000,
        "D-A" | "D-M" => code |= 0b0000_0100_1100_0000,
        "A-D" | "M-D" => code |= 0b0000_0001_1100_0000,
        "D&A" | "D&M" => code |= 0b0000_0000_0000_0000,
        "D|A" | "D|M" => code |= 0b0000_0101_0100_0000,
        val => panic!("Invalid comparison: {val}"),
    }

    if dest.contains('A') {
        code |= 0b0000_0000_0010_0000;
    }
    if dest.contains('D') {
        code |= 0b0000_0000_0001_0000;
    }
    if dest.contains('M') {
        code |= 0b0000_0000_0000_1000;
    }

    format!("{code:016b}\n").into()
}
