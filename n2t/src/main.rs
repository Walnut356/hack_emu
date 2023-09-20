
use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
};

use n2t::software::compiler::JackCompiler;
use n2t::software::vm::vm_to_asm;


pub fn test_data_path(file_path: &str) -> PathBuf {
    match std::env::var("ENV_ROOT_DIR") {
        Ok(path) => Path::new(&path).join(file_path),
        Err(_) => Path::new(&std::env::current_dir().unwrap())
            .join("../")
            .join(file_path),
    }
}


fn main() {
    let path = test_data_path(r"test_files\ch 11\Square");
    vm_to_asm(&path);
}
