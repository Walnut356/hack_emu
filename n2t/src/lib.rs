/// logical implementations using only NAND chip + manually constructed chips
pub mod hardware {
    pub mod logic_gate {
        pub mod alu;
        pub mod arithmetic;
        pub mod cpu;
        pub mod gates;
        pub mod memory;
    }

    /// shortcut implementations in native rust to speed up processing
    pub mod native {
        pub mod alu;
        pub mod cpu;
        pub mod gates;
        pub mod instructions;
        pub mod memory;
    }
}

pub mod software {
    pub mod assembler;
    pub mod vm;
}

pub mod utils {
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
    };

    pub fn hack_to_vec(path: &Path) -> Vec<u16> {
        let buffer = get_file_buffer(path, "hack");
        let mut program = Vec::new();

        for line in buffer.lines() {
            program.push(u16::from_str_radix(line, 2).expect("Not a binary number"))
        }
        program
    }

    pub fn get_file_buffer(path: &Path, ext: &str) -> String {
        assert_eq!(
            path.extension().unwrap(),
            ext,
            "Expected file extension '{:?}', got file extension {:?}",
            ext,
            path.extension().unwrap()
        );

        let file = File::open(path).unwrap();
        let mut stream = BufReader::new(file);
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        buffer
    }

    // kinda disgusting but it'll do.
    pub fn bitvec_from_int(mut int: u16) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..16 {
            result.push((int & 0b0000_0000_0000_0001) as u8);
            int = int >> 1;
        }
        result.into_iter().rev().collect()
    }

    pub fn int_from_bitvec(vec: &Vec<u8>) -> u16 {
        let mut result: u16 = 0;
        for (i, j) in vec.into_iter().enumerate() {
            result |= (*j) as u16;
            if i < vec.len() - 1 {
                result = result << 1;
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

    pub fn decode_instr(instr: u16, vars: &[u16]) {
        if instr & 0b1000_0000_0000_0000 == 0 {
            let val = instr & 0b0111_1111_1111_1111;
            match val {
                0 => println!("@SP"),
                1 => println!("@LCL"),
                2 => println!("@ARG"),
                3 => println!("@THIS"),
                4 => println!("@THAT"),
                5 => println!("@R5"),
                6 => println!("@R6"),
                7 => println!("@R7"),
                8 => println!("@R8"),
                9 => println!("@R9"),
                10 => println!("@R10"),
                11 => println!("@R11"),
                12 => println!("@R12"),
                13 => println!("@R13"),
                14 => println!("@R14"),
                15 => println!("@R15"),
                _ => println!("@{val}"),
            }

            return;
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
            1 => "; Jump if greater than",
            2 => "; Jump if equal",
            3 => "; Jump if greater than or equal",
            4 => "; Jump if less than",
            5 => "; Jump if not equal",
            6 => "; Jump if less than or equal",
            7 => "; Unconditional jump",
            _ => "Error",
        };
        let out = format!("{store_in}{cmp}{jump}");
        let mut temp_out = cmp.replace("A", &vars[0].to_string());
        temp_out = temp_out.replace("D", &vars[1].to_string());
        temp_out = temp_out.replace("M", &vars[2].to_string());

        let out2 = format!("{store_in}{temp_out}");

        println!("{out} | {out2}")
    }
}
