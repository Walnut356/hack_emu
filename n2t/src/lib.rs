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
        let mut a_or_c = match instr[0] {
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

    pub fn decode_instr(instr: u16) {
        if instr & 0b1000_0000_0000_0000 == 0 {
            println!(
                "load value {} into A register",
                instr & 0b0111_1111_1111_1111
            );
            return;
        }
        let a_or_m = match instr & 0b0001_0000_0000_0000 > 0 {
            false => "using value of A as val",
            true => "using RAM[A] as val",
        };
        let cmp = match (instr & 0b0000_1111_1100_0000) >> 6 {
            0b0000_0000_0010_1010 => "0",
            0b0000_0000_0011_1111 => "1",
            0b0000_0000_0011_1010 => "-1",
            0b0000_0000_0000_1100 => "D",
            0b0000_0000_0011_0000 => "val",
            0b0000_0000_0000_1101 => "!D",
            0b0000_0000_0011_0001 => "!val",
            0b0000_0000_0000_1111 => "minus D",
            0b0000_0000_0011_0011 => "minus val",
            0b0000_0000_0001_1111 => "D + 1",
            0b0000_0000_0011_0111 => "val + 1",
            0b0000_0000_0000_1110 => "D - 1",
            0b0000_0000_0011_0010 => "val - 1",
            0b0000_0000_0000_0010 => "D + val",
            0b0000_0000_0001_0011 => "D - val",
            0b0000_0000_0000_0111 => "val - D",
            0b0000_0000_0000_0000 => "D & val",
            0b0000_0000_0001_0101 => "D | val",
            _ => "Error",
        };

        let store_in = match (instr & 0b0000_0000_0011_1000) >> 3 {
            0 => "None",
            1 => "RAM[A]",
            2 => "D",
            3 => "D and RAM[A]",
            4 => "A",
            5 => "A and RAM[A]",
            6 => "A and D",
            7 => "A, D, and RAM[A]",
            _ => "Error",
        };

        let jump = match instr & 0b0000_0000_0000_0111 {
            0 => "Never",
            1 => "If greater than",
            2 => "If equal",
            3 => "If greater than or equal",
            4 => "If less than",
            5 => "If not equal",
            6 => "If less than or equal",
            7 => "Always",
            _ => "Error",
        };

        println!(
            "{a_or_m}, compute '{cmp}' and store the value in {store_in}. {jump} jump to ROM[A]."
        )
    }
}
