#![allow(non_snake_case)]

use std::iter::zip;

pub fn truth_table(op: fn(u8, u8) -> u8) -> [[u8; 2]; 2] {
    let mut result = [[0, 0], [0, 0]];

    result[0][0] = op(0, 0);
    result[0][1] = op(0, 1);
    result[1][0] = op(1, 0);
    result[1][1] = op(1, 1);
    result
}

pub fn multi_truth_table(op: fn(u8, u8, u8) -> u8) -> [u8; 8] {
    let mut result = [0; 8];

    result[0] = op(0, 0, 0);
    result[1] = op(0, 1, 0);
    result[2] = op(1, 0, 0);
    result[3] = op(1, 1, 0);
    result[4] = op(0, 0, 1);
    result[5] = op(0, 1, 1);
    result[6] = op(1, 0, 1);
    result[7] = op(1, 1, 1);

    result
}

pub fn print_table(result: [[u8; 2]; 2]) {
    println!("   0  1 ");
    println!("0 {:?}", result[0]);
    println!("1 {:?}", result[1]);
}

pub fn bitvec_to_u8(a: &Vec<u8>) -> u8 {
    let mut result = 0;
    for (i, j) in a.into_iter().enumerate() {
        result |= j;
        if i < 7 {
            result = result << 1;
        }
    }

    result
}

pub fn bitvec_to_u16(a: &Vec<u8>) -> u8 {
    let mut result = 0;
    for (i, j) in a.into_iter().enumerate() {
        result |= j;
        if i < 15 {
            result = result << 1;
        }
    }

    result
}

/**
Base gate for all other gates

<table>
    <caption>Truth Table</caption>
    <tr>
        <th> </th><th>0</th><th>1</th>
    </tr>
    <tr>
        <th>0</th><td>1</td><td>1</td>
    </tr>
    <tr>
        <th>1</th><td>1</td><td>0</td>
    </tr>

</table>
*/
pub fn NAND(a: u8, b: u8) -> u8 {
    if a & b == 1 {
        0
    } else {
        1
    }
}

/**
Opposite of input
*/
pub fn NOT(a: u8) -> u8 {
    NAND(a, a)
}

pub fn multi_NOT(a: &Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for &i in a {
        out.push(NOT(i));
    }
    out
}

/**
<table>
    <caption>Truth Table</caption>
    <tr>
        <th> </th><th>0</th><th>1</th>
    </tr>
    <tr>
        <th>0</th><td>0</td><td>0</td>
    </tr>
    <tr>
        <th>1</th><td>0</td><td>1</td>
    </tr>

</table>
*/
pub fn AND(a: u8, b: u8) -> u8 {
    NOT(NAND(a, b))
}

pub fn multi_AND(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for (&i, &j) in zip(a, b) {
        out.push(AND(i, j));
    }
    out
}

/**
<table>
    <caption>Truth Table</caption>
    <tr>
        <th> </th><th>0</th><th>1</th>
    </tr>
    <tr>
        <th>0</th><td>0</td><td>1</td>
    </tr>
    <tr>
        <th>1</th><td>1</td><td>1</td>
    </tr>

</table>
*/
pub fn OR(a: u8, b: u8) -> u8 {
    NAND(NOT(a), NOT(b))
}

pub fn multi_OR(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for (&i, &j) in zip(a, b) {
        out.push(OR(i, j));
    }
    out
}

/**
<table>
    <caption>Truth Table</caption>
    <tr>
        <th> </th><th>0</th><th>1</th>
    </tr>
    <tr>
        <th>0</th><td>0</td><td>1</td>
    </tr>
    <tr>
        <th>1</th><td>1</td><td>0</td>
    </tr>

</table>
*/
pub fn XOR(a: u8, b: u8) -> u8 {
    AND(NAND(a, b), OR(a, b))
}

pub fn multi_XOR(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for (&i, &j) in zip(a, b) {
        out.push(XOR(i, j));
    }
    out
}

/**
<table>
    <caption>Truth Table</caption>
    <tr>
        <th> a </th><th> b </th><th>sel</th><th>res</th>
    </tr>
    <tr>
        <td>0</td><td>0</td><td>0</td><th>0</th>
    </tr>
    <tr>
        <td>0</td><td>1</td><td>0</td><th>0</th>
    </tr>
    <tr>
        <td>1</td><td>0</td><td>0</td><th>1</th>
    </tr>
    <tr>
        <td>1</td><td>1</td><td>0</td><th>1</th>
    </tr>
    <tr>
        <td>0</td><td>0</td><td>1</td><th>0</th>
    </tr>
    <tr>
        <td>0</td><td>1</td><td>1</td><th>1</th>
    </tr>
    <tr>
        <td>1</td><td>0</td><td>1</td><th>0</th>
    </tr>
    <tr>
        <td>1</td><td>1</td><td>1</td><th>1</th>
    </tr>
</table>
*/
pub fn MUX(a: u8, b: u8, sel: u8) -> u8 {
    OR(AND(NOT(sel), a), AND(sel, b))
}

/// if sel == 0, return a
///
/// if sel == 1, return b
pub fn multi_MUX(a: &Vec<u8>, b: &Vec<u8>, sel: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..a.len() {
        out.push(MUX(a[i], b[i], sel));
    }
    out
}

pub fn MUX_4(a: u8, b: u8, c: u8, d: u8, s0: u8, s1: u8) -> u8 {
    let out1 = MUX(a, b, s1);
    let out2 = MUX(c, d, s1);
    MUX(out1, out2, s0)
}

pub fn multi_MUX_4(a: &Vec<u8>, b: &Vec<u8>, c: &Vec<u8>, d: &Vec<u8>, s0: u8, s1: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..a.len() {
        out.push(MUX_4(a[i], b[i], c[i], d[i], s0, s1));
    }
    out
}

pub fn MUX_8(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8, s0: u8, s1: u8, s2: u8) -> u8 {
    MUX(MUX_4(a, b, c, d, s1, s2), MUX_4(e, f, g, h, s1, s2), s0)
}

pub fn multi_MUX_8(
    a: &Vec<u8>,
    b: &Vec<u8>,
    c: &Vec<u8>,
    d: &Vec<u8>,
    e: &Vec<u8>,
    f: &Vec<u8>,
    g: &Vec<u8>,
    h: &Vec<u8>,
    s0: u8,
    s1: u8,
    s2: u8,
) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..a.len() {
        out.push(MUX_8(
            a[i], b[i], c[i], d[i], e[i], f[i], g[i], h[i], s0, s1, s2,
        ));
    }
    out
}

/**
<table>
    <caption>Truth Table</caption>
    <tr>
        <th>sel</th><th>out a</th><th>out b</th>
    </tr>
    <tr>
        <th>0</th><td>in</td><td>0</td>
    </tr>
    <tr>
        <th>1</th><td>0</td><td>in</td>
    </tr>

</table>
*/
pub fn DEMUX(input: u8, sel: u8) -> (u8, u8) {
    (AND(NOT(sel), input), AND(sel, input))
}

pub fn DEMUX_4(input: u8, s0: u8, s1: u8) -> (u8, u8, u8, u8) {
    let temp = DEMUX(input, s0);
    let (a, b) = DEMUX(temp.0, s1);
    let (c, d) = DEMUX(temp.1, s1);
    (a, b, c, d)
}

pub fn multi_DEMUX_4(input: &Vec<u8>, s0: u8, s1: u8) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut out = (
        Vec::with_capacity(input.len()),
        Vec::with_capacity(input.len()),
        Vec::with_capacity(input.len()),
        Vec::with_capacity(input.len()),
    );
    for &i in input {
        let temp = DEMUX_4(i, s0, s1);
        out.0.push(temp.0);
        out.1.push(temp.1);
        out.2.push(temp.2);
        out.3.push(temp.3);
    }
    out
}

pub fn DEMUX_8(input: u8, s0: u8, s1: u8, s2: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let temp = DEMUX(input, s0);
    let (a, b, c, d) = DEMUX_4(temp.0, s1, s2);
    let (e, f, g, h) = DEMUX_4(temp.1, s1, s2);

    (a, b, c, d, e, f, g, h)
}
