use std::iter::zip;

use crate::hardware::logic_gate::arithmetic::incrementer;
use crate::hardware::logic_gate::gates::*;

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

// #[derive(Clone, Debug, PartialEq)]
// pub struct Register {
//     pub data: [DFF; 16],
// }

// impl Register {
//     pub fn new() -> Self {
//         Register {
//             data: [
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//                 DFF::new(),
//             ],
//         }
//     }

//     fn cycle(&mut self, input: &[u8; 16], load: u8) {
//         for i in 0..16 {
//             self.data[i].cycle(input[i], load);
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub data: Vec<u8>,
}

impl Register {
    pub fn new() -> Self {
        Register { data: vec![0; 16] }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, load: u8) {
        for i in 0..16 {
            self.data[i] = MUX(self.data[i], input[i], load);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM8 {
    pub reg: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8 {
            reg: [
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

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 3], load: u8) -> Vec<u8> {
        let temp = DEMUX_8(load, address[0], address[1], address[2]);
        for i in 0..16 {
            self.reg[i].cycle(input, temp[i]);
        }
        multi_MUX_8(
            &self.reg[0].data,
            &self.reg[1].data,
            &self.reg[2].data,
            &self.reg[3].data,
            &self.reg[4].data,
            &self.reg[5].data,
            &self.reg[6].data,
            &self.reg[7].data,
            address[0],
            address[1],
            address[2],
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM64 {
    pub ram8: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> Self {
        RAM64 {
            ram8: [
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
            ],
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 6], load: u8) -> Vec<u8> {
        let temp = DEMUX_8(load, address[3], address[4], address[5]);
        let mut out = Vec::new();
        for i in 0..16 {
            out.push(self.ram8[i].cycle(input, address[0..=2].try_into().unwrap(), temp[i]));
        }

        multi_MUX_8(
            &out[0], &out[1], &out[2], &out[3], &out[4], &out[5], &out[6], &out[7], address[3],
            address[4], address[5],
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM512 {
    pub ram64: [RAM64; 8],
}

impl RAM512 {
    pub fn new() -> Self {
        RAM512 {
            ram64: [
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
            ],
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 9], load: u8) -> Vec<u8> {
        let temp = DEMUX_8(load, address[6], address[7], address[8]);
        let mut out = Vec::new();
        for i in 0..16 {
            out.push(self.ram64[i].cycle(input, address[0..=5].try_into().unwrap(), temp[i]));
        }

        multi_MUX_8(
            &out[0], &out[1], &out[2], &out[3], &out[4], &out[5], &out[6], &out[7], address[6],
            address[7], address[8],
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM4K {
    pub ram512: [RAM512; 8],
}

impl RAM4K {
    pub fn new() -> Self {
        RAM4K {
            ram512: [
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
            ],
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 12], load: u8) -> Vec<u8> {
        let temp = DEMUX_8(load, address[9], address[10], address[11]);
        let mut out = Vec::new();
        for i in 0..16 {
            out.push(self.ram512[i].cycle(input, address[0..=8].try_into().unwrap(), temp[i]));
        }

        multi_MUX_8(
            &out[0],
            &out[1],
            &out[2],
            &out[3],
            &out[4],
            &out[5],
            &out[6],
            &out[7],
            address[9],
            address[10],
            address[11],
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM16K {
    pub ram4k: [RAM4K; 8],
}

impl RAM16K {
    pub fn new() -> Self {
        RAM16K {
            ram4k: [
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
                RAM4K::new(),
            ],
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 14], load: u8) -> Vec<u8> {
        let temp = DEMUX_4(load, address[12], address[13]);
        let mut out = Vec::new();
        for i in 0..16 {
            out.push(self.ram4k[i].cycle(input, address[0..=11].try_into().unwrap(), temp[i]));
        }

        multi_MUX_4(&out[0], &out[1], &out[2], &out[3], address[12], address[13])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RAM32K {
    pub ram4k: [RAM16K; 2],
}

impl RAM32K {
    pub fn new() -> Self {
        RAM32K {
            ram4k: [RAM16K::new(), RAM16K::new()],
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, address: &[u8; 15], load: u8) -> Vec<u8> {
        let temp = DEMUX(load, address[14]);
        let mut out = Vec::new();
        for i in 0..16 {
            out.push(self.ram4k[i].cycle(input, address[0..=13].try_into().unwrap(), temp[i]));
        }

        multi_MUX(&out[0], &out[1], address[14])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstPtr {
    pub val: Register,
}

impl InstPtr {
    pub fn new() -> Self {
        InstPtr {
            val: Register::new(),
        }
    }

    pub fn cycle(&mut self, input: &Vec<u8>, load: u8, inc: u8, reset: u8) -> &Vec<u8> {
        let plus_one = incrementer(&self.val.data);
        let temp1 = multi_MUX(&self.val.data, &plus_one, inc);
        let temp2 = multi_MUX(&temp1, &input, load);
        let out = multi_MUX(&temp2, &vec![0; 16], reset);

        self.val.cycle(&out, 1);
        &self.val.data
    }
}
