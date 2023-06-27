use n2t::hardware::logic_gate::cpu::Computer;
use n2t::software::assembler::*;
use n2t::utils::*;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    let path = Path::new(r#"C:\Users\ant_b\Documents\Coding Projects\nand_2_tetris\n2t\src\software\asm files\Max.asm"#);
    to_machine_code(&path)
}
