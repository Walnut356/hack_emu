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

use super::memory::*;

pub struct CPU {}

pub struct Computer {
    pub mem: RAM32K,
    pub cpu: CPU,
    // minor liberty to make loading the program and testing less painful. The ROM is doing basically the same thing as
    // the RAM anyway, except it never changes, so I shouldn't be missing many learning opportunities.
    pub rom: Vec<u16>,
}

impl Computer {
    pub fn new(program: Vec<u16>) {}
}
fn execute() {
    loop {}
}
