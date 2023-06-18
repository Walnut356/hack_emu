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
        if i < 16 {
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
        <th>1</th><td>1</td><td>1</td>
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

pub fn multi_NOT(a: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for i in a {
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
        <th>1</th><td>1</td><td>0</td>
    </tr>

</table>
*/
pub fn XOR(a: u8, b: u8) -> u8 {
    AND(NAND(a, b), OR(a, b))
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
