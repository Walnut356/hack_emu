
use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
};

use n2t::software::compiler::JackCompiler;


pub fn test_data_path(file_path: &str) -> PathBuf {
    match std::env::var("ENV_ROOT_DIR") {
        Ok(path) => Path::new(&path).join(file_path),
        Err(_) => Path::new(&std::env::current_dir().unwrap())
            .join("../")
            .join(file_path),
    }
}

fn test_convertbin() {
    let paths = [(
        "./test_files/ch 11/ConvertToBin/Main.jack",
        "./test_files/ch 11/ConvertToBin/Main.vm",
        "./test_files/ch 11/ConvertToBin/MainTarget.vm",
    )];

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

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            // don't fail on dumb label naming conventions
            if (a.starts_with("if-goto") && b.starts_with("if-goto"))
                || (a.starts_with("goto") && b.starts_with("goto"))
                || (a.starts_with("label") && b.starts_with("label"))
            {
                continue;
            }
            assert_eq!(a, b);
        }
    }
}
fn main() {
    test_convertbin();
}
