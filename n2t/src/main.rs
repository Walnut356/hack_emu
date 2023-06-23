use n2t::logic_gate;
use n2t::native;
use std::time::Instant;

fn main() {
    let vec: Vec<u8> = vec![0, 1, 2, 3, 4];
    let res: &[u8; 3] = vec[0..3].try_into().unwrap();

    println!("{:?}", res);
    println!("{:?}", vec);
}
