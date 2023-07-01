#![allow(unused)]

use crate::utils::get_file_buffer;
use lazy_static::lazy_static;
use std::clone;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pointers {
    stack: u16,
    local: u16,
    arg: u16,
    this: u16,
    that: u16,
    temp: u16,
    stat: u16,
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

    // set stack pointer to default location at the start of every program
    // other people's solutions dont, but it seems like its the expectation
    output.push_str("//init stack pointer\n@256\nD=A\n@SP\nM=D\n");

    let mut ptrs = Pointers::default();

    for line in buffer.lines() {
        if line.starts_with("//") | line.is_empty() {
            continue;
        }

        // for debug purposes
        output.push_str(format!("//{line}\n").as_str());

        let (instr, loc, val) = {
            let mut temp = line.split_whitespace();
            (
                temp.next().unwrap(),
                temp.next().unwrap_or("None"),
                temp.next().unwrap_or("None"),
            )
        };

        match instr {
            "push" => output.push_str(push(loc, val, &mut ptrs).as_str()),
            "pop" => output.push_str(pop(loc).as_str()),
            "add" => output.push_str(add().as_str()),
            val => panic!("Invalid instruction {val}"),
        }
    }

    // force infinite loop to "terminate" program
    output.push_str("(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP");
    write!(out_file, "{output}").unwrap();
    out_file.flush().unwrap();

    out_path
}

fn push<'a>(loc: &str, val: &str, ptrs: &mut Pointers) -> String {
    let mut result = String::new();

    match loc {
        "constant" => {
            if ptrs.stack == u16::MAX {
                panic!("Stack overflow")
            }
            ptrs.stack += 1;
            result.push_str(
                format!("{}{}{}", set_d_const(val), set_mem_d("SP"), incr_ptr("SP")).as_str(),
            )
        }
        "local" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("LCL")).as_str()),
        "argument" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("ARG")).as_str()),
        "this" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("THIS")).as_str()),
        "that" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("THAT")).as_str()),
        "temp" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("TEMP")).as_str()),
        "static" => result.push_str(format!("{}{}", set_d_const(val), set_mem_d("STATIC")).as_str()),
        val => panic!("Invalid push location: {val}"),
    }

    result
}

fn pop(loc: &str) -> String {
    format!("{}{}", decr_ptr(loc), set_d_mem(loc))
}

fn add() -> String {
    format!("{}{}{}M=D+M\n{}", pop("SP"), decr_ptr("SP"), set_a_ptr("SP"), incr_ptr("SP"))
}

// Drop-in instructions for use in compound statements

fn set_a_ptr(loc: &str) -> String {
    format!("@{loc}\nA=M\n")
}

fn set_mem_d(loc: &str) -> String {
    format!("{}M=D\n", set_a_ptr(loc))
}

fn set_d_mem(loc: &str) -> String {
    format!("{}D=M\n", set_a_ptr(loc))
}

fn set_d_const(val: &str) -> String {
    format!("@{val}\nD=A\n")
}

fn incr_ptr(ptr: &str) -> String {
    format!("@{ptr}\nM=M+1\n")
}

fn decr_ptr(ptr: &str) -> String {
    format!("@{ptr}\nM=M-1\n")
}
