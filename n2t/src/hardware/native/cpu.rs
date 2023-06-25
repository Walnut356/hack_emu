pub struct ScreenSize {
    width: u16,
    height: u16,
}

const START_OF_ROM: u16 = 0b1000_0000_0000_0000;
const SCREEN_MAP: u16 = 0x4000;
const SCREEN_BYTES: u16 = 8192;
const SCREEN_SIZE: ScreenSize = ScreenSize {
    width: 512,
    height: 256,
};
const KBD: u16 = 0x6000;

#[derive(Debug)]
pub struct CPU {
    pub d: u16,
    pub a: u16,
    pub m: u16,
    pub mem: Vec<u16>,
}

impl CPU {
    pub fn new() -> Self {
        let mem = vec![0; 32768];
        CPU {
            d: 0,
            a: 0,
            m: 0,
            mem,
        }
    }
}
