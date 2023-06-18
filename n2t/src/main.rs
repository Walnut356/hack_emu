use n2t::gates::*;

fn main() {
    let thing = vec![1, 0, 0, 0, 0, 0, 0, 0];
    let result = bitvec_to_u8(&thing);

    println!("{result:?}");
}
