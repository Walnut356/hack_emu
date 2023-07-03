use std::path::Path;

use n2t::{
    hardware::native::cpu::Computer,
    software::{assembler::asm_to_hack, vm::vm_to_asm},
    utils::hack_to_vec,
};

fn get_computer(file_path: &str) -> Computer {
    let path = Path::new(file_path);
    let asm = vm_to_asm(&path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);

    let mut cpu = Computer::new(program);

    cpu
}

#[test]
fn test_simpleadd() {
    let mut cpu = get_computer(r#"../test_files/ch 7/SimpleAdd.vm"#);

    while cpu.execute(false, false) {}

    assert_eq!(
        cpu.ram[0], 257,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(cpu.ram[256], 15, "Incorrect calcultion of '7 + 8'");
}

#[test]
fn test_stacktest() {
    let mut cpu = get_computer(r#"../test_files/ch 7/StackTest.vm"#);

    while cpu.execute(false, false) {}

    assert_eq!(
        cpu.ram[0], 266,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()), -115);
}

#[test]
fn test_basictest() {
    let mut cpu = get_computer("../test_files/ch 7/BasicTest.vm");

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 472);
    assert_eq!(cpu.ram[300], 10);
    assert_eq!(cpu.ram[401], 21);
    assert_eq!(cpu.ram[402], 22);
    assert_eq!(cpu.ram[3006], 36);
    assert_eq!(cpu.ram[3012], 42);
    assert_eq!(cpu.ram[3015], 45);
    assert_eq!(cpu.ram[11], 510);
}

// thorough stack test
// #[test]
// fn test_stacktest() {
//     let path = Path::new(r#"../test_files/ch 7/StackTest.vm"#);
//     let asm = vm_to_asm(&path);
//     let machine = asm_to_hack(&asm);
//     let program = hack_to_vec(&machine);

//     let mut cpu = Computer::new(program);

//     let mut pc_stop = 29;

//     // ----------------------------------------------------- EQ ----------------------------------------------------- //
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 257,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[256].to_ne_bytes()), -1);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 258,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[257].to_ne_bytes()), 0);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 259,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[258].to_ne_bytes()), 0);
//     pc_stop += 25;

//     // ----------------------------------------------------- LT ----------------------------------------------------- //
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 260,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[259].to_ne_bytes()), 0);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 261,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[260].to_ne_bytes()), 0);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 262,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[261].to_ne_bytes()), -1);
//     pc_stop += 25;

//     // ----------------------------------------------------- GT ----------------------------------------------------- //
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 263,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[262].to_ne_bytes()), 0);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 264,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[263].to_ne_bytes()), 0);
//     pc_stop += 25;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 265,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[264].to_ne_bytes()), -1);
//     // ----------------------------------------------- Non-comparison ----------------------------------------------- //

//     // push 57, 31, 53, add 31 and 53
//     pc_stop += 29;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 267,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), 84);
//     // push 112, subtract prev result from 112
//     pc_stop += 16;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 267,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), 28);

//     // negate prev result
//     pc_stop += 3;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 267,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), -28);

//     // and prev result with 84
//     pc_stop += 8;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 266,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()), -28 & 57); // should be 32

//     // push 82, or 82 with prev result
//     pc_stop += 14;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 266,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()), 32 | 82); // should be 114

//     // not prev result
//     pc_stop += 4;
//     cpu.run_until(pc_stop, false, false);
//     assert_eq!(
//         cpu.ram[0], 266,
//         "Stack pointer pointing to incorrect memory location"
//     );
//     assert_eq!(i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()), -115); // not 114 should be -115
// }
