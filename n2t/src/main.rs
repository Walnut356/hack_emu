#![allow(unused_imports)]

use n2t::hardware::logic_gate::cpu::Computer;
use n2t::software::assembler::*;
use n2t::software::vm::*;
use n2t::utils::*;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    let path = Path::new(r#"../ch 7 vm files/SimpleAdd.vm"#);
    translate(&path);
}
