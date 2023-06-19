use n2t::gates::*;

fn thing(a:u8, b:u8) -> u8 {
    NAND(NOT(a), b)
}

fn main() {
    let result = truth_table(thing);
    print_table(result);

}
