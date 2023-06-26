use std::time::Instant;

use crate::hardware::logic_gate::arithmetic::*;
use crate::hardware::logic_gate::gates::*;
use crate::utils::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ControlBits {
    // in
    pub zx: u8,
    pub nx: u8,
    pub zy: u8,
    pub ny: u8,
    pub f: u8,
    pub no: u8,

    // out
    pub zr: u8,
    pub ng: u8,
}

impl ControlBits {
    pub fn new() -> Self {
        ControlBits {
            zx: 0,
            nx: 0,
            zy: 0,
            ny: 0,
            f: 0,
            no: 0,
            zr: 0,
            ng: 0,
        }
    }
}

/// MUX solution
pub fn mux_ALU(mut x: Vec<u8>, mut y: Vec<u8>, control: &mut ControlBits) -> Vec<u8> {
    x = multi_MUX(&x, &multi_XOR(&x, &x), control.zx);
    x = multi_MUX(&x, &multi_NOT(&x), control.nx);

    y = multi_MUX(&y, &multi_XOR(&y, &y), control.zy);
    y = multi_MUX(&y, &multi_NOT(&y), control.ny);

    let mut result = multi_MUX(&multi_AND(&x, &y), &adder(&x, &y), control.f);

    result = multi_MUX(&result, &multi_NOT(&result), control.no);

    result
}

/// MUX_4 solution
pub fn ALU(mut x: &Vec<u8>, mut y: &Vec<u8>, control: &mut ControlBits) -> Vec<u8> {
    let not_x = &multi_NOT(&x);
    let not_y = &multi_NOT(&y);
    let zero = &multi_XOR(&x, &x);
    let not_zero = &multi_NOT(zero);

    let temp_x = multi_MUX_4(x, not_x, zero, not_zero, control.zx, control.nx);

    let temp_y = multi_MUX_4(y, not_y, zero, not_zero, control.zy, control.ny);

    let x_and_y = multi_AND(&x, &y);
    let x_plus_y = adder(&x, &y);

    let mut result = multi_MUX_4(
        &x_and_y,
        &multi_NOT(&x_and_y),
        &x_plus_y,
        &multi_NOT(&x_plus_y),
        control.f,
        control.no,
    );

    control.ng = *result.last().unwrap();
    control.zr = is_zero(&result);

    result
}

pub fn bench_alu() {
    let mut control = ControlBits {
        zx: 0,
        nx: 0,
        zy: 1,
        ny: 1,
        f: 1,
        no: 0,
        zr: 0,
        ng: 0,
    };
    let mut val = vec![0; 16];

    let now = Instant::now();
    for i in 0..10000 {
        val = ALU(
            &bitvec_from_int(17),
            &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            &mut control,
        );
    }
    let dur = now.elapsed();
    println!("{:?}", dur.as_micros());

    println!("result = {:?}", int_from_bitvec(&val));
}
