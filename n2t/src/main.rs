#![allow(unused_imports)]

use n2t::hardware::native::cpu::Computer;
use n2t::software::assembler::*;
use n2t::software::vm::*;
use n2t::utils::*;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    let path = Path::new(r#"..\test_files\ch 7\BasicTest.vm"#);
    let asm = vm_to_asm(&path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);

    let mut cpu = Computer::new(program);

    while cpu.execute(false, true) {
    }

    println!("{:?}", cpu.a);
}
