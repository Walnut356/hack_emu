use n2t::software::assembler::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[test]
pub fn test_max() {
    let path = Path::new(r#"src/software/asm files/Max.asm"#);
    let output = to_machine_code(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Max_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    assert_eq!(buf1, buf2);
}

#[test]
pub fn test_pong() {
    let path = Path::new(r#"src/software/asm files/Pong.asm"#);
    let output = to_machine_code(&path);

    let out_file = File::open(output).unwrap();
    let mut buf1 = String::new();
    let mut out_stream = BufReader::new(out_file);
    out_stream.read_to_string(&mut buf1).unwrap();

    let test_path = Path::new(r#"../target_files/Pong_target.hack"#);
    let test_against = File::open(test_path).unwrap();
    let mut buf2 = String::new();
    let mut test_stream = BufReader::new(test_against);
    test_stream.read_to_string(&mut buf2).unwrap();

    assert!(buf1 == buf2);
}
