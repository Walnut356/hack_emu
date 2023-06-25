/// most of the functions of gates are supplanted by basics like &, |, ^, etc.
/// for consistency i'll go ahead and write it out though

pub fn NAND(a: u16, b: u16) -> u16 {
    !(a & b)
}

pub fn NOT(a: u16) -> u16 {
    !a
}

pub fn AND(a: u16, b: u16) -> u16 {
    a & b
}

pub fn OR(a: u16, b: u16) -> u16 {
    a | b
}

pub fn XOR(a: u16, b: u16) -> u16 {
    a ^ b
}

pub fn MUX(a: u16, b: u16, sel: u16) -> u16 {
    match sel {
        0 => a,
        1 => b,
        _ => panic!("MUX requires selector to be 0 or 1, got {sel}"),
    }
}

pub fn MUX_4(a: u16, b: u16, c: u16, d: u16, sel: u16) -> u16 {
    match sel {
        0 => a,
        1 => b,
        2 => c,
        3 => d,
        _ => panic!("MUX requires selector to be a number 0-3, got {sel}"),
    }
}

pub fn MUX_8(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16, sel: u16) -> u16 {
    match sel {
        0 => a,
        1 => b,
        2 => c,
        3 => d,
        4 => e,
        5 => f,
        6 => g,
        7 => h,
        _ => panic!("MUX requires selector to be a number 0-7, got {sel}"),
    }
}

pub fn DEMUX(input: u16, sel: u16) -> (u16, u16) {
    match sel {
        0 => (input, 0),
        1 => (0, input),
        _ => panic!("MUX requires selector to be 0 or 1, got {sel}"),
    }
}

pub fn DEMUX_4(input: u16, sel: u16) -> (u16, u16, u16, u16) {
    match sel {
        0 => (input, 0, 0, 0),
        1 => (0, input, 0, 0),
        2 => (0, 0, input, 0),
        3 => (0, 0, 0, input),
        _ => panic!("MUX requires selector to be 0 or 1, got {sel}"),
    }
}

pub fn DEMUX_8(input: u16, sel: u16) -> (u16, u16, u16, u16, u16, u16, u16, u16) {
    match sel {
        0 => (input, 0, 0, 0, 0, 0, 0, 0),
        1 => (0, input, 0, 0, 0, 0, 0, 0),
        2 => (0, 0, input, 0, 0, 0, 0, 0),
        3 => (0, 0, 0, input, 0, 0, 0, 0),
        4 => (0, 0, 0, 0, input, 0, 0, 0),
        5 => (0, 0, 0, 0, 0, input, 0, 0),
        6 => (0, 0, 0, 0, 0, 0, input, 0),
        7 => (0, 0, 0, 0, 0, 0, 0, input),
        _ => panic!("MUX requires selector to be 0 or 1, got {sel}"),
    }
}