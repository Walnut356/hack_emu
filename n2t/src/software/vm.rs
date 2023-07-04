#![allow(unused)]

use crate::software::vm_instructions::*;
use crate::utils::get_file_buffer;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;
use std::{clone, fmt};
use strum_macros::EnumString;

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

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    #[strum(to_string = "SP")]
    Constant,
    #[strum(to_string = "LCL")]
    Local,
    #[strum(to_string = "ARG")]
    Argument,
    #[strum(to_string = "THIS")]
    This,
    #[strum(to_string = "THAT")]
    That,
    #[strum(to_string = "R")]
    Temp,
    // None,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Instruction {
    Pop,
    Push,
    Add,
    Sub,
    Eq,
    Lt,
    Gt,
    Neg,
    Not,

    Label,
    Static,
    Func,
    ReturnSymbol,
}

/// Accepts a Path to a ".vm" file, returns a Path to the generated ".asm"
pub fn vm_to_asm(path: &Path) -> PathBuf {
    let mut out_path = PathBuf::from(path);

    // Create a container to hold file buffers, allowing the function to handle single files or full directories
    let mut files = Vec::new();

    if path.is_dir() {
        let mut file_list = path.read_dir().unwrap();
        while let Some(Ok(file)) = file_list.next() {
            let f_path = file.path();
            if f_path.ends_with(".vm") {
                files.push((
                    get_file_buffer(&f_path, "vm"),
                    f_path.parent().unwrap().to_str().unwrap().to_owned(),
                ));
            }
        }
    } else {
        files.push((
            get_file_buffer(path, "vm"),
            path.file_stem() // there literally has to be a better way, right?
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        ));
        let mut out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    }

    // Init output .asm file
    out_path.set_extension("asm");
    let mut out_file = File::create(out_path.clone()).unwrap();
    let mut output = String::new();
    output.push_str(init_program().as_str());

    // helper variables for unique labels
    let mut eq_count = 0;
    let mut lt_count = 0;
    let mut gt_count = 0;

    // Iterate over
    for (file, f_path) in files {
        let mut lines = file.lines();

        // Managing this with global state is easier (lol) than passing it around a bunch, especially when many funcs
        // don't need it. I don't plan on multithreading, so it shouldn't be a problem.
        *FILE_NAME.lock().unwrap() = f_path.to_owned();

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

            let loc = match seg {
                "constant" => Segment::Constant,
                "local" => Segment::Local,
                "argument" => Segment::Argument,
                "this" => Segment::This,
                "that" => Segment::That,
                "temp" => Segment::Temp,
                _ => Segment::Other(seg.to_owned()),
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
    }

    // force infinite loop to "terminate" program
    output.push_str(finalize_program());
    write!(out_file, "{output}").unwrap();
    out_file.flush().unwrap();

    out_path
}

/// sets pointer values "sensible defaults"
fn init_program() -> String {
    format!("{}", "//init 'stack' pointer\n@256\nD=A\n@SP\nM=D\n",)
    // TODO call Sys.Init
}

fn finalize_program() -> &'static str {
    "(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP"
}
