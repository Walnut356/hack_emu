use std::time::Instant;

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

pub fn bench_native() {
    // use n2t::native::arithmetic::*;
    // use n2t::native::gates::*;
    let mut control = ControlBits {
        zx: false,
        nx: false,
        zy: true,
        ny: true,
        f: true,
        no: false,
        zr: false,
        ng: false,
    };
    let mut val = 0;

    let now = Instant::now();
    for i in 0..10000 {
        val = ALU(17, 3, &mut control);
    }
    let dur = now.elapsed();
    println!("{:?}", dur.as_micros());

    println!("result = {val:?}");
}
