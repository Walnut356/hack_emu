use n2t::gates::*;
use n2t::arithmetic::*;

fn main() {
    assert_eq!(half_adder(1, 1), full_adder(1, 1, 0));
}
