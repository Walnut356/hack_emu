#![allow(unused)]

use crate::utils::get_file_buffer;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{clone, fmt};

// TODO
// constants/memory mappings
// translate functions:
//  add, sub, neg
//  eq, gt, lt
//  and, or, not

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Segment {
    stack,
    lcl,
    arg,
    this,
    that,
    temp,
    none,
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
        }
    }
}

/// Accepts a Path to a ".vm" file, returns a Path to the generated ".asm"
pub fn vm_to_asm(path: &Path) -> PathBuf {
    let mut buffer = get_file_buffer(path, "vm");

    // Managing this with global state is easier than passing it around a bunch,
    // especially when many functions won't need it
    let val = path.file_stem().unwrap().to_str().unwrap().clone();
    *FILE_NAME.lock().unwrap() = val.clone().to_owned();

    let mut out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    out_path.set_extension("asm");

    let mut out_file = File::create(out_path.clone()).unwrap();

    let mut output = String::new();

    output.push_str(init_ptrs().as_str());

    let mut eq_count = 0;
    let mut lt_count = 0;
    let mut gt_count = 0;

    for line in buffer.lines() {
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
            _ => panic!("Invalid segment name"),
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
fn init_ptrs() -> String {
    format!(
        "{}{}{}{}{}",
        "//init stack pointer\n@256\nD=A\n@SP\nM=D\n",
        "//init local pointer\n@300\nD=A\n@LCL\nM=D\n",
        "//init argument pointer\n@400\nD=A\n@ARG\nM=D\n",
        "//init this pointer\n@3000\nD=A\n@THIS\nM=D\n",
        "//init that pointer\n@3010\nD=A\n@THAT\nM=D\n"
    )
}

/// pushes a value to the stack. If the location is a memory segment (e.g. "Local"), val is treated as an index into
/// the memory segment, and the value at the resultant memory location is pushed onto the stack.
fn push<'a>(loc: Segment, val: &str) -> String {
    let mut result = String::new();

    use Segment::*;
    match loc {
        stack => result
            .push_str(format!("{}{}{}", set_d_const(val), set_mem_d(loc), incr_ptr(loc)).as_str()),
        // base offset stored at RAM[1]
        _ => result.push_str(
            format!(
                "{}{}{}{}",
                set_a_offset(loc, val),
                "D=M\n",
                set_mem_d(Segment::stack),
                incr_ptr(Segment::stack)
            )
            .as_str(),
        ),
    }

    result
}

/// pops a value off of the stack and stores it in D if val = None, otherwise stores it in RAM[loc+val]
fn pop(loc: Segment, val: Option<&str>) -> String {
    if loc == Segment::stack {
        format!("{}{}", decr_ptr(loc), "D=M\n")
    } else {
        let ind = val.unwrap();
        format!(
            "{}{}{}{}{}{}{}{}",
            set_a_offset(loc, ind), // e.g. set A local[0]'s address
            "D=A\n",                // set D to local[0]'s address
            "@R13\n",
            "M=D\n",                   // store local[0]'s address in R13
            pop(Segment::stack, None), // pop stack into D
            "@R13\n",
            "A=M\n", // Access local[0]'s address from R13
            "M=D\n"  // set RAM[local[0]] to popped value
        )
    }
}

// binary ops
fn add() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::stack, None),
        decr_ptr(Segment::stack),
        "M=D+M\n",
        incr_ptr(Segment::stack)
    )
}

fn sub() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::stack, None),
        decr_ptr(Segment::stack),
        "M=M-D\n",
        incr_ptr(Segment::stack)
    )
}

fn and() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::stack, None),
        decr_ptr(Segment::stack),
        "M=D&M\n",
        incr_ptr(Segment::stack)
    )
}

fn or() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::stack, None),
        decr_ptr(Segment::stack),
        "M=D|M\n",
        incr_ptr(Segment::stack)
    )
}

// unary ops
fn not() -> String {
    // No need to manipulate the stack pointer when the value is being removed and put straight back on.
    format!("@SP\nA=M-1\nM=!M\n")
}

fn neg() -> String {
    format!("@SP\nA=M-1\nM=-M\n")
}

// comparisons
fn eq(eq_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@EQ_",
        eq_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(EQ_",
        eq_count,
    )
}

fn lt(lt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@LT_",
        lt_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(LT_",
        lt_count,
    )
}

fn gt(gt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@GT_",
        gt_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(GT_",
        gt_count,
    )
}

// Drop-in instructions for use in compound statements

/// sets A to a pointer's base address
fn set_a_ptr(loc: Segment) -> String {
    format!("@{loc}\nA=M\n")
}

/// sets A to `ind` offset of `loc` pointer's base address
fn set_a_offset(loc: Segment, ind: &str) -> String {
    // for the idiomatic "R0-R15" virtual registers
    if loc == Segment::temp {
        let off = ind.parse::<u16>().unwrap() + 5;
        format!("@{loc}{off}\n")
    } else {
        format!("@{ind}\nD=A\n@{loc}\nA=D+M\n")
    }
}

/// sets `RAM[loc]` to the value stored in `D`
fn set_mem_d(loc: Segment) -> String {
    format!("{}M=D\n", set_a_ptr(loc))
}

/// sets `D` to the value in `RAM[loc]`
fn set_d_mem(loc: Segment) -> String {
    format!("{}D=M\n", set_a_ptr(loc))
}

/// sets `D` to `val`
fn set_d_const(val: &str) -> String {
    format!("@{val}\nD=A\n")
}

/// leaves A as the post-incr memory location
fn incr_ptr(loc: Segment) -> String {
    format!("@{loc}\nAM=M+1\n")
}

/// leaves A as the post-decr memory location
fn decr_ptr(loc: Segment) -> String {
    format!("@{loc}\nAM=M-1\n")
}
