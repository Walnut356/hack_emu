use std::iter::zip;

use crate::logic_gate::gates::*;

// DFF is considered "fundamental", so while this is would realistically work, i'll just be using Vecs as the minimum
// so i don't have to refactor all of the prior logic to work with DFFs.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DFF {
    pub data: u8,
}

impl DFF {
    pub fn new() -> Self {
        DFF { data: 0 }
    }
    pub fn cycle(&mut self, input: u8, load: u8) {
        self.data = MUX(self.data, input, load);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub data: [u8; 16],
}

impl Register {
    pub fn new() -> Self {
        Register {
            data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn cycle(&mut self, input: &[u8; 16], load: u8) {
        for i in 0..16 {
            self.data[i] = MUX(self.data[i], input[i], load);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM8 {
    pub data: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8 {
            data: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
        }
    }

    pub fn cycle(&mut self, data: &[u8; 16], load: u8) {}
}
