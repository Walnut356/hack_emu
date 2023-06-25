#[derive(Clone, Debug, PartialEq)]
pub struct RAM {
    pub data: Vec<u16>,
}

impl RAM {
    pub fn new() -> Self {
        RAM {
            data: vec![0; 16384],
        }
    }

    pub fn cycle(&mut self, input: u16, address: u16, load: u8) -> u16 {
        if address > 0b0011_1111_1111_1111 {
            panic!("address too high, got: {address:?}")
        }
        if load > 0 {
            self.data[address as usize] = input;
        }

        self.data[address as usize]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstPtr {
    val: u16,
}

impl InstPtr {
    pub fn new() -> Self {
        InstPtr { val: 0 }
    }

    pub fn cycle(&mut self, input: u16, load: u8, inc: u8, reset: u8) -> u16 {
        if inc > 0 {
            self.val += 1;
        }
        if load > 0 {
            self.val = input;
        }
        if reset > 0 {
            self.val = 0;
        }

        self.val
    }
}
