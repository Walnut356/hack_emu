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

    let cpu = Computer::new(program);

    cpu
}

// ------------------------------------------------------------------------------------------------------------------ //
//                                                       Part 1                                                       //
// ------------------------------------------------------------------------------------------------------------------ //

#[test]
fn test_simpleadd() {
    let mut cpu = get_computer(r#"../test_files/ch 7/SimpleAdd.vm"#);

    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer
    cpu.ram[16] = 3; // "pointer" pointer

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
    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer
    cpu.ram[16] = 3; // "pointer" pointer
    while cpu.execute(false, false) {}

    assert_eq!(
        cpu.ram[0], 266,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(
        i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()),
        !((28 & 57) | 82)
    );
}

#[test]
fn test_basictest() {
    let mut cpu = get_computer("../test_files/ch 7/BasicTest.vm");
    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer
    cpu.ram[16] = 3; // "pointer" pointer

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

#[test]
fn test_pointertest() {
    let mut cpu = get_computer("../test_files/ch 7/PointerTest.vm");
    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 6084);
    assert_eq!(cpu.ram[3], 3030);
    assert_eq!(cpu.ram[4], 3040);
    assert_eq!(cpu.ram[3032], 32);
    assert_eq!(cpu.ram[3046], 46);
}

#[test]
fn test_statictest() {
    let mut cpu = get_computer("../test_files/ch 7/StaticTest.vm");
    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer
    cpu.ram[16] = 3; // "pointer" pointer

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 1110);
}

// thorough stack test
#[test]
fn test_stepwise_stacktest() {
    let path = Path::new(r#"../test_files/ch 7/StackTest.vm"#);
    let asm = vm_to_asm(&path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);

    let mut cpu = Computer::new(program);

    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[3] = 3000; // "this" pointer
    cpu.ram[4] = 3010; // "that" pointer
    cpu.ram[16] = 3; // "pointer" pointer

    let mut pc_stop = 29;

    // ----------------------------------------------------- EQ ----------------------------------------------------- //
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 257,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[256].to_ne_bytes()), -1);
    pc_stop += 25;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 258,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[257].to_ne_bytes()), 0);
    pc_stop += 25;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 259,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[258].to_ne_bytes()), 0);
    pc_stop += 25;

    // ----------------------------------------------------- LT ----------------------------------------------------- //
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 260,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[259].to_ne_bytes()), 0);
    pc_stop += 25;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 261,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[260].to_ne_bytes()), -1);
    pc_stop += 25;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 262,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[261].to_ne_bytes()), 0);
    pc_stop += 25;

    // ----------------------------------------------------- GT ----------------------------------------------------- //
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 263,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[262].to_ne_bytes()), -1);
    pc_stop += 26;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 264,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[263].to_ne_bytes()), 0);
    pc_stop += 25;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 265,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[264].to_ne_bytes()), 0);
    // ----------------------------------------------- Non-comparison ----------------------------------------------- //

    // push 57, 31, 53, add 31 and 53
    pc_stop += 29;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 267,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), 84);
    // push 112, subtract prev result from 112
    pc_stop += 14;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 267,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), -28);

    // negate prev result
    pc_stop += 3;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 267,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[266].to_ne_bytes()), 28);

    // and prev result with 84
    pc_stop += 8;
    cpu.run_until(pc_stop, false, true);
    assert_eq!(
        cpu.ram[0], 266,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()), 28 & 57); // should be 24

    // push 82, or 82 with prev result
    pc_stop += 15;
    cpu.run_until(pc_stop, false, true);
    assert_eq!(
        cpu.ram[0], 266,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(
        i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()),
        (28 & 57) | 82
    ); // should be 90

    // not prev result
    pc_stop += 4;
    cpu.run_until(pc_stop, false, false);
    assert_eq!(
        cpu.ram[0], 266,
        "Stack pointer pointing to incorrect memory location"
    );
    assert_eq!(
        i16::from_ne_bytes(cpu.ram[265].to_ne_bytes()),
        !((28 & 57) | 82)
    ); // not 90 should be
}

// ------------------------------------------------------------------------------------------------------------------ //
//                                                       Part 2                                                       //
// ------------------------------------------------------------------------------------------------------------------ //

#[test]
fn test_basicloop() {
    let mut cpu = get_computer("../test_files/ch 8/ProgramFlow/BasicLoop/BasicLoop.vm");
    // forcing initialized pointers to match official software test conditions
    cpu.ram[1] = 300; // "local" pointer
    cpu.ram[2] = 400; // "argument" pointer
    cpu.ram[400] = 3; // argument initial val

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[0], 257);
    assert_eq!(cpu.ram[256], 6);
}

#[test]
fn test_fibseries() {
    let mut cpu = get_computer("../test_files/ch 8/ProgramFlow/FibonacciSeries/FibonacciSeries.vm");

    // force initialized poitners to match official software test conditions
    cpu.ram[1] = 300;
    cpu.ram[2] = 400;
    cpu.ram[400] = 6;
    cpu.ram[401] = 3000;

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[3000..=3005], [0, 1, 1, 2, 3, 5])
}

#[test]
fn test_simplefunction() {
    let mut cpu = get_computer("../test_files/ch 8/FunctionCalls/SimpleFunction/SimpleFunction.vm");
    cpu.ram[0] = 317;
    cpu.ram[1] = 317;
    cpu.ram[2] = 310;
    cpu.ram[3] = 3000;
    cpu.ram[4] = 4000;
    cpu.ram[310] = 1234;
    cpu.ram[311] = 37;
    cpu.ram[312] = 1000;
    cpu.ram[313] = 305;
    cpu.ram[314] = 300;
    cpu.ram[315] = 3010;
    cpu.ram[316] = 4010;

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[0..=4], [311, 305, 300, 3010, 4010]);
    assert_eq!(cpu.ram[310], 1196);
}
