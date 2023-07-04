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

    /// executes the instruction pointed to by `self.pc` in `self.rom`.
    ///
    /// Returns true if execution should continue, returns false if an infinite loop is hit and execution should reset
    /// or terminate.
    pub fn execute(&mut self, reset: bool, log: bool) -> bool {
        self.time += 1;
        let instr = self.rom[self.pc as usize];
        // form: [i, i, i, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3]

        if log {
            println!(
                "\na register: {}, d register: {}, alu_out: {}, out_m: {}\nRAM[A]: {}, Stack Ptr: {}, RAM[Stack Ptr]: {}\n",
                self.a, self.d, self.alu_out, self.m_in, self.ram[self.a as usize], self.ram[0], self.ram[self.ram[0] as usize]
            );
            println!(
                "Cycle: {}, PC: {}, inst: {:016b}",
                self.time, self.pc, instr
            );
            decode_instr(instr, &[self.a, self.d, self.ram[self.a as usize]]);
        }

        if (instr == 0b1110101010000111) && (self.a == self.pc - 1) {
            return false;
        };


        let out_bits = self.flags.bits() & 0b0000_0011;
        let in_bits = ((instr & 0b0000_1111_1100_0000) >> 4) as u8;
        self.flags = BitFlags::from_bits(out_bits | in_bits).unwrap();

        let instr_type = match instr < 0b1000_0000_0000_0000 {
            true => InstrType::A,
            false => InstrType::C,
        };

        if instr_type == InstrType::A {
            self.a = instr;
            self.pc += 1;
            return true;
        }

        let input = match (0b0001_0000_0000_0000 & instr) == 0 {
            true => self.a,
            false => self.ram[(self.a & 0b0111_1111_1111_1111) as usize],
        };

        // calc
        self.alu_out = ALU(self.d, input, &mut self.flags);

        // set output values
        let addr = (self.a & 0b0111_1111_1111_1111) as usize;

        if (instr_type == InstrType::C) && ((instr & 0b0000_0000_0000_1000) > 0) {
            self.ram[addr] = self.alu_out;
        }

        if (instr_type == InstrType::C) && ((instr & 0b0000_0000_0001_0000) > 0) {
            self.d = self.alu_out
        }

        // this needs to happen after RAM is updated, otherwise the target RAM address is incorrect
        if (instr_type == InstrType::C) && (instr & 0b0000_0000_0010_0000 > 0) {
            self.a = self.alu_out;
        }

        self.m_in = self.ram[(self.a & 0b0111_1111_1111_1111) as usize];

        // jump check
        let mut should_jump = false;

        if instr_type == InstrType::C {
            let neg = self.flags.contains(ControlBits::Neg);
            let zero = self.flags.contains(ControlBits::Zero);
            should_jump = match instr & 0b0000_0000_0000_0111 {
                0 => false, // Never jump
                1 => !neg & !zero, // If comp > 0 (JGT)
                2 => zero, // If comp = 0 (JEQ)
                3 => !neg, // If comp >= 0 (JGE)
                4 => neg & !zero, // If comp < 0 (JLT)
                5 => !zero, // If comp != 0 (JNE)
                6 => neg, // If comp <= 0 (JLE)
                7 => true, // Unconditional jump
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

        return true;
    }

    /// Executes until self.pc = pc_stop
    pub fn run_until(&mut self, pc_stop: u16, reset: bool, log: bool) -> bool {
        assert!(pc_stop <= self.rom.len() as u16);
        let mut cont = false;
        while self.pc != pc_stop {
            cont = self.execute(reset, log);
        }
        cont
    }
}
