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
    pub mod compiler;
    pub mod vm;
    pub mod vm_instructions;
}

pub mod utils;

pub const STACK_START: usize = 256;
pub const STACK_MAX: usize = 2047;

pub const STACK_POINTER: usize = 0;
pub const LCL: usize = 1;
pub const ARG: usize = 2;
pub const THIS: usize = 3;
pub const THAT: usize = 4;
pub const TEMP_START: usize = 5;
pub const TEMP_MAX: usize = 12;
pub const VAR_START: usize = 13;
pub const VAR_MAX: usize = 15;
pub const STATIC_START: usize = 16;
pub const STATIC_MAX: usize = 255;
