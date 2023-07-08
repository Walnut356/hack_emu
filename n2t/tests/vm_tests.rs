use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use n2t::{
    hardware::native::cpu::Computer,
    software::{assembler::asm_to_hack, vm::vm_to_asm},
    utils::{hack_to_vec, u16_from_i16},
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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 8; // skip over bootstrapping code
        }
    }

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
    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 8; // skip over bootstrapping code
        }
    }

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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 8; // skip over bootstrapping code
        }
    }

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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 8; // skip over bootstrapping code
        }
    }

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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 8; // skip over bootstrapping code
        }
    }

    assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 1110);
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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 55; // skip over bootstrapping code
        }
        if cpu.time > 600 {
            break;
        }
    }

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

    while cpu.execute(false, false) {
        if cpu.pc == 4 {
            cpu.pc = 55; // skip over bootstrapping code
        }
    }

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
    cpu.ram[312] = 9;
    cpu.ram[313] = 305;
    cpu.ram[314] = 300;
    cpu.ram[315] = 3010;
    cpu.ram[316] = 4010;

    cpu.pc = 55; // skip over bootstrapping code
    while cpu.execute(false, false) {
        if cpu.time == 85 {
            // return statement beginning
            assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 1196)
        }
        if cpu.pc == 185 {
            // since there's nowhere to return to, we break just before the return
            break;
        }
    }

    assert_eq!(cpu.ram[0..=4], [311, 305, 300, 3010, 4010]);
    assert_eq!(cpu.ram[310], 1196);
}

#[test]
fn test_nestedcall() {
    let mut cpu = get_computer("../test_files/ch 8/FunctionCalls/NestedCall/");

    cpu.ram[0] = 261;
    cpu.ram[1] = 261;
    cpu.ram[2] = 256;
    cpu.ram[3] = u16_from_i16(-3);
    cpu.ram[4] = u16_from_i16(-4);
    cpu.ram[5] = u16_from_i16(-1); // test results
    cpu.ram[6] = u16_from_i16(-1);
    cpu.ram[256] = 1234; // fake stack frame from call Sys.init
    cpu.ram[257] = u16_from_i16(-1);
    cpu.ram[258] = u16_from_i16(-2);
    cpu.ram[259] = u16_from_i16(-3);
    cpu.ram[260] = u16_from_i16(-4);

    cpu.ram[261] = u16_from_i16(-1); // Initialize stack to check for local segment
    cpu.ram[262] = u16_from_i16(-1); // being cleared to zero.
    cpu.ram[263] = u16_from_i16(-1);
    cpu.ram[264] = u16_from_i16(-1);
    cpu.ram[265] = u16_from_i16(-1);
    cpu.ram[266] = u16_from_i16(-1);
    cpu.ram[267] = u16_from_i16(-1);
    cpu.ram[268] = u16_from_i16(-1);
    cpu.ram[269] = u16_from_i16(-1);
    cpu.ram[270] = u16_from_i16(-1);
    cpu.ram[271] = u16_from_i16(-1);
    cpu.ram[272] = u16_from_i16(-1);
    cpu.ram[273] = u16_from_i16(-1);
    cpu.ram[274] = u16_from_i16(-1);
    cpu.ram[275] = u16_from_i16(-1);
    cpu.ram[276] = u16_from_i16(-1);
    cpu.ram[277] = u16_from_i16(-1);
    cpu.ram[278] = u16_from_i16(-1);
    cpu.ram[279] = u16_from_i16(-1);
    cpu.ram[280] = u16_from_i16(-1);
    cpu.ram[281] = u16_from_i16(-1);
    cpu.ram[282] = u16_from_i16(-1);
    cpu.ram[283] = u16_from_i16(-1);
    cpu.ram[284] = u16_from_i16(-1);
    cpu.ram[285] = u16_from_i16(-1);
    cpu.ram[286] = u16_from_i16(-1);
    cpu.ram[287] = u16_from_i16(-1);
    cpu.ram[288] = u16_from_i16(-1);
    cpu.ram[289] = u16_from_i16(-1);
    cpu.ram[290] = u16_from_i16(-1);
    cpu.ram[291] = u16_from_i16(-1);
    cpu.ram[292] = u16_from_i16(-1);
    cpu.ram[293] = u16_from_i16(-1);
    cpu.ram[294] = u16_from_i16(-1);
    cpu.ram[295] = u16_from_i16(-1);
    cpu.ram[296] = u16_from_i16(-1);
    cpu.ram[297] = u16_from_i16(-1);
    cpu.ram[298] = u16_from_i16(-1);
    cpu.ram[299] = u16_from_i16(-1);

    while cpu.execute(false, true) {
        if cpu.pc == 4 {
            cpu.ram[0] = 261;
        }
        if cpu.time > 4000 {
            break;
        }
    }

    assert_eq!(cpu.ram[0..=6], [261, 261, 256, 4000, 5000, 135, 246])
}

#[test]
fn test_fibelement() {
    let mut cpu = get_computer("../test_files/ch 8/FunctionCalls/FibonacciElement/");

    while cpu.execute(false, false) {}

    assert_eq!(cpu.ram[0], 262);
    assert_eq!(cpu.ram[(cpu.ram[0] - 1) as usize], 3);
}
