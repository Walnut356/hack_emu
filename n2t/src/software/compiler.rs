use crate::utils::get_file_buffers;
use std::{
    fs::File,
    path::{Path, PathBuf}, io::Write,
};

pub fn jack_to_vm(path: &Path) -> PathBuf {
    let mut out_path;

    if path.is_file() {
        out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    } else {
        out_path = Path::new(path).join(path.file_stem().unwrap());
    }

    let files = get_file_buffers(path, "jack");

    // Init output .asm file
    out_path.set_extension("xml");
    let mut out_file = File::create(out_path.clone()).unwrap();
    let mut output = String::new();

    out_file.flush().unwrap();

    out_path
}
