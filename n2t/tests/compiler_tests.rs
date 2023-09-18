//! Tests for chapter 11

use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
};

use n2t::{
    hardware::native::cpu::Computer,
    software::{assembler::asm_to_hack, compiler::*, vm::vm_to_asm},
    utils::hack_to_vec,
};

fn get_computer(file_path: &str) -> Computer {
    let path = Path::new(file_path);
    let asm = vm_to_asm(path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);



    Computer::new(program)
}

pub fn test_data_path(file_path: &str) -> PathBuf {
    match std::env::var("ENV_ROOT_DIR") {
        Ok(path) => Path::new(&path).join(file_path),
        Err(_) => Path::new(&std::env::current_dir().unwrap())
            .join("../")
            .join(file_path),
    }
}

#[test]
fn test_seven() {
    let paths = [
        (
            "./test_files/ch 11/Seven/Main.jack",
            "./test_files/ch 11/Seven/Main.vm",
            "./test_files/ch 11/Seven/MainTarget.vm",
        ),
    ];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        println!("{:?}", target_path);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            assert_eq!(a, b)
        }
    }
}