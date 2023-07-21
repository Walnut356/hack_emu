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

fn get_computer(file_path: &str) -> Computer {
    let path = Path::new(file_path);
    let asm = vm_to_asm(&path);
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
fn test_noexpression_square() {
    let path = test_data_path("./test_files/ch 10/Square/SquareGame.jack");
    let _vm = jack_to_vm(&path);

    let path1 = test_data_path("./test_files/ch 10/Square/SquareGame.xml");
    let mut file1 = File::open(path1).unwrap();
    let mut t_1 = String::new();
    file1.read_to_string(&mut t_1).unwrap();
    let path2 = test_data_path("./test_files/ch 10/Square/SquareGameTExample.xml");
    let mut file2 = File::open(path2).unwrap();
    let mut t_2 = String::new();
    file2.read_to_string(&mut t_2).unwrap();

    for (a, b) in zip(t_1.lines(), t_2.lines()) {
        assert_eq!(a, b)
    }
}
