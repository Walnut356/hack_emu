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

pub fn execute(program: Vec<u16>) {
    // I'm more or less required to use vec's for RAM/ROM as 64k Vec<u8>'s of size 16 is kindof a lot of memory.
    let rom = program;

    loop {
        let mut reset = false;
        let mut pc = InstPtr::new();
        let mut ram = RAM32K::new();
        let mut reg_a = Register::new();
        let mut reg_d = Register::new();
        let mut alu_out: Vec<u8> = vec![0; 16];
        let mut in_m: Vec<u8> = vec![0; 16];
        let mut control = ControlBits::new();
        let mut counter: u32 = 1;

        while !reset {
            // ------------------------------------- Input and register updates ------------------------------------- //
            //reset = reset input
            let instr = bitvec_from_int(rom[int_from_bitvec(&pc.val.data) as usize]);
            println!(
                "a register: {}, d register: {}, alu_out: {}, out_m: {}",
                int_from_bitvec(&reg_a.data),
                int_from_bitvec(&reg_d.data),
                int_from_bitvec(&alu_out),
                ram.out,
            );
            println!("instr#: {}, pc: {}", counter, int_from_bitvec(&pc.val.data));
            decode_instr(&instr);
            counter += 1;
            control = ControlBits {
                zx: instr[4],
                nx: instr[5],
                zy: instr[6],
                ny: instr[7],
                f: instr[8],
                no: instr[9],
                zr: control.zr,
                ng: control.ng,
            };
            reg_a.cycle(
                &multi_MUX(&instr, &alu_out, instr[0]),
                OR(NOT(instr[0]), AND(instr[0], instr[10])),
            );

            // -------------------------------------- alu processing and output ------------------------------------- //
            alu_out = ALU(
                &reg_d.data,
                &multi_MUX(&reg_a.data, &in_m, instr[3]),
                &mut control,
            );

            reg_d.cycle(&alu_out, AND(instr[0], instr[11]));

            // ick
            in_m = bitvec_from_int(ram.cycle(
                int_from_bitvec(&alu_out),
                int_from_bitvec(&reg_a.data) & 0b0111_1111_1111_1111,
                AND(instr[0], instr[12]),
            ));

            let should_jump = AND(
                instr[0],
                MUX_8(
                    0,
                    control.ng,
                    control.zr,
                    OR(control.ng, control.zr),
                    AND(NOT(control.ng), NOT(control.zr)),
                    NOT(control.zr),
                    NOT(control.ng),
                    1,
                    instr[13],
                    instr[14],
                    instr[15],
                ),
            );

            pc.cycle(&reg_a.data, should_jump, 1, 0);
        }
    }
}
