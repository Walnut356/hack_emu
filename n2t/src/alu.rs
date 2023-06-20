use crate::arithmetic::*;
use crate::gates::*;

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


/// naive solution, calculates everything. If i'm careful i can probably use mux and demux to skip unnecessary steps?
///
/// ~35000ns debug
///
/// ~3000ns release
pub fn ALU(mut x: Vec<u8>, mut y: Vec<u8>, control: &mut ControlBits) -> Vec<u8> {
    x = multi_MUX(&x, &multi_XOR(&x, &x), control.zx);
    x = multi_MUX(&x, &multi_NOT(&x), control.nx);

    y = multi_MUX(&y, &multi_XOR(&y, &y), control.zy);
    y = multi_MUX(&y, &multi_NOT(&y), control.ny);

    let mut result = multi_MUX(&multi_AND(&x, &y), &adder(&x, &y), control.f);

    result = multi_MUX(&result, &multi_NOT(&result), control.no);

    result
}
