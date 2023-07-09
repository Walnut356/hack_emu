use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::utils::get_file_buffers;

/// Accepts a Path to a ".asm" file, returns a Path to the generated machine code file
/// with the ".hack" extension
pub fn asm_to_hack(path: &Path) -> PathBuf {
    let mut files = get_file_buffers(path, "asm");

    let buffer = files.pop().unwrap().0;

    let mut symbol_table: HashMap<String, u16> = HashMap::new();

    let mut i = 0u16;
    while i < 16 {
        symbol_table.insert(format!("R{i:?}"), i);
        i += 1;
    }
    symbol_table.insert("SP".to_string(), 0);
    symbol_table.insert("LCL".to_string(), 1);
    symbol_table.insert("ARG".to_string(), 2);
    symbol_table.insert("THIS".to_string(), 3);
    symbol_table.insert("THAT".to_string(), 4);
    symbol_table.insert("SCREEN".to_string(), 16384);
    symbol_table.insert("KBD".to_string(), 24576);

    let mut var_counter = 16u16;
    let mut first_pass: Vec<String> = Vec::new();

    let mut lines = buffer.lines();

    while let Some(Ok(line)) = lines.next() {
        // it's kinda dumb, but i have to do this otherwise the var counter drifts forward due to A instructions
        // that represent jump labels that haven't been defined yet.
        let mut trimmed = line.trim().to_owned();
        trimmed = trimmed
            .split_whitespace()
            .next()
            .unwrap_or(&trimmed)
            .to_owned();

        if trimmed.starts_with('(') {
            symbol_table.insert(trimmed[1..trimmed.len() - 1].to_string(), 0u16);
        }
        first_pass.push(trimmed);
    }

    let mut second_pass = Vec::new();

    for line in first_pass {
        // I could probably use regex but it seems a bit excessive for something so constrained
        if line.starts_with("//") | line.is_empty() {
            continue;
        }

        if line.starts_with('(') {
            symbol_table.insert(
                line[1..line.len() - 1].to_string(),
                second_pass.len() as u16,
            );
            continue;
        }
        if line.starts_with('@') {
            second_pass.push(line.to_string());
            if let Ok(_num) = line[1..].parse::<u16>() {
                continue;
            } else {
                match symbol_table.get(&line[1..].to_string()) {
                    Some(_) => continue,
                    None => {
                        symbol_table.insert(line[1..].to_string(), var_counter);
                        var_counter += 1;
                    }
                };
            }
            continue;
        }
        second_pass.push(line.to_string());
    }

    let mut out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    out_path.set_extension("hack");

    let mut out_file = File::create(out_path.clone()).unwrap();

    let mut output = String::new();

    for instr in second_pass {
        let mut code: u16;

        // a instruction
        if instr.starts_with("@") {
            if let Some(&val) = symbol_table.get(&instr[1..]) {
                // a little dumb, but the book specifies that it should be stored as text
                // the binary file parser in me is sad though
                code = val;
            } else {
                code = u16::from_ne_bytes(instr[1..].parse::<i16>().unwrap().to_ne_bytes());
            }
            output.push_str(format!("{code:016b}\n").as_str());
            continue;
        }
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

            match instr.as_str().chars().nth(0).unwrap() {
                '0' => code |= 0b0000_1010_1000_0000,
                'A' => code |= 0b0000_1100_0000_0000,
                'M' => code |= 0b0001_1100_0000_0000,
                'D' => code |= 0b0000_0011_0000_0000,
                val => panic!("Invalid jump comparitor: {val}"),
            }

            output.push_str(format!("{code:016b}\n").as_str());
            continue;
        }

        // everything else
        let (dest, comp) = instr.split_once('=').unwrap();
        // determines whether to use A as a value or a pointer
        if comp.contains('M') {
            code |= 0b0001_0000_0000_0000;
        }

        assert!(
            comp.len() <= 3,
            "Comparison {} longer than 3 characters",
            comp
        );
        assert!(
            dest.len() <= 3,
            "Destination {} longer than 3 characters",
            dest
        );

        match comp {
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

        output.push_str(format!("{code:016b}\n").as_str());
    }

    output.pop(); // removes trailing newline
    write!(out_file, "{output}").unwrap();
    out_file.flush().unwrap();

    out_path
}
