#![allow(unused_imports)]

use n2t::hardware::native::cpu::Computer;
use n2t::software::assembler::*;
use n2t::software::vm::*;
use n2t::utils::*;
use prettytable::ptable;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::iter::zip;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let path1 = Path::new("./test_files/ch 10/ExpressionLessSquare/SquareGame.xml");
    let mut file1 = File::open(path1).unwrap();
    let mut t_1 = String::new();
    file1.read_to_string(&mut t_1).unwrap();
    let path2 = Path::new("./test_files/ch 10/ExpressionLessSquare/SquareGameTExample.xml");
    let mut file2 = File::open(path2).unwrap();
    let mut t_2 = String::new();
    file2.read_to_string(&mut t_2).unwrap();

    for (a, b) in zip(t_1.lines(), t_2.lines()) {
        assert_eq!(a, b)
    }
}
