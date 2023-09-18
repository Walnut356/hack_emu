use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn u16_from_i16(val: i16) -> u16 {
    u16::from_ne_bytes(val.to_ne_bytes())
}

fn get_file_buffer(path: &Path, ext: &str) -> BufReader<File> {
    assert_eq!(
        path.extension().unwrap(),
        ext,
        "Expected file extension '{:?}', got file extension {:?}",
        ext,
        path.extension().unwrap()
    );

    let file = File::open(path).unwrap();


    BufReader::new(file)
}

/// Accepts a path and an extension. Returns a tuple of the file reader and the file's name (no
/// file extension or full path) .If the path is a directory, the returned Vec will contain multiple
/// elements, if it is a file, it will contain 1.
///
/// Panics if there are no files of the given extension
pub fn get_file_buffers(path: &Path, ext: &str) -> Vec<(BufReader<File>, String)> {
    let mut files = Vec::new();

    if path.is_dir() {
        let mut file_list = path.read_dir().unwrap();
        while let Some(Ok(file)) = file_list.next() {
            let f_path = file.path();
            if f_path.extension() == Some(OsStr::new(ext)) {
                files.push((
                    get_file_buffer(&f_path, ext),
                    f_path.file_stem().unwrap().to_str().unwrap().to_owned(),
                ));
            }
        }
    } else {
        files.push((
            get_file_buffer(path, ext),
            path.file_stem() // there literally has to be a better way, right?
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        ));
    }
    if files.is_empty() {
        panic! {"No files with extension {} in directory '{:?}'", ext, path}
    }
    files
}

pub fn hack_to_vec(path: &Path) -> Vec<u16> {
    let buffer = get_file_buffer(path, "hack");
    let mut program = Vec::new();

    let mut lines = buffer.lines();

    while let Some(Ok(line)) = lines.next() {
        program.push(u16::from_str_radix(&line, 2).expect("Not a binary number"))
    }
    program
}

// kinda disgusting but it'll do.
pub fn bitvec_from_int(mut int: u16) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..16 {
        result.push((int & 0b0000_0000_0000_0001) as u8);
        int >>= 1;
    }
    result.into_iter().rev().collect()
}

pub fn int_from_bitvec(vec: &Vec<u8>) -> u16 {
    let mut result: u16 = 0;
    for (i, j) in vec.iter().enumerate() {
        result |= (*j) as u16;
        if i < vec.len() - 1 {
            result <<= 1;
        }
    }
    result
}

pub fn decode_bitvec_instr(instr: &Vec<u8>) {
    // form: [i, i, i, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3]
    let a_or_c = match instr[0] {
        0 => {
            println!(
                "a instr: load value {} into A register",
                int_from_bitvec(instr)
            );
            return;
        }
        1 => "c instr",
        _ => "Error",
    };

    let a_or_m = match instr[3] {
        0 => "using value of A as val",
        1 => "using RAM[A] as val",
        _ => "Error",
    };

    let cmp = match instr[4..10] {
        [1, 0, 1, 0, 1, 0] => "0",
        [1, 1, 1, 1, 1, 1] => "1",
        [1, 1, 1, 0, 1, 0] => "-1",
        [0, 0, 1, 1, 0, 0] => "D",
        [1, 1, 0, 0, 0, 0] => "val",
        [0, 0, 1, 1, 0, 1] => "!D",
        [1, 1, 0, 0, 0, 1] => "!val",
        [0, 0, 1, 1, 1, 1] => "minus D",
        [1, 1, 0, 0, 1, 1] => "minus val",
        [0, 1, 1, 1, 1, 1] => "D + 1",
        [1, 1, 0, 1, 1, 1] => "val + 1",
        [0, 0, 1, 1, 1, 0] => "D - 1",
        [1, 1, 0, 0, 1, 0] => "val - 1",
        [0, 0, 0, 0, 1, 0] => "D + val",
        [0, 1, 0, 0, 1, 1] => "D - val",
        [0, 0, 0, 1, 1, 1] => "val - D",
        [0, 0, 0, 0, 0, 0] => "D & val",
        [0, 1, 0, 1, 0, 1] => "D | val",
        _ => "Error",
    };

    let store_in = match instr[10..13] {
        [0, 0, 0] => "None",
        [0, 0, 1] => "RAM[A]",
        [0, 1, 0] => "D",
        [0, 1, 1] => "D and RAM[A]",
        [1, 0, 0] => "A",
        [1, 0, 1] => "A and RAM[A]",
        [1, 1, 0] => "A and D",
        [1, 1, 1] => "A, D, and RAM[A]",
        _ => "Error",
    };

    let jump = match instr[13..=15] {
        [0, 0, 0] => "Never",
        [0, 0, 1] => "If greater than",
        [0, 1, 0] => "If equal",
        [0, 1, 1] => "If greater than or equal",
        [1, 0, 0] => "If less than",
        [1, 0, 1] => "If not equal",
        [1, 1, 0] => "If less than or equal",
        [1, 1, 1] => "Always",
        _ => "Error",
    };

    println!("{a_or_c}: {a_or_m}, compute '{cmp}' and store the value in {store_in}. {jump} jump to ROM[A].")
}

pub fn decode_instr(instr: u16, vars: &[u16]) -> String {
    if instr & 0b1000_0000_0000_0000 == 0 {
        let val = instr & 0b0111_1111_1111_1111;
        return match val {
            0 => "@SP".to_owned(),
            1 => "@LCL".to_owned(),
            2 => "@ARG".to_owned(),
            3 => "@THIS".to_owned(),
            4 => "@THAT".to_owned(),
            5 => "@R5".to_owned(),
            6 => "@R6".to_owned(),
            7 => "@R7".to_owned(),
            8 => "@R8".to_owned(),
            9 => "@R9".to_owned(),
            10 => "@R10".to_owned(),
            11 => "@R11".to_owned(),
            12 => "@R12".to_owned(),
            13 => "@R13".to_owned(),
            14 => "@R14".to_owned(),
            15 => "@R15".to_owned(),
            _ => format!("@{val}"),
        };
    }
    let a_or_m = match instr & 0b0001_0000_0000_0000 > 0 {
        false => "A",
        true => "M",
    };

    let cmp = match (instr & 0b0000_1111_1100_0000) >> 6 {
        0b0000_0000_0010_1010 => "0".to_owned(),
        0b0000_0000_0011_1111 => "1".to_owned(),
        0b0000_0000_0011_1010 => "-1".to_owned(),
        0b0000_0000_0000_1100 => "D".to_owned(),
        0b0000_0000_0011_0000 => a_or_m.to_owned(),
        0b0000_0000_0000_1101 => "!D".to_owned(),
        0b0000_0000_0011_0001 => format!("!{a_or_m}"),
        0b0000_0000_0000_1111 => "-D".to_owned(),
        0b0000_0000_0011_0011 => format!("-{a_or_m}"),
        0b0000_0000_0001_1111 => "D+1".to_owned(),
        0b0000_0000_0011_0111 => format!("{a_or_m}+1"),
        0b0000_0000_0000_1110 => "D-1".to_owned(),
        0b0000_0000_0011_0010 => format!("{a_or_m}-1"),
        0b0000_0000_0000_0010 => format!("D+{a_or_m}"),
        0b0000_0000_0001_0011 => format!("D-{a_or_m}"),
        0b0000_0000_0000_0111 => format!("{a_or_m}-D"),
        0b0000_0000_0000_0000 => format!("D & {a_or_m}"),
        0b0000_0000_0001_0101 => format!("D | {a_or_m}"),
        _ => "Error".to_owned(),
    };

    let store_in = match (instr & 0b0000_0000_0011_1000) >> 3 {
        0 => "",
        1 => "M=",
        2 => "D=",
        3 => "DM=",
        4 => "A=",
        5 => "AM=",
        6 => "AD=",
        7 => "ADM=",
        _ => "Error",
    };

    let jump = match instr & 0b0000_0000_0000_0111 {
        0 => "",
        1 => "; Jump if greater than 0",
        2 => "; Jump if equal to 0",
        3 => "; Jump if greater than or equal to 0",
        4 => "; Jump if less than 0",
        5 => "; Jump if not equal to 0",
        6 => "; Jump if less than or equal to 0",
        7 => "; Unconditional jump",
        _ => "Error",
    };
    let out = format!("{store_in}{cmp}{jump}");
    let mut temp_out = cmp.replace('A', &vars[0].to_string());
    temp_out = temp_out.replace('D', &vars[1].to_string());
    temp_out = temp_out.replace('M', &vars[2].to_string());

    let out2 = format!("{out} | {store_in}{temp_out}");

    out2
}
