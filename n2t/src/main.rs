use n2t::hardware::logic_gate::cpu::Computer;
use n2t::software::assembler::*;
use n2t::utils::*;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    let path = Path::new(r#"src/software/asm files/Pong.asm"#);
    to_machine_code(&path);
}
