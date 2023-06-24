use n2t::logic_gate;
use n2t::native;
use std::time::Instant;

fn main() {
    let vec = 0b1000_0000_0000_0000;
    let res = 0b0000_0000_0000_0001;

    println!("{:?}", res);
    println!("{:?}", vec);
}
