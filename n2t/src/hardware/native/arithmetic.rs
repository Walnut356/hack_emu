/// just like gates this module is mostly irrelevant due to basic language features
/// things like the half and full adder aren't useful, and adder is just .wrapping_add().
///
pub fn adder(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
