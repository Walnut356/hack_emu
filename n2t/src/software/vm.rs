#![allow(unused, non_camel_case_types)]

use crate::software::vm_instructions::*;
use crate::utils::get_file_buffer;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{clone, fmt};

const STACK_START: usize = 256;
const STACK_MAX: usize = 2047;

const STACK_POINTER: usize = 0;
const LCL: usize = 1;
const ARG: usize = 2;
const THIS: usize = 3;
const THAT: usize = 4;
const TEMP_START: usize = 5;
const TEMP_MAX: usize = 12;
const VAR_START: usize = 13;
const VAR_MAX: usize = 15;
const STATIC_START: usize = 16;
const STATIC_MAX: usize = 255;
const FILE_NAME: Mutex<String> = Mutex::new(String::new());

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    stack,
    lcl,
    arg,
    this,
    that,
    temp,
    none,
    other(String),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::stack => write!(f, "SP"),
            Segment::lcl => write!(f, "LCL"),
            Segment::arg => write!(f, "ARG"),
            Segment::this => write!(f, "THIS"),
            Segment::that => write!(f, "THAT"),
            Segment::temp => write!(f, "R"),
            Segment::none => write!(f, "None"),
            Segment::other(val) => write!(f, "{}", val.to_ascii_uppercase()),
        }
    }
}

/// Accepts a Path to a ".vm" file, returns a Path to the generated ".asm"
pub fn vm_to_asm(path: &Path) -> PathBuf {
    let mut buffer = get_file_buffer(path, "vm");
    let mut lines = buffer.lines();

    // Managing this with global state is easier than passing it around a bunch,
    // especially when many functions won't need it
    let val = path.file_stem().unwrap().to_str().unwrap().clone();
    *FILE_NAME.lock().unwrap() = val.clone().to_owned();

    let mut out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    out_path.set_extension("asm");

    let mut out_file = File::create(out_path.clone()).unwrap();

    let mut output = String::new();

    output.push_str(init_stack().as_str());

    let mut eq_count = 0;
    let mut lt_count = 0;
    let mut gt_count = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("//") | line.is_empty() {
            continue;
        }

        // for debug purposes
        output.push_str(format!("//{line}\n").as_str());

        let (instr, mut seg, val) = {
            let mut temp = line.split_whitespace();
            (
                temp.next().unwrap(),
                temp.next().unwrap_or("None"),
                temp.next(),
            )
        };

        if (instr, seg, val) == ("pop", "that", Some("5")) {
            println!("")
        }

        let loc = match seg {
            "constant" => Segment::stack,
            "local" => Segment::lcl,
            "argument" => Segment::arg,
            "this" => Segment::this,
            "that" => Segment::that,
            "temp" => Segment::temp,
            "None" => Segment::none,
            _ => Segment::other(seg.to_owned()),
        };

        match instr {
            "push" => output
                .push_str(push(loc, val.expect("Got push instruction with no value")).as_str()),
            "pop" => output.push_str(pop(loc, val).as_str()),
            "add" => output.push_str(add().as_str()),
            "sub" => output.push_str(sub().as_str()),
            "eq" => {
                output.push_str(eq(eq_count).as_str());
                eq_count += 1;
            }
            "lt" => {
                output.push_str(lt(lt_count).as_str());
                eq_count += 1;
            }
            "gt" => {
                output.push_str(gt(gt_count).as_str());
                eq_count += 1;
            }
            "and" => output.push_str(and().as_str()),
            "or" => output.push_str(or().as_str()),
            "not" => output.push_str(not().as_str()),
            "neg" => output.push_str(neg().as_str()),
            val => panic!("Invalid instruction {val}"),
        }
    }

    // force infinite loop to "terminate" program
    output.push_str("(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP");
    write!(out_file, "{output}").unwrap();
    out_file.flush().unwrap();

    out_path
}

/// sets pointer values "sensible defaults"
fn init_stack() -> String {
    format!("{}", "//init 'stack' pointer\n@256\nD=A\n@SP\nM=D\n",)
}
