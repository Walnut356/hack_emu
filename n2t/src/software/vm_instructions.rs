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
        "{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@EQ_",
        eq_count,
        "\nD;JEQ\n",
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(EQ_",
        eq_count,
    )
}

pub fn lt(lt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n", // init push to true
        "@LT_",
        lt_count,
        "\nD;JLT\n", // if M-D is positive, leave as true, otherwise set to false
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(LT_",
        lt_count,
    )
}

/// compares the top 2 values on the stack, pushes -1 if
pub fn gt(gt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{})\n",
        pop(Segment::Constant, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@GT_",
        gt_count,
        "\nD;JGT\n", // jump if M-D is negative, i.e. M is greater than D
        "@SP\n",
        "A=M-1\n",
        "M=0\n",
        "(GT_",
        gt_count,
    )
}

pub fn jump_if_zero(dest: String) -> String {
    format!(
        "{}{}{}@{}\n{}",
        "@SP\n", "AM=M-1\n", "D=M\n", dest, "D;JNE\n"
    )
}

pub fn jump_uncond(dest: String) -> String {
    format!("@{dest}\n0;JMP\n")
}

pub fn func_return(func_label: &String) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        // store frame memory loc in R[14]
        "@LCL\n",
        "D=M\n",
        "@R14\n",
        "M=D\n",
        // store return address (RAM[frame-5]) in R15
        "@5\n",
        "D=A\n",
        "@R14\n",
        "A=M-D\n",
        // pop stack to RAM[arg.0]
        pop(Segment::Argument, Some("0")),
        // set stack to RAM[arg + 1]
        "@ARG\n",
        "D=M+1\n",
        "@SP\n",
        "M=D\n",
        // Restore THAT
        "@R14\n",
        "A=M-1\n",
        "D=M\n",
        "@THAT\n",
        "M=D\n",
        // Restore THIS
        "@2\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        "@THIS\n",
        "M=D\n",
        // Restore ARG
        "@3\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        "@ARG\n",
        "M=D\n",
        // Restore LCL
        "@4\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        "@LCL\n",
        "M=D\n",
        jump_uncond(func_label.to_owned()),
    )
}

// Drop-in instructions for use in compound statements

/// sets A to a pointer's base address
pub fn set_a_ptr(loc: &Segment) -> String {
    format!("@{}\nA=M\n", *loc)
}

/// sets A to `ind` offset of `loc` pointer's base address
pub fn set_a_offset(loc: &Segment, ind: &str) -> String {
    match *loc {
        Segment::Temp => {
            // for the idiomatic "R0-R15" virtual registers
            let off = ind.parse::<u16>().unwrap() + 5;
            format!("@{loc}{off}\n")
        }
        Segment::Pointer => {
            // pointer "segment" is just THIS and THAT registers
            if ind == "0" {
                format!("@THIS\n")
            } else {
                format!("@THAT\n")
            }
        }
        _ => format!("@{ind}\nD=A\n@{loc}\nA=D+M\n"),
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
    format!("@{loc}\nAM=M+1\n")
}

/// leaves A as the post-decr memory location
pub fn decr_ptr(loc: &Segment) -> String {
    format!("@{loc}\nAM=M-1\n")
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
