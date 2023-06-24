#[derive(Clone, Debug, PartialEq)]
pub struct RAM16K {
    pub data: Vec<Vec<Vec<Vec<Vec<u16>>>>>,
}

impl RAM16K {
    pub fn new() -> Self {
        RAM16K {
            data: vec![vec![vec![vec![vec![0u16; 8]; 8]; 8]; 8]; 4],
        }
    }

    pub fn cycle(&mut self, input: u16, address: u16, load: bool) -> u16 {
        let l1 = ((address & 0b0011_0000_0000_1100) << 12) as usize;
        let l2 = ((address & 0b0000_0000_0111_0000) << 9) as usize;
        let l3 = ((address & 0b0000_0011_1000_0000) << 6) as usize;
        let l4 = ((address & 0b0001_1100_0000_0000) << 3) as usize;
        let l5 = (address & 0b1110_0000_0000_0000) as usize;

        let out = &mut self.data[l1][l2][l3][l4][l5];

        if load {
            *out = input;
        }

        out.clone()
    }
}
