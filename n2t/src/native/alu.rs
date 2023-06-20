#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ControlBits {
    // in
    pub zx: bool,
    pub nx: bool,
    pub zy: bool,
    pub ny: bool,
    pub f: bool,
    pub no: bool,

    // out
    pub zr: bool,
    pub ng: bool,
}

pub fn ALU(mut x: u16, mut y: u16, control: &mut ControlBits) -> u16 {
    if control.zx {
        x = 0
    };
    if control.nx {
        x = !x
    };

    if control.zy {
        y = 0
    };
    if control.ny {
        y = !y
    };

    let mut result;

    if control.f {
        result = x.wrapping_add(y);
    } else {
        result = x & y;
    }

    control.zr = result == 0;
    control.ng = result < 0;

    result
}
