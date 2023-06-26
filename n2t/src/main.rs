use n2t::hardware::logic_gate::alu::bench_alu;
use n2t::hardware::native::alu::*;
use n2t::utils::*;
use std::io::stdin;
use std::time::Instant;

fn main() {

    let y = 1000;
    let z = bitvec_from_int(y);
    let w = int_from_bitvec(&z);
    println!("{z:?}, {w:?}");
    // bench_alu();
    // bench_native();
    // let x: u16;
    // let mut s = String::new();

    // stdin().read_line(&mut s).unwrap();
    // s.pop();
    // x = s.parse().unwrap();

    // let now = Instant::now();
    // let y = bitvec_from_int(x);
    // let dur = now.elapsed();
    // println!("{:?}", dur.as_nanos());

    // let now = Instant::now();
    // let z = int_from_bitvec(&y);
    // let dur = now.elapsed();
    // println!("{:?}", dur.as_nanos());

    // println!("{y:?}");
    // println!("{z:?}");
}
