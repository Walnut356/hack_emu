use crate::{software::vm::Segment, utils::u16_from_i16};
use lazy_static::lazy_static;

/// calls an infinite loop at the end of the program
pub static INFINITE_LOOP: &str = "(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP\n";
pub static INCR_STACK: &str = "@SP\nAM=M+1\n";
pub static DECR_STACK: &str = "@SP\nAM=M-1\n";
/// sets `A` to the memory location of the top value on the stack (`RAM[SP - 1]`) WITHOUT modifying the stack pointer
pub static SET_A_STACK_TOP: &str = "@SP\nA=M-1\n";
/// Pops the top value from the stack and stores it in `D`
pub static POP_STACK: &str = "@SP\nAM=M-1\nD=M\n";
/// Assumes `D` is set to value to be pushed, **does not modify the stack pointer**. `A` is set to the address the SP is
/// pointing to.
pub static SET_STACK_D: &str = "@SP\nA=M\nM=D\n";
/// Assumes that A is set to a pointer, sets A to the memory address pointed to by A
pub static DEREF_A: &str = "A=M\n";

lazy_static! {
    pub static ref BOOTSTRAP: String = format!(
        "{}{}{}{}{}",
        "//init 'stack' pointer\n@256\nD=A\n@SP\nM=D\n",
        "//call Sys.init\n",
        func_call(&"Sys.init".to_owned(), &"Sys.init$ret0".to_owned(), "0"),
        "//Sys.init should never return, but just in case it does, here's another loop trap\n",
        "@INFINITE_LOOP\n0;JMP\n",
    );

    /// Consumes the top 2 values of the stack, bitwise ANDs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref ADD: String = format!(
        "{}{}{}",
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D+M\n",
    );

    /// Subracts the top value of the stack from the second-to-top value of the stack and stores the result on the new top
    /// of the stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref SUB: String = format!(
        "{}{}{}",
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=M-D\n",
    );

    /// Consumes the top 2 values of the stack, bitwise ANDs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref AND: String =format!(
        "{}{}{}",
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D&M\n",
    );

    /// Consumes the top 2 values of the stack, bitwise ORs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref OR: String =format!(
        "{}{}{}",
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D|M\n",
    );

    pub static ref NOT: String = format!("{SET_A_STACK_TOP}M=!M\n");

    pub static ref NEG: String = format!("{SET_A_STACK_TOP}M=-M\n");

}

pub fn set_a(val: &str) -> String {
    format!("@{val}\n")
}

/// sets A to a pointer's base address
pub fn set_a_ptr(loc: &Segment) -> String {
    format!("@{loc}\n",)
}

/// sets A to `ind` offset of `loc` pointer's base address
pub fn set_a_offset(loc: &Segment, ind: &str) -> String {
    match *loc {
        Segment::Temp => {
            // for the Temp segment (registers R5-R12)
            let off = u16_from_i16(ind.parse::<i16>().unwrap() + 5);
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

/// Sets `D` to the value that the `loc` pointer is pointing to
pub fn set_d_deref(loc: &Segment) -> String {
    format!("{}{}{}", set_a_ptr(loc), DEREF_A, "D=M\n")
}

/// Sets the value `loc` is pointing to to `D`
pub fn set_deref_d(loc: &Segment) -> String {
    format!("{}{}{}", set_a_ptr(loc), DEREF_A, "M=D\n")
}

/// sets `D` to `val`
pub fn set_d_const(val: &str) -> String {
    format!("@{val}\nD=A\n")
}

/// Pushes a value to the stack.
///
/// If the location is a memory segment (e.g. "Local"), `val` is treated as an index into
/// the memory segment, and the value at the resultant memory location is pushed onto the stack.
///
/// If `val` is None and the location is a memory segment, the value of the pointer itself is stored to the stack.
pub fn push<'a>(loc: Segment, val: Option<&str>) -> String {
    use Segment::*;
    match loc {
        Stack => {
            if let Some(i) = val {
                format!("{}{}{}", set_d_const(i), set_deref_d(&loc), INCR_STACK)
            } else {
                panic!("Got instruction to push constant with no value");
            }
        }
        _ => {
            if let Some(i) = val {
                format!(
                    "{}{}{}{}",
                    set_a_offset(&loc, i),
                    "D=M\n",
                    set_deref_d(&Segment::Stack),
                    INCR_STACK,
                )
            } else {
                format!(
                    "{}{}{}{}",
                    set_a_ptr(&loc),
                    "D=A\n",
                    SET_STACK_D,
                    INCR_STACK,
                )
            }
        }
    }
}

/// Pops a value off of the stack and stores it in D if val = None, otherwise stores it in RAM[loc+val]
pub fn pop(loc: Segment, val: Option<&str>) -> String {
    if loc == Segment::Stack {
        format!("{}{}", DECR_STACK, "D=M\n")
    } else {
        let ind = val.unwrap();
        format!(
            "{}{}{}{}{}{}{}",
            set_a_offset(&loc, ind), // e.g. set A local[0]'s address
            "D=A\n",                 // set D to local[0]'s address
            set_mem_d(&Segment::Static("R13".to_owned())), // store local[0]'s address in R13
            POP_STACK,               // pop stack into D
            set_a("R13"),
            DEREF_A, // Access local[0]'s address from R13
            "M=D\n"  // set RAM[local[0]] to popped value
        )
    }
}

// comparisons
pub fn eq(eq_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}({}{})\n",
        pop(Segment::Stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@EQ_",
        eq_count,
        "\nD;JEQ\n",
        SET_A_STACK_TOP,
        "M=0\n",
        "EQ_",
        eq_count,
    )
}

pub fn lt(lt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}({}{})\n",
        pop(Segment::Stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n", // init push to true
        "@LT_",
        lt_count,
        "\nD;JLT\n", // if M-D is positive, leave as true, otherwise set to false
        SET_A_STACK_TOP,
        "M=0\n",
        "LT_",
        lt_count,
    )
}

/// compares the top 2 values on the stack, pushes -1 if
pub fn gt(gt_count: u16) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}({}{})\n",
        pop(Segment::Stack, None),
        "A=A-1\n",
        "D=M-D\n",
        "M=-1\n",
        "@GT_",
        gt_count,
        "\nD;JGT\n", // jump if M-D is negative, i.e. M is greater than D
        SET_A_STACK_TOP,
        "M=0\n",
        "GT_",
        gt_count,
    )
}

pub fn jump_if_zero(dest: String) -> String {
    format!("{}@{}\n{}", POP_STACK, dest, "D;JNE\n")
}

pub fn jump_uncond(dest: String) -> String {
    format!("@{dest}\n0;JMP\n")
}

pub fn func_return() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        // store frame memory loc in R[14]
        set_d_mem(&Segment::Local),
        set_mem_d(&Segment::Static("R14".to_owned())),
        // store return address (RAM[frame-5]) in R15
        set_a("5"),
        "D=A\n",
        set_a("R14"),
        "D=M-D\n",
        set_mem_d(&Segment::Static("R15".to_owned())),
        // pop stack to RAM[arg.0]
        pop(Segment::Argument, Some("0")),
        // set stack to RAM[arg + 1]
        set_a("ARG"),
        "D=M+1\n",
        set_mem_d(&Segment::Stack),
        // Restore THAT
        "@R14\nA=M-1\n",
        "D=M\n",
        set_mem_d(&Segment::That),
        // Restore THIS
        "@2\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        set_mem_d(&Segment::This),
        // Restore ARG
        "@3\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        set_mem_d(&Segment::Argument),
        // Restore LCL
        "@4\nD=A\n@R14\nA=M-D\n",
        "D=M\n",
        set_mem_d(&Segment::Local),
        set_a("R15"),
        DEREF_A,
        "0;JMP\n",
    )
}

pub fn func_call(func_label: &String, return_addr: &String, n_args: &str) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}@{}\n{}({})\n",
        push(Segment::Stack, Some(return_addr)),
        // this is kindof a hack but i'd have to rework quite a few things to get this to work "properly"
        set_d_mem(&Segment::Local),
        SET_STACK_D,
        INCR_STACK,
        set_d_mem(&Segment::Argument),
        SET_STACK_D,
        INCR_STACK,
        set_d_mem(&Segment::This),
        SET_STACK_D,
        INCR_STACK,
        set_d_mem(&Segment::That),
        SET_STACK_D,
        INCR_STACK,
        // Set ARG to SP-5-n_args
        set_a(n_args),
        "D=A\n",
        "@5\n",
        "D=A-D\n",
        "@SP\n",
        "D=M-D\n",
        set_mem_d(&Segment::Argument),
        // Set LCL to SP
        set_d_mem(&Segment::Stack),
        set_mem_d(&Segment::Local),
        func_label,
        "0;JMP\n",
        return_addr,
    )
}
