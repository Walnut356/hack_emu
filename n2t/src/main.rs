use n2t::alu::*;
use n2t::arithmetic::*;
use n2t::gates::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let val = ALU(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        &mut ControlBits {
            zx: 0,
            nx: 0,
            zy: 1,
            ny: 1,
            f: 0,
            no: 0,
            zr: 0,
            ng: 0,
        },
    );
    let dur = now.elapsed();
    println!("{:?}", dur.as_nanos());

    println!("{val:?}");
}
