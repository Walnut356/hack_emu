#![allow(unused)]

use crate::software::vm_instructions::*;
use crate::utils::get_file_buffers;
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

// TODO use box str instead of String?

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
pub enum Segment {
    #[strum(to_string = "SP")]
    #[strum(serialize = "constant")]
    Stack,
    #[strum(to_string = "LCL")]
    #[strum(serialize = "local")]
    Local,
    #[strum(to_string = "ARG")]
    #[strum(serialize = "argument")]
    Argument,
    #[strum(to_string = "THIS")]
    #[strum(serialize = "this")]
    This,
    #[strum(to_string = "THAT")]
    #[strum(serialize = "that")]
    That,
    #[strum(to_string = "R")]
    #[strum(serialize = "temp")]
    Temp,
    #[strum(serialize = "pointer")]
    Pointer,
    #[strum(default)]
    Literal(String),
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
    And,
    Or,

    Label,
    Goto,
    #[strum(serialize = "if-goto")]
    IfGoto,
    Function,
    Call,
    Return,
}

/// Accepts a Path to a ".vm" file, returns a Path to the generated ".asm"
pub fn vm_to_asm(path: &Path) -> PathBuf {
    let mut out_path;

    if path.is_file() {
        out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    } else {
        out_path = Path::new(path).join(path.file_stem().unwrap());
    }

    let mut files = get_file_buffers(path, "vm");

    // Init output .asm file
    out_path.set_extension("asm");
    let mut out_file = File::create(out_path.clone()).unwrap();
    let mut output = String::new();
    output.push_str(BOOTSTRAP.as_str());

    // helper variables for unique labels
    let mut eq_count = 0;
    let mut lt_count = 0;
    let mut gt_count = 0;
    let mut ret_counts: HashMap<String, usize> = HashMap::new();

    // Iterate over
    for (file, f_name) in files {
        let mut lines = file.lines();

        // Managing this with global state is easier (lol) than passing it around a bunch, especially when many funcs
        // don't need it. I don't plan on multithreading, so it shouldn't be a problem.
        let file_name = f_name.to_owned();
        // let mut func_label = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.starts_with("//") | line.is_empty() {
                continue;
            }

            // record vm instruction as comment for debug purposes
            output.push_str(format!("//{line}\n").as_str());

            let mut temp = line.split_whitespace();
            let instr = Instruction::from_str(temp.next().unwrap())
                .expect(format!("Invalid instruction: {}", line).as_str());

            {
                use Instruction::*;

                match instr {
                    Pop => {
                        // 2 tokens: pointer and offset
                        let target = Segment::from_str(
                            temp.next().expect("Pop instruction with no location"),
                        )
                        .unwrap(); // the default should mean this never fails
                        let val = temp.next();

                        output.push_str(&pop(target, val))
                    }
                    Push => {
                        // TODO do statics get a unique name?
                        // 2 tokens: pointer and offset (or "constant" and value)
                        let target = Segment::from_str(
                            temp.next().expect("Push instruction with no location"),
                        )
                        .unwrap(); // the default should mean this never fails
                        let val = temp.next();
                        output.push_str(&push(target, val))
                    }
                    Add => output.push_str(&ADD),
                    Sub => output.push_str(&SUB),
                    Eq => {
                        output.push_str(&eq(eq_count));
                        eq_count += 1;
                    }
                    Lt => {
                        output.push_str(&lt(lt_count));
                        lt_count += 1;
                    }
                    Gt => {
                        output.push_str(&gt(gt_count));
                        gt_count += 1;
                    }
                    Neg => output.push_str(&NEG),
                    Not => output.push_str(&NOT),
                    And => output.push_str(&AND),
                    Or => output.push_str(&OR),
                    // Flow control
                    Label => {
                        // label + file name
                        let l_name = temp.next().expect("Label instruction with no label name");
                        assert!(
                            !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                            "Labels must not start with a digit. Got: {}",
                            l_name
                        );
                        output.push_str(&format!("({})\n", l_name));
                    }
                    Goto => {
                        let l_name = temp.next().expect("Jump instruction with no label");
                        assert!(
                            !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                            "Labels must not start with a digit. Got: {}",
                            l_name
                        );

                        output.push_str(&jump(format!("{}", l_name)))
                    } // label + file name
                    IfGoto => {
                        // label + file name
                        let l_name = temp.next().expect("Jump instruction with no label name");
                        assert!(
                            !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                            "Labels must not start with a digit. Got: {}",
                            l_name
                        );
                        output.push_str(&jump_if_zero(format!("{}", l_name)));
                    }
                    Function => {
                        // function name + nVars
                        let l_name = temp.next().expect("Function definition with no name");
                        assert!(
                            !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                            "Function name must not start with a digit. Got: {}",
                            l_name
                        );

                        output.push_str(&label(l_name));
                        let n_vars: usize = temp
                            .next()
                            .expect("Function definition without nVars")
                            .parse()
                            .unwrap();

                        for _ in 0..n_vars {
                            output.push_str(&push(Segment::Stack, Some("0")))
                        }
                    }
                    Call => {
                        // function name + nArgs
                        let l_name = temp.next().expect("Function call with no name");
                        assert!(
                            !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                            "Function name must not start with a digit. Got: {}",
                            l_name
                        );
                        let func_name = format!("{l_name}");

                        let n_args = temp.next().expect("Function Call with no Arg count");

                        let c = ret_counts.entry(func_name.clone()).or_default();
                        let return_addr = format!("{func_name}$ret{c}");
                        output.push_str(&func_call(&func_name, &return_addr, n_args));
                        *c += 1;
                    }
                    Return => {
                        output.push_str(&func_return());
                    } // 0 tokens
                }
            }
        }
    }

    write!(out_file, "{output}").unwrap();
    out_file.flush().unwrap();

    out_path
}
