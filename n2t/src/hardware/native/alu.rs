#![allow(non_snake_case)]

use super::cpu::ControlBits;
use enumflags2::BitFlags;

pub fn ALU(mut x: u16, mut y: u16, control: &mut BitFlags<ControlBits>) -> u16 {
    if control.contains(ControlBits::ZeroX) {
        x = 0
    };
    if control.contains(ControlBits::NotX) {
        x = !x
    };

    if control.contains(ControlBits::ZeroY) {
        y = 0
    };
    if control.contains(ControlBits::NotY) {
        y = !y
    };

    let mut result;

    if control.contains(ControlBits::FSelect) {
        result = x.wrapping_add(y);
    } else {
        result = x & y;
    }

    if control.contains(ControlBits::NotOut) {
        result = !result;
    }

    if result == 0 {
        control.insert(ControlBits::Zero);
    } else {
        control.remove(ControlBits::Zero);
    }

    if result >= 0b1000_0000_0000_0000 {
        control.insert(ControlBits::Neg);
    } else {
        control.remove(ControlBits::Neg);
    }

    result
}
