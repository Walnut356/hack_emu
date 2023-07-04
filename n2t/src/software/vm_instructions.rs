use crate::software::vm::Segment;

/// pushes a value to the stack. If the location is a memory segment (e.g. "Local"), val is treated as an index into
/// the memory segment, and the value at the resultant memory location is pushed onto the stack.
pub fn push<'a>(loc: Segment, val: &str) -> String {
    let mut result = String::new();

    use Segment::*;
    match loc {
        Constant => result.push_str(
            format!("{}{}{}", set_d_const(val), set_mem_d(&loc), incr_ptr(&loc)).as_str(),
        ),
        // base offset stored at RAM[1]
        _ => result.push_str(
            format!(
                "{}{}{}{}",
                set_a_offset(&loc, val),
                "D=M\n",
                set_mem_d(&Segment::Constant),
                incr_ptr(&Segment::Constant)
            )
            .as_str(),
        ),
    }

    result
}

/// pops a value off of the stack and stores it in D if val = None, otherwise stores it in RAM[loc+val]
pub fn pop(loc: Segment, val: Option<&str>) -> String {
    if loc == Segment::Constant {
        format!("{}{}", decr_ptr(&loc), "D=M\n")
    } else {
        let ind = val.unwrap();
        format!(
            "{}{}{}{}{}{}{}{}",
            set_a_offset(&loc, ind), // e.g. set A local[0]'s address
            "D=A\n",                 // set D to local[0]'s address
            "@R13\n",
            "M=D\n",                      // store local[0]'s address in R13
            pop(Segment::Constant, None), // pop stack into D
            "@R13\n",
            "A=M\n", // Access local[0]'s address from R13
            "M=D\n"  // set RAM[local[0]] to popped value
        )
    }
}

// binary ops
pub fn add() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::Constant, None),
        decr_ptr(&Segment::Constant),
        "M=D+M\n",
        incr_ptr(&Segment::Constant)
    )
}

pub fn sub() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::Constant, None),
        decr_ptr(&Segment::Constant),
        "M=M-D\n",
        incr_ptr(&Segment::Constant)
    )
}

pub fn and() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::Constant, None),
        decr_ptr(&Segment::Constant),
        "M=D&M\n",
        incr_ptr(&Segment::Constant)
    )
}

pub fn or() -> String {
    format!(
        "{}{}{}{}",
        pop(Segment::Constant, None),
        decr_ptr(&Segment::Constant),
        "M=D|M\n",
        incr_ptr(&Segment::Constant)
    )
}

// unary ops
pub fn not() -> String {
    // No need to manipulate the stack pointer when the value is being removed and put straight back on.
    format!("@SP\nA=M-1\nM=!M\n")
}

pub fn neg() -> String {
    format!("@SP\nA=M-1\nM=-M\n")
}

// comparisons
pub fn eq(eq_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@EQ_",
        eq_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(EQ_",
        eq_count,
    )
}

pub fn lt(lt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@LT_",
        lt_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(LT_",
        lt_count,
    )
}

pub fn gt(gt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@GT_",
        gt_count,
        "\nD;JEQ\n",
        "// if not equal\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(GT_",
        gt_count,
    )
}

// Drop-in instructions for use in compound statements

/// sets A to a pointer's base address
pub fn set_a_ptr(loc: &Segment) -> String {
    format!("@{}\nA=M\n", *loc)
}

/// sets A to `ind` offset of `loc` pointer's base address
pub fn set_a_offset(loc: &Segment, ind: &str) -> String {
    // for the idiomatic "R0-R15" virtual registers
    if *loc == Segment::Temp {
        let off = ind.parse::<u16>().unwrap() + 5;
        format!("@{}{off}\n", *loc)
    } else {
        format!("@{ind}\nD=A\n@{}\nA=D+M\n", *loc)
    }
}

/// sets `RAM[loc]` to the value stored in `D`
pub fn set_mem_d(loc: &Segment) -> String {
    format!("{}M=D\n", set_a_ptr(loc))
}

/// sets `D` to the value in `RAM[loc]`
pub fn set_d_mem(loc: &Segment) -> String {
    format!("{}D=M\n", set_a_ptr(loc))
}

/// sets `D` to `val`
pub fn set_d_const(val: &str) -> String {
    format!("@{val}\nD=A\n")
}

/// leaves A as the post-incr memory location
pub fn incr_ptr(loc: &Segment) -> String {
    format!("@{}\nAM=M+1\n", *loc)
}

/// leaves A as the post-decr memory location
pub fn decr_ptr(loc: &Segment) -> String {
    format!("@{}\nAM=M-1\n", *loc)
}

/// sets stack pointer to 256 and calls Sys.Init
pub fn init_program() -> String {
    format!("{}", "//init 'stack' pointer\n@256\nD=A\n@SP\nM=D\n",)
    // TODO call Sys.Init
}

/// calls an infinite loop at the end of the program
pub fn finalize_program() -> &'static str {
    "(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP"
}
