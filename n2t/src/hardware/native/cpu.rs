use super::alu::ALU;
use crate::{
    hardware::native::os::{Block, OS},
    utils::{decode_instr, BuiltInFunc},
};
use enumflags2::{bitflags, BitFlags};
use prettytable::ptable;
use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum InstrType {
    /// Instructions used to load data into the A register
    ///
    /// Always starts with 0b1
    A,
    /// Custom instruction type used for OS function calls
    ///
    /// Always starts with 0b110
    B,
    /// Instructions meant to do some general computation, comparison, or jump
    ///
    /// always starts with 0b111
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
    pub os: OS,
}

impl Computer {
    pub fn new(mut program: Vec<u16>) -> Self {
        if program.len() < u16::MAX as usize {
            program.resize(u16::MAX as usize, 0);
        }
        Computer {
            d: 0,
            a: 0,
            pc: 0,
            ram: vec![0; 32768],
            rom: program,
            flags: BitFlags::default(),
            time: 0,
            alu_out: 0,
            m_in: 0,
            os: Default::default(),
        }
    }

    /// executes the instruction pointed to by `self.pc` in `self.rom`.
    ///
    /// Returns true if execution should continue, returns false if an infinite loop is hit and execution should reset
    /// or terminate.
    pub fn step(&mut self, reset: bool, log: bool) {
        self.time += 1;
        let instr = self.rom[self.pc as usize];
        // form: [i, i, i, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3]

        if log {
            let _mem = ptable!(
                ["A", "D", "SP", "LCL", "ARG", "THIS", "THAT"],
                [
                    self.a,
                    self.d,
                    self.ram[0],
                    self.ram[1],
                    self.ram[2],
                    self.ram[3],
                    self.ram[4],
                ],
                [
                    "RAM[A]",
                    "RAM[D]",
                    "RAM[SP]",
                    "RAM[LCL]",
                    "RAM[ARG]",
                    "RAM[THIS]",
                    "RAM[THAT]"
                ],
                [
                    self.ram.get(self.a as usize).unwrap_or(&0),
                    self.ram.get(self.d as usize).unwrap_or(&0),
                    self.ram.get(self.ram[0] as usize).unwrap_or(&0),
                    self.ram.get(self.ram[1] as usize).unwrap_or(&0),
                    self.ram.get(self.ram[2] as usize).unwrap_or(&0),
                    self.ram.get(self.ram[3] as usize).unwrap_or(&0),
                    self.ram.get(self.ram[4] as usize).unwrap_or(&0),
                ]
            );

            let _timing = ptable!(
                ["PC", self.pc],
                ["Time", self.time],
                [
                    "Instr",
                    decode_instr(
                        instr,
                        &[
                            self.a,
                            self.d,
                            *self.ram.get(self.a as usize).unwrap_or(&0u16)
                        ]
                    )
                ] // , ["Binary", format!("{:016b}", instr)]
            );
        }

        let out_bits = self.flags.bits() & 0b0000_0011;
        let in_bits = ((instr & 0b0000_1111_1100_0000) >> 4) as u8;
        self.flags = BitFlags::from_bits(out_bits | in_bits).unwrap();

        let instr_type = match instr & 0b1000_0000_0000_0000 {
            0 => InstrType::A,
            _ => match instr & 0b1110_0000_0000_0000 {
                0b1110_0000_0000_0000 => InstrType::C,
                0b1100_0000_0000_0000 => InstrType::B,
                _ => panic!("Invalid instruction format: {instr:?}"),
            },
        };

        match instr_type {
            InstrType::A => {
                self.a = instr;
                self.pc += 1;
            }
            InstrType::B => {
                let func = BuiltInFunc::from_repr(instr).unwrap();
                match func {
                    BuiltInFunc::MathMul => self.os_mul(),
                    BuiltInFunc::MathDiv => self.os_div(),
                    BuiltInFunc::MathMin => self.os_min(),
                    BuiltInFunc::MathMax => self.os_max(),
                    BuiltInFunc::MathSqrt => self.os_sqrt(),
                    BuiltInFunc::StringNew => todo!(),
                    BuiltInFunc::StringDispose => todo!(),
                    BuiltInFunc::StringLength => todo!(),
                    BuiltInFunc::StringCharAt => todo!(),
                    BuiltInFunc::StringSetChar => todo!(),
                    BuiltInFunc::StringAppendChar => todo!(),
                    BuiltInFunc::StringEraseLast => todo!(),
                    BuiltInFunc::StringIntVal => todo!(),
                    BuiltInFunc::StringSetInt => todo!(),
                    BuiltInFunc::StringBackspace => todo!(),
                    BuiltInFunc::StringDblQuote => todo!(),
                    BuiltInFunc::StringNewline => todo!(),
                    BuiltInFunc::ArrayNew => todo!(),
                    BuiltInFunc::ArrayDispose => todo!(),
                    BuiltInFunc::OutputMoveCursor => todo!(),
                    BuiltInFunc::OutputPrintChar => todo!(),
                    BuiltInFunc::OutputPrintString => todo!(),
                    BuiltInFunc::OutputPrintInt => todo!(),
                    BuiltInFunc::OutputPrintLn => todo!(),
                    BuiltInFunc::OutputBackspace => todo!(),
                    BuiltInFunc::ScreenClear => todo!(),
                    BuiltInFunc::ScreenSetColor => todo!(),
                    BuiltInFunc::ScreenDrawPixel => todo!(),
                    BuiltInFunc::ScreenDrawLine => todo!(),
                    BuiltInFunc::ScreenDrawRectangle => todo!(),
                    BuiltInFunc::ScreenDrawCircle => todo!(),
                    BuiltInFunc::KeyboardPressed => todo!(),
                    BuiltInFunc::KeyboardReadChar => todo!(),
                    BuiltInFunc::KeyboardReadLine => todo!(),
                    BuiltInFunc::KeyboardReadInt => todo!(),
                    BuiltInFunc::MemPeek => todo!(),
                    BuiltInFunc::MemPoke => todo!(),
                    BuiltInFunc::MemAlloc => todo!(),
                    BuiltInFunc::MemDealloc => todo!(),
                    BuiltInFunc::SysInit => todo!(),
                    BuiltInFunc::SysHalt => todo!(),
                    BuiltInFunc::SysError => todo!(),
                    BuiltInFunc::SysWait => todo!(),
                }
            }
            InstrType::C => {
                let input = match (0b0001_0000_0000_0000 & instr) == 0 {
                    true => self.a,
                    false => self.ram[self.a as usize],
                };

                // calc
                self.alu_out = ALU(self.d, input, &mut self.flags);

                // set output values
                let addr = self.a as usize;

                if (instr & 0b0000_0000_0000_1000) != 0 {
                    self.ram[addr] = self.alu_out;
                }

                if (instr & 0b0000_0000_0001_0000) != 0 {
                    self.d = self.alu_out
                }

                // this needs to happen after RAM is updated, otherwise the target RAM address is incorrect
                if (instr & 0b0000_0000_0010_0000) != 0 {
                    self.a = self.alu_out;
                }

                self.m_in = self.ram[(self.a & 0b0111_1111_1111_1111) as usize];

                let neg = self.flags.contains(ControlBits::Neg);
                let zero = self.flags.contains(ControlBits::Zero);
                let should_jump = match instr & 0b0000_0000_0000_0111 {
                    0 => false,        // Never jump
                    1 => !neg & !zero, // If comp > 0 (JGT)
                    2 => zero,         // If comp = 0 (JEQ)
                    3 => !neg,         // If comp >= 0 (JGE)
                    4 => neg,          // If comp < 0 (JLT)
                    5 => !zero,        // If comp != 0 (JNE)
                    6 => neg | zero,   // If comp <= 0 (JLE)
                    _ => true,         // Unconditional jump
                };

                if should_jump {
                    if instr & 0b111 == 7 {
                        // dbg!(self.time);
                        // dbg!(self.a);
                        // dbg!(self.pc);
                    }
                    self.pc = self.a;
                } else {
                    self.pc += 1;
                }

                if reset {
                    self.pc = 0
                }
            }
        }
    }

    /// Executes until cpu.time == time
    pub fn run_until(&mut self, time: usize, reset: bool, log: bool) {
        while self.time < time {
            self.step(reset, log);
        }
    }

    /// Steps exactly `cycles` times
    pub fn run_exact(&mut self, cycles: usize, reset: bool, log: bool) {
        for _ in 0..cycles {
            self.step(reset, log);
        }
    }

    pub fn sp(&mut self) -> &mut u16 {
        let ind = self.ram[0] as usize;
        &mut self.ram[ind]
    }

    pub fn stack_top(&mut self) -> &mut u16 {
        let ind = self.ram[0] as usize;
        &mut self.ram[ind - 1]
    }

    pub fn lcl(&mut self) -> &mut u16 {
        let ind = self.ram[1] as usize;
        &mut self.ram[ind]
    }

    pub fn arg(&mut self) -> &mut u16 {
        let ind = self.ram[2] as usize;
        &mut self.ram[ind]
    }

    pub fn this(&mut self) -> &mut u16 {
        let ind = self.ram[3] as usize;
        &mut self.ram[ind]
    }

    pub fn that(&mut self) -> &mut u16 {
        let ind = self.ram[4] as usize;
        &mut self.ram[ind]
    }

    pub fn get_screen(&mut self) -> &mut [u16] {
        &mut self.ram[0x4000..0x6000]
    }

    pub fn get_keyboard(&mut self) -> &mut u16 {
        &mut self.ram[0x6000]
    }

    // pub fn emu_return(&mut self, value: Option<u16>) {
    //     // grab return address
    //     let lcl = *self.lcl();
    //     let arg = *self.arg();

    //     let ret_addr = self.ram[(lcl - 5) as usize];

    //     if let Some(val) = value {
    //         // set stack pointer to 1 past return address
    //         self.ram[0] = self.ram[(arg + 1) as usize];

    //         // set return value to new top of stack
    //         self.ram[arg as usize] = val;
    //     } else {
    //         self.ram[0] = self.ram[arg as usize]
    //     }

    //     self.ram[1] = self.ram[(lcl - 4) as usize];
    //     self.ram[2] = self.ram[(lcl - 3) as usize];
    //     self.ram[3] = self.ram[(lcl - 2) as usize];
    //     self.ram[4] = self.ram[(lcl - 1) as usize];

    //     self.pc = ret_addr;
    // }

    pub fn push_stack(&mut self, value: u16) {
        let stack = self.ram[0] as usize;
        self.ram[stack] = value;
        *self.sp() += 1;
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.ram[0] -= 1;

        self.ram[self.ram[0] as usize]
    }

    // ------------------------------------------------------------------------------------------ //
    //                                             Mem                                            //
    // ------------------------------------------------------------------------------------------ //

    pub fn os_alloc(&mut self, defrag: bool) {
        if defrag {
            self.os_defrag();
        }

        let length = (*self.stack_top() as usize) + 1;

        let mut remove: Option<usize> = None;
        let mut result: Option<usize> = None;

        for (i, block) in self.os.free_list.iter_mut().enumerate() {
            match block.len.cmp(&length) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => {
                    self.ram[block.offset] = length as u16;
                    remove = Some(i);
                    result = Some(block.offset + 1);
                    break;
                }
                std::cmp::Ordering::Greater => {
                    self.ram[block.offset] = length as u16;
                    result = Some(block.offset + 1);
                    block.offset += length;
                    block.len -= length;
                    break;
                }
            }
        }

        if let Some(i) = remove {
            // slow, but list size should be relatively small so it's whatever
            self.os.free_list.swap_remove(i);
        }

        if let Some(val) = result {
            *self.stack_top() = val as u16
        } else {
            if defrag {
                panic!("Unable to allocate memory")
            }
            self.os_alloc(true);
        }
    }

    pub fn os_dealloc(&mut self) {
        let offset = self.pop_stack() as usize;
        let length = self.ram[offset - 1] as usize;

        self.os.free_list.push(Block::new(offset, length));
    }

    pub fn os_defrag(&mut self) {
        self.os.free_list.sort();

        let mut remove_list = Vec::new();

        let mut iter = self.os.free_list.iter_mut();
        let mut prev = iter.next().unwrap();


        for (i, block) in iter.enumerate() {
            if prev.offset + prev.len == block.offset {
                remove_list.push(i + 1);
                prev.len += block.len;
                self.ram[prev.offset] = prev.len as u16;
                // don't update `prev` if we merged the blocks, thus allowing `prev` to be merged
                // with the next block too if possible. `block` is being removed thus shouldn't
                // be used
            } else {
                prev = block;
            }
        }

        for i in remove_list {
            self.os.free_list.swap_remove(i);
        }
    }

    pub fn os_peek(&mut self) {
        let addr = *self.stack_top() as usize;

        *self.stack_top() = self.ram[addr];
    }

    pub fn os_poke(&mut self) {
        let addr = self.pop_stack() as usize;
        let val = *self.stack_top();

        self.ram[addr] = val;
    }

    // ------------------------------------------------------------------------------------------ //
    //                                            Math                                            //
    // ------------------------------------------------------------------------------------------ //

    pub fn os_mul(&mut self) {
        let x = self.pop_stack();
        let y = *self.stack_top();

        *self.stack_top() = x.wrapping_mul(y);
    }

    pub fn os_div(&mut self) {
        let x = self.pop_stack();
        let y = *self.stack_top();

        *self.stack_top() = x / y;
    }

    pub fn os_min(&mut self) {
        let x = self.pop_stack();
        let y = *self.stack_top();

        *self.stack_top() = x.min(y);
    }

    pub fn os_max(&mut self) {
        let x = self.pop_stack();
        let y = *self.stack_top();

        *self.stack_top() = x.max(y);
    }

    pub fn os_sqrt(&mut self) {
        let x = *self.stack_top();

        *self.stack_top() = (x as f32).sqrt() as u16;
    }

    // ------------------------------------------------------------------------------------------ //
    //                                            Array                                           //
    // ------------------------------------------------------------------------------------------ //

    pub fn os_array_new(&mut self) {
        self.os_alloc(false);
    }

    pub fn os_array_dispose(&mut self) {
        let addr = self.ram[3];

        self.push_stack(addr);

        self.os_dealloc()
    }
}
