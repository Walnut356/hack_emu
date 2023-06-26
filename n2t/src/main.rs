use n2t::hardware::logic_gate::alu::bench_alu;
use n2t::hardware::native::alu::*;
use n2t::hardware::logic_gate::cpu::Computer;
use n2t::utils::*;
use std::io::stdin;
use std::time::Instant;

fn main() {
    // bench_alu();
    // bench_native();
    // let x: u16;
    // let mut s = String::new();

    // stdin().read_line(&mut s).unwrap();
    // s.pop();
    // x = s.parse().unwrap();
    let mut comp = Computer::new(vec![
        0b0000000000000000,
        0b1111110000010000,
        0b0000000000000001,
        0b1111010011010000,
        0b0000000000001010,
        0b1110001100000001,
        0b0000000000000001,
        0b1111110000010000,
        0b0000000000001100,
        0b1110101010000111,
        0b0000000000000000,
        0b1111110000010000,
        0b0000000000000010,
        0b1110001100001000,
        0b0000000000001110,
        0b1110101010000111,
    ]);
    comp.ram.data[0] = 3;
    comp.ram.data[1] = 5;

    let now = Instant::now();
    comp.execute(false, false);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);

    // assert_eq!(comp.ram[2], 5);
    // comp.execute(true, true);
    // comp.ram[0] = 23456;
    // comp.ram[1] = 12345;
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // comp.execute(false, true);
    // assert_eq!(comp.ram[2], 23456)

    let dur = now.elapsed();
    println!("{:?}", dur.as_nanos());

    // let now = Instant::now();
    // let z = int_from_bitvec(&y);
    // let dur = now.elapsed();
    // println!("{:?}", dur.as_nanos());

    // println!("{y:?}");
    // println!("{z:?}");
}
