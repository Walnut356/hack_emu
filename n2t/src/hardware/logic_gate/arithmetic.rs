use std::iter::zip;

use crate::hardware::logic_gate::gates::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Add {
    pub sum: u8,
    pub carry: u8,
}

pub fn half_adder(a: u8, b: u8) -> Add {
    Add {
        sum: XOR(a, b),
        carry: AND(a, b),
    }
}

pub fn full_adder(a: u8, b: u8, c: u8) -> Add {
    let s1 = half_adder(a, b);
    let s2 = half_adder(s1.sum, c);

    Add {
        sum: s2.sum,
        carry: OR(s1.carry, s2.carry),
    }
}

pub fn adder(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(a.len());
    let mut c = 0;

    for (&i, &j) in zip(a, b).rev() {
        let temp = full_adder(i, j, c);
        result.push(temp.sum);
        c = temp.carry;
    }

    result.into_iter().rev().collect()
}

pub fn incrementer(a: &[u8]) -> Vec<u8> {
    let mut one = vec![0; a.len()];
    let last = one.last_mut().unwrap();
    *last = 1;
    let mut result = Vec::with_capacity(a.len());
    let mut c = 0;

    for (&i, &j) in zip(a, &one).rev() {
        let temp = full_adder(i, j, c);
        result.push(temp.sum);
        c = temp.carry;
    }
    result.into_iter().rev().collect()
}

pub fn is_zero(a: &[u8]) -> u8 {
    let mut temp = a.to_vec();

    while temp.len() > 1 {
        for (i, (j, &k)) in zip(temp.clone(), temp.clone().iter().rev()).enumerate() {
            temp[i] = OR(j, k);
        }
        temp.truncate(temp.len() / 2);
    }

    NOT(temp[0])
}
