// TODO cpu
// A register
// D register
// InstrPtr
// ALU
// TODO Memory
// RAM16K (addr 0-16,383)
// screen (8K, addr 16,384-24,575)
// keyboard (16 bits, addr 24,576)
// TODO ROM32K

use crate::utils::*;

use super::alu::*;
use super::gates::*;
use crate::hardware::logic_gate::memory::{InstPtr, Register};
use crate::hardware::native::memory::RAM32K;

// unscientific benchmark puts the execution time of a single instruction at ~7800ns release/44,000ns debug, which is about 128khz release/23khz debug
pub struct Computer {
    pub a: Register,
    pub d: Register,
    pub pc: InstPtr,
    /// also called out_m
    pub alu_out: Vec<u8>,
    pub in_m: Vec<u8>,
    pub time: u32,
    pub flags: ControlBits,
    pub reset: bool,
    // I'm more or less required to use vec's for RAM/ROM as 64k Vec<u8>'s of size 16 is kindof a lot of memory.
    pub rom: Vec<u16>,
    pub ram: RAM32K,
}

impl Computer {
    pub fn new(program: Vec<u16>) -> Self {
        Computer {
            a: Register::new(),
            d: Register::new(),
            pc: InstPtr::new(),
            alu_out: vec![0; 16],
            in_m: vec![0; 16],
            time: 1,
            flags: ControlBits::new(),
            reset: false,
            rom: program,
            ram: RAM32K::new(),
        }
    }

    pub fn set_program(&mut self, program: Vec<u16>) {
        self.rom = program
    }

    /// executes the next instruction
    pub fn execute(&mut self, log: bool, reset: bool) {
        // ------------------------------------- Input and register updates ------------------------------------- //
        let instr = bitvec_from_int(self.rom[int_from_bitvec(&self.pc.val.data) as usize]);

        if log {
            println!(
                "a register: {}, d register: {}, self.alu_out: {}, out_m: {}",
                int_from_bitvec(&self.a.data),
                i16::from_ne_bytes(int_from_bitvec(&self.d.data).to_ne_bytes()),
                int_from_bitvec(&self.alu_out),
                self.ram.out,
            );
            println!(
                "cycle #: {}, pc: {}",
                self.time,
                int_from_bitvec(&self.pc.val.data)
            );
            decode_bitvec_instr(&instr);
        }

        self.time += 1;
        self.flags = ControlBits {
            zx: instr[4],
            nx: instr[5],
            zy: instr[6],
            ny: instr[7],
            f: instr[8],
            no: instr[9],
            zr: self.flags.zr,
            ng: self.flags.ng,
        };

        self.a.cycle(
            &multi_MUX(&instr, &self.alu_out, instr[0]),
            OR(NOT(instr[0]), AND(instr[0], instr[10])),
        );

        // -------------------------------------- alu processing and output ------------------------------------- //
        self.alu_out = ALU(
            &self.d.data,
            &multi_MUX(&self.a.data, &self.in_m, instr[3]),
            &mut self.flags,
        );

        self.d.cycle(&self.alu_out, AND(instr[0], instr[11]));

        // ick
        self.in_m = bitvec_from_int(self.ram.cycle(
            int_from_bitvec(&self.alu_out),
            int_from_bitvec(&self.a.data) & 0b0111_1111_1111_1111,
            AND(instr[0], instr[12]),
        ));

        let should_jump = AND(
            instr[0],
            MUX_8(
                0,
                NOT(self.flags.ng),
                self.flags.zr,
                OR(NOT(self.flags.ng), self.flags.zr),
                AND(self.flags.ng, NOT(self.flags.zr)),
                NOT(self.flags.zr),
                self.flags.ng,
                1,
                instr[15],
                instr[14],
                instr[13],
            ),
        );

        let mut should_reset = 0u8;
        if reset {
            should_reset = 1;
        }

        self.pc.cycle(&self.a.data, should_jump, 1, should_reset);

        //TODO screen & keyboard callback
    }
}
