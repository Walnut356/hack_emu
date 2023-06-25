use n2t::utils::*;

fn  main() {
    let x: u16 = 17;
    let y = bitvec_from_int(x);
    let z = int_from_bitvec(&y);
    println!("{y:?}");
    println!("{z:?}");
}
