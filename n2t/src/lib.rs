/// logical implementations using only NAND chip + manually constructed chips
pub mod hardware {
    pub mod logic_gate {
        pub mod alu;
        pub mod arithmetic;
        pub mod cpu;
        pub mod gates;
        pub mod memory;
    }

    /// shortcut implementations in native rust to speed up processing
    pub mod native {
        pub mod alu;
        pub mod cpu;
        pub mod gates;
        pub mod instructions;
        pub mod memory;
        pub mod os;
    }
}

pub mod software {
    pub mod assembler;
    pub mod compiler;
    pub mod compiler_utils;
    pub mod tokenizer;
    pub mod tokenizer_utils;
    pub mod vm;
    pub mod vm_instructions;
    pub mod writer_impl;
}

pub mod utils;

pub const STACK_START: usize = 256;
pub const STACK_MAX: usize = 2047;

pub const HEAP_START: usize = 2048;

pub const STACK_POINTER: usize = 0;
pub const LCL: usize = 1;
pub const ARG: usize = 2;
pub const THIS: usize = 3;
pub const THAT: usize = 4;
pub const TEMP_START: usize = 5;
pub const TEMP_MAX: usize = 12;
pub const VAR_START: usize = 13;
pub const VAR_MAX: usize = 15;
pub const STATIC_START: usize = 16;
pub const STATIC_MAX: usize = 255;

pub const SCREEN_START: usize = 0x4000;
pub const SCREEN_END: usize = 0x6000;

pub const KEYBOARD: usize = 0x6000;

use std::{fs::File, path::PathBuf};

use bitvec::prelude::*;

pub fn pixels_from_bitplane(vals: &[u16], buffer: &mut [u32]) {
    let thing = BitVec::<_, Lsb0>::from_slice(vals);
    assert_eq!(thing.len(), 256 * 512);
    for (i, bit) in thing.into_iter().enumerate() {
        if bit {
            buffer[i] = 0x00000000;
        } else {
            buffer[i] = 0xFFFFFFFF;
        }
    }
    // for (i, val) in vals.iter().enumerate() {
    //     // dbg!(val);
    //     let temp = val.view_bits::<Lsb0>();
    //     for (j, bit) in temp.iter().enumerate() {
    //         // dbg!(&bit);
    //         if *bit {
    //             buffer[i + j] = 0x00000000;
    //         } else {
    //             buffer[i + j] = 0xFFFFFFFF;
    //         }
    //     }
    // }
}

pub fn u16_to_u8_array(vals: &mut [u16]) -> &mut [u8] {
    let len = vals.len().checked_mul(2).unwrap();
    let ptr: *mut u8 = vals.as_mut_ptr().cast();
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

use hardware::native::cpu::Computer;
use software::{assembler::asm_to_hack, compiler::JackCompiler, vm::vm_to_asm};
use std::io::Write;
use utils::{decode_instr, hack_to_vec};

#[derive(Debug)]
pub struct HackEmulator {
    pub program: PathBuf,
    // pub instr: Vec<String>,
    pub cpu: Computer,
}

impl HackEmulator {
    /// Accepts a path to a .jack file or a folder containing .jack files.
    pub fn new(program: PathBuf) -> Self {
        let vm_path = JackCompiler::compile(&program);
        let asm_path = vm_to_asm(&vm_path);
        let hack_path = asm_to_hack(&asm_path);
        let machine_code = hack_to_vec(&hack_path);
        // let instr = machine_code
        //     .iter()
        //     .map(|x| decode_instr(*x, &[0, 0, 0]))
        //     .collect::<Vec<_>>();

        // let mut temp = File::create("./temp.asm").unwrap();
        // for line in instr {
        //     writeln!(temp, "{}", line).unwrap();
        // }
        let computer = Computer::new(machine_code);

        HackEmulator {
            program,
            cpu: computer,
            // instr,
        }
    }

    pub fn get_screen(&self) -> &[u16] {
        &self.cpu.ram[0x4000..0x6000]
    }

    pub fn get_keyboard(&self) -> &u16 {
        &self.cpu.ram[0x6000]
    }

    pub fn set_keyboard(&mut self, key_code: u16) {
        self.cpu.ram[0x6000] = key_code
    }
}
