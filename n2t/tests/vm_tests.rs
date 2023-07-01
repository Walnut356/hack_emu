use std::path::Path;

use n2t::{
    hardware::native::cpu::Computer,
    software::{assembler::asm_to_hack, vm::vm_to_asm},
    utils::hack_to_vec,
};

#[test]
fn test_simpleadd() {
    let path = Path::new(r#"../ch 7 vm files/SimpleAdd.vm"#);
    let asm = vm_to_asm(&path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);

    let mut cpu = Computer::new(program);

    while cpu.execute(false, false) {}

    assert_eq!(
        cpu.ram[0], 257,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(cpu.ram[256], 15, "Incorrect calcultion of '7 + 8'");
}
