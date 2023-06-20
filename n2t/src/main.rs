use n2t::alu::*;
use n2t::arithmetic::*;
use n2t::gates::*;
use std::time::Instant;

fn main() {
    let z = is_zero(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let nz = is_zero(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

    println!("{z:?}");
    println!("{nz:?}");
}

fn bench_alu() {
    let mut control = ControlBits {
        zx: 0,
        nx: 0,
        zy: 1,
        ny: 1,
        f: 0,
        no: 0,
        zr: 0,
        ng: 0,
    };
    let mut val = vec![0; 16];

    let now = Instant::now();
    for i in 0..10000 {
        val = ALU(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            &mut control,
        );
    }
    let dur = now.elapsed();
    println!("{:?}", dur.as_nanos());

    println!("{val:?}");
}
