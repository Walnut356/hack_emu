use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
};

use n2t::{
    hardware::native::cpu::Computer,
    software::{assembler::asm_to_hack, compiler::jack_to_vm, vm::vm_to_asm},
    utils::hack_to_vec,
};

use serial_test::serial;

fn get_computer(file_path: &str) -> Computer {
    let path = Path::new(file_path);
    let asm = vm_to_asm(path);
    let machine = asm_to_hack(&asm);
    let program = hack_to_vec(&machine);

    let cpu = Computer::new(program);

    cpu
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
#[serial]
fn test_square() {
    let paths = [
        (
            "./test_files/ch 10/Square/Main.jack",
            "./test_files/ch 10/Square/Main.xml",
            "./test_files/ch 10/Square/MainExample.xml",
        ),
        (
            "./test_files/ch 10/Square/Square.jack",
            "./test_files/ch 10/Square/Square.xml",
            "./test_files/ch 10/Square/SquareExample.xml",
        ),
        (
            "./test_files/ch 10/Square/SquareGame.jack",
            "./test_files/ch 10/Square/SquareGame.xml",
            "./test_files/ch 10/Square/SquareGameExample.xml",
        ),
    ];

    for (jack, xml, example) in paths {
        let path = test_data_path(jack);
        let _vm = jack_to_vm(&path);

        let xml_path = test_data_path(xml);
        let mut xml_out = File::open(xml_path).unwrap();
        let mut output_text = String::new();
        xml_out.read_to_string(&mut output_text).unwrap();

        let example_path = test_data_path(example);
        println!("{:?}", example_path);
        let mut example_out = File::open(example_path).unwrap();
        let mut example_text = String::new();
        example_out.read_to_string(&mut example_text).unwrap();

        for (a, b) in zip(output_text.lines(), example_text.lines()) {
            assert_eq!(a, b)
        }
    }
}

#[test]
#[serial]
fn test_array() {
    let paths = [
        (
            "./test_files/ch 10/ArrayTest/Main.jack",
            "./test_files/ch 10/ArrayTest/Main.xml",
            "./test_files/ch 10/ArrayTest/MainExample.xml",
        ),
    ];

    for (jack, xml, example) in paths {
        let path = test_data_path(jack);
        let _vm = jack_to_vm(&path);

        let xml_path = test_data_path(xml);
        let mut xml_out = File::open(xml_path).unwrap();
        let mut output_text = String::new();
        xml_out.read_to_string(&mut output_text).unwrap();

        let example_path = test_data_path(example);
        println!("{:?}", example_path);
        let mut example_out = File::open(example_path).unwrap();
        let mut example_text = String::new();
        example_out.read_to_string(&mut example_text).unwrap();

        for (a, b) in zip(output_text.lines(), example_text.lines()) {
            assert_eq!(a, b)
        }
    }
}