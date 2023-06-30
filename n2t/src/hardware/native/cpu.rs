use enumflags2::{bitflags, BitFlags};

use crate::utils::decode_instr;

use super::alu::ALU;

#[derive(Debug, PartialEq)]
pub enum InstrType {
    A,
    C,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlBits {
    // in
    ZeroX = 0b1000_0000,
    NotX = 0b0100_0000,
    ZeroY = 0b0010_0000,
    NotY = 0b0001_0000,
    FSelect = 0b0000_1000,
    NotOut = 0b0000_0100,

    // out
    Zero = 0b0000_0010,
    Neg = 0b0000_00001,
}

// unscientific benchmark shows a single instruction takes ~200ns release/1100ns debug which is ~5mhz release/.9mhz release
#[derive(Debug)]
pub struct Computer {
    pub d: u16,
    pub a: u16,
    pub pc: u16,
    pub ram: Vec<u16>,
    pub rom: Vec<u16>,
    pub flags: BitFlags<ControlBits>,
    pub time: usize,
    pub alu_out: u16,
    pub m_in: u16,
}

impl Computer {
    pub fn new(program: Vec<u16>) -> Self {
        Computer {
            d: 0,
            a: 0,
            pc: 0,
            ram: vec![0; 32786],
            rom: program,
            flags: BitFlags::default(),
            time: 0,
            alu_out: 0,
            m_in: 0,
        }
    }

    pub fn execute(&mut self, reset: bool, log: bool) {
        self.time += 1;
        let instr = self.rom[self.pc as usize];
        // form: [i, i, i, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3]

        if log {
            println!("Cycle: {}, PC: {}, inst: {:b}", self.time, self.pc, instr);
            decode_instr(instr);
            println!(
                "a register: {}, d register: {}, alu_out: {}, out_m: {}\n",
                self.a, self.d, self.alu_out, self.m_in
            );
        }

        let out_bits = self.flags.bits() & 0b0000_0011;
        let in_bits = ((instr & 0b0000_1111_1100_0000) >> 4) as u8;
        self.flags = BitFlags::from_bits(out_bits | in_bits).unwrap();

        let instr_type = match instr < 0b1000_0000_0000_0000 {
            true => InstrType::A,
            false => InstrType::C,
        };

        if instr_type == InstrType::A {
            self.a = instr;
        }
        if (instr_type == InstrType::C) & (instr & 0b0000_0000_0010_0000 > 0) {
            self.a = self.alu_out;
        }

        let input = match (0b0001_0000_0000_0000 & instr) == 0 {
            true => self.a,
            false => self.m_in,
        };

        self.alu_out = ALU(self.d, input, &mut self.flags);

        if (instr_type == InstrType::C) & ((instr & 0b0000_0000_0001_0000) > 0) {
            self.d = self.alu_out
        }

        let addr = (self.a & 0b0111_1111_1111_1111) as usize;

        if (instr_type == InstrType::C) & ((instr & 0b0000_0000_0000_1000) > 0) {
            self.ram[addr] = self.alu_out;
        }
        self.m_in = self.ram[addr];

        let mut should_jump = false;

        if instr_type == InstrType::C {
            let neg = self.flags.contains(ControlBits::Neg);
            let zero = self.flags.contains(ControlBits::Zero);
            should_jump = match instr & 0b0000_0000_0000_0111 {
                0 => false,
                1 => !neg,
                2 => zero,
                3 => !neg | zero,
                4 => neg & !zero,
                5 => !zero,
                6 => neg,
                7 => true,
                _ => panic!("somehow got a number higher than 7 on a bitwise AND with 7"),
            }
        }

        self.pc += 1;
        if should_jump {
            self.pc = self.a;
        }
        if reset {
            self.pc = 0
        }
    }
}
