use std::fmt::Display;

use crate::{software::vm::Segment, utils::u16_from_i16};
use concat_string::concat_string;
use lazy_static::lazy_static;
use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
pub enum Reg {
    A,
    D,
    M,
    AD,
    AM,
    DM,
    ADM,
}

/// calls an infinite loop at the end of the program
pub const INFINITE_LOOP: &str = "(INFINITE_LOOP)\n@INFINITE_LOOP\n0;JMP\n";
pub const INCR_STACK: &str = "@SP\nAM=M+1\n";
pub const DECR_STACK: &str = "@SP\nAM=M-1\n";
/// sets `A` to the memory location of the top value on the stack (`RAM[SP - 1]`) WITHOUT modifying the stack pointer
pub const SET_A_STACK_TOP: &str = "@SP\nA=M-1\n";
/// Pops the top value from the stack and stores it in `D`
pub const POP_STACK: &str = "@SP\nAM=M-1\nD=M\n";
/// Assumes `D` is set to value to be pushed, **does not modify the stack pointer**. `A` is set to the address the SP is
/// pointing to.
pub const SET_STACK_D: &str = "@SP\nA=M\nM=D\n";
/// Assumes that A is set to a pointer, sets A to the memory address pointed to by A
pub const DEREF_A: &str = "A=M\n";
/// Pushes D to the stack, increments stack pointer
pub const PUSH_D_STACK: &str = "@SP\nA=M\nM=D\n@SP\nAM=M+1\n";
pub const JUMP_UNCOND: &str = "0;JMP\n";

/// Returns:
/// ```no_test
///  "{dest}={src}\n"
/// ```
pub fn load(dest: Reg, src: &str) -> String {
    concat_string!(dest.to_string(), "=", src, "\n")
}

/// Returns:
/// ```no_test
///  "@{val}\n"
/// ```
pub fn load_const<D: Display>(val: D) -> String {
    concat_string!("@", val.to_string(), "\n")
}

/// Returns:
/// ```no_test
///  "({val})\n"
/// ```
pub fn label(val: &str) -> String {
    concat_string!("(", val, ")\n")
}

/// Returns:
/// ```no_test
///  "{comp};JEQ\n"
/// ```
pub fn jeq(comp: &str) -> String {
    concat_string!(comp, ";JEQ\n")
}

/// Returns:
/// ```no_test
///  "{comp};JLT\n"
/// ```
pub fn jlt(comp: &str) -> String {
    concat_string!(comp, ";JLT\n")
}

/// Returns:
/// ```no_test
///  "{comp};JGT\n"
/// ```
pub fn jgt(comp: &str) -> String {
    concat_string!(comp, ";JGT\n")
}

/// Returns:
/// ```no_test
///  "{comp};JNE\n"
/// ```
pub fn jne(comp: &str) -> String {
    concat_string!(comp, ";JNE\n")
}

/// Returns:
/// ```no_test
///  "@{dest}\n{JUMP_UNCOND}\n"
/// ```
pub fn jump(dest: String, func_name: &str) -> String {
    concat_string!("@", func_name, "$", dest, "\n", JUMP_UNCOND)
}

lazy_static! {

    /// Initializes the stack pointer and calls Sys.init, takes 53 instructions
    ///
    ///
    pub static ref BOOTSTRAP: String = concat_string!(
        "@256\nD=A\n@SP\nM=D\n",
        func_call(&"Sys.init".to_owned(), &"Sys.init$ret0".to_owned(), "0")
        // INFINITE_LOOP
    );

    /// Consumes the top 2 values of the stack, bitwise ANDs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref ADD: String = concat_string!(
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D+M\n"
    );

    /// Subracts the top value of the stack from the second-to-top value of the stack and stores the result on the new top
    /// of the stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref SUB: String = concat_string!(
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=M-D\n"
    );

    /// Consumes the top 2 values of the stack, bitwise ANDs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref AND: String = concat_string!(
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D&M\n"
    );

    /// Consumes the top 2 values of the stack, bitwise ORs them together, and stores the result on the new top of the
    /// stack.
    ///
    /// Registers upon exit: SP=SP-1, A=SP, D=RAM\[SP]
    pub static ref OR: String = concat_string!(
        pop(Segment::Stack, None),
        SET_A_STACK_TOP,
        "M=D|M\n"
    );

    pub static ref NOT: String = concat_string!(SET_A_STACK_TOP, "M=!M\n");

    pub static ref NEG: String = concat_string!(SET_A_STACK_TOP, "M=-M\n");

}

/// sets A to `ind` offset of `loc` pointer's base address
pub fn set_a_offset(loc: &Segment, ind: &str) -> String {
    match *loc {
        Segment::Temp => {
            // for the Temp segment (registers R5-R12)
            let off = u16_from_i16(ind.parse::<i16>().unwrap() + 5);
            concat_string!("@R", off.to_string(), "\n")
        }
        Segment::Pointer => {
            // pointer "segment" is just THIS and THAT registers
            if ind == "0" {
                load_const("THIS")
            } else {
                load_const("THAT")
            }
        }
        _ => concat_string!(
            load_const(loc.to_string().as_str()),
            load(Reg::D, "M"),
            load_const(ind),
            load(Reg::A, "D+A")
        ),
    }
}

/// Pushes a value to the stack.
///
/// If the location is a memory segment (e.g. "Local"), `val` is treated as an index into
/// the memory segment, and the value at the resultant memory location is pushed onto the stack.
///
/// If `val` is None and the location is a memory segment, the value of the pointer itself is stored to the stack.
pub fn push(loc: Segment, val: Option<&str>) -> String {
    use Segment::*;
    match loc {
        Stack => {
            if let Some(i) = val {
                concat_string!(load_const(i), load(Reg::D, "A"), PUSH_D_STACK)
            } else {
                panic!("Got instruction to push constant with no value");
            }
        }
        Temp => {
            let index: u16 = val.unwrap().parse().unwrap();
            concat_string! {
                load_const(concat_string!("R", (5 + index).to_string())),
                load(Reg::D, "M"),
                PUSH_D_STACK
            }
        }
        Pointer => {
            let index: u16 = val.unwrap().parse().unwrap();
            match index {
                0 => concat_string! {
                    load_const("THIS"),
                    load(Reg::D, "M"),
                    PUSH_D_STACK
                },
                1 => concat_string! {
                    load_const("THAT"),
                    load(Reg::D, "M"),
                    PUSH_D_STACK
                },
                _ => panic!("Invalid pointer index: {index}. Expected 0 or 1"),
            }
        }
        _ => {
            if let Some(i) = val {
                concat_string!(set_a_offset(&loc, i), load(Reg::D, "M"), PUSH_D_STACK)
            } else {
                concat_string!(
                    load_const(loc.to_string().as_str()),
                    load(Reg::D, "A"),
                    PUSH_D_STACK
                )
            }
        }
    }
}

/// Pops a value off of the stack and stores it in D if val = None, otherwise stores it in RAM[loc+val]
pub fn pop(loc: Segment, val: Option<&str>) -> String {
    match loc {
        Segment::Stack => POP_STACK.to_owned(),
        Segment::Temp => {
            let index: u16 = val.unwrap().parse().unwrap();
            concat_string! {
                POP_STACK,
                load_const(concat_string!("R", (5 + index).to_string())),
                load(Reg::M, "D")
            }
        }
        Segment::Pointer => {
            let index: u16 = val.unwrap().parse().unwrap();
            match index {
                0 => concat_string!(POP_STACK, load_const("THIS"), load(Reg::M, "D")),
                1 => concat_string!(POP_STACK, load_const("THAT"), load(Reg::M, "D")),
                _ => panic!("Invalid pointer index: {index}. Expected 0 or 1"),
            }
        }
        _ => {
            let ind = val.unwrap();
            concat_string!(
                set_a_offset(&loc, ind), // e.g. set A local[0]'s address
                load(Reg::D, "A"),       // set D to local[0]'s address
                load_const("R13"),       // store local[0]'s address in R13
                load(Reg::M, "D"),
                POP_STACK, // pop stack into D
                load_const("R13"),
                DEREF_A,           // Access local[0]'s address from R13
                load(Reg::M, "D")  // set RAM[local[0]] to popped value
            )
        }
    }
}

// comparisons
pub fn eq(eq_count: usize, func_name: &str) -> String {
    let lab = concat_string!(func_name, "EQ_", eq_count.to_string());
    concat_string!(
        POP_STACK,
        load(Reg::A, "A-1"),
        load(Reg::D, "M-D"),
        load(Reg::M, "-1"),
        load_const(&lab),
        jeq("D"),
        SET_A_STACK_TOP,
        load(Reg::M, "0"),
        label(&lab)
    )
}

pub fn lt(lt_count: usize, func_name: &str) -> String {
    let lab = concat_string!(func_name, "LT_", lt_count.to_string());
    concat_string!(
        POP_STACK,
        load(Reg::A, "A-1"),
        load(Reg::D, "M-D"),
        load(Reg::M, "-1"),
        load_const(&lab),
        jlt("D"),
        SET_A_STACK_TOP,
        load(Reg::M, "0"),
        label(&lab)
    )
}

/// compares the top 2 values on the stack, pushes -1 if
pub fn gt(gt_count: usize, func_name: &str) -> String {
    let lab = concat_string!(func_name, "GT_", gt_count.to_string());
    concat_string!(
        POP_STACK,
        load(Reg::A, "A-1"),
        load(Reg::D, "M-D"),
        load(Reg::M, "-1"),
        load_const(&lab),
        jgt("D"),
        SET_A_STACK_TOP,
        load(Reg::M, "0"),
        label(&lab)
    )
}

pub fn jump_if_zero(dest: String, func_name: &str) -> String {
    concat_string!(
        POP_STACK,
        load_const(concat_string!(func_name, "$", dest)),
        jne("D")
    )
}

pub fn func_return() -> String {
    concat_string!(
        // store frame memory loc in R[15]
        load_const("LCL"),
        load(Reg::D, "M"),
        load_const("R15"),
        load(Reg::M, "D"),
        // store return address (RAM[frame-5]) in R14
        // D=RAM[R13] due to prev instruction
        load_const(5),
        load(Reg::A, "D-A"),
        load(Reg::D, "M"),
        load_const("R14"),
        load(Reg::M, "D"),
        // pop stack to RAM[arg.0]
        pop(Segment::Argument, Some("0")),
        // set stack to RAM[arg + 1]
        load_const("ARG"),
        load(Reg::D, "M+1"),
        load_const("SP"),
        load(Reg::M, "D"),
        // Restore THAT
        load_const("R15"),
        load(Reg::D, "M"),
        load_const(1),
        load(Reg::A, "D-A"),
        load(Reg::D, "M"),
        load_const("THAT"),
        load(Reg::M, "D"),
        // Restore THIS
        load_const("R15"),
        load(Reg::D, "M"),
        load_const(2),
        load(Reg::A, "D-A"),
        load(Reg::D, "M"),
        load_const("THIS"),
        load(Reg::M, "D"),
        // Restore ARG
        load_const("R15"),
        load(Reg::D, "M"),
        load_const(3),
        load(Reg::A, "D-A"),
        load(Reg::D, "M"),
        load_const("ARG"),
        load(Reg::M, "D"),
        // Restore LCL
        load_const("R15"),
        load(Reg::D, "M"),
        load_const(4),
        load(Reg::A, "D-A"),
        load(Reg::D, "M"),
        load_const("LCL"),
        load(Reg::M, "D"),
        // jump to return addr
        load_const("R14"),
        DEREF_A,
        JUMP_UNCOND
    )
}

pub fn func_call(func_label: &String, return_addr: &String, n_args: &str) -> String {
    concat_string!(
        push(Segment::Stack, Some(return_addr)),
        load_const("LCL"),
        load(Reg::D, "M"),
        PUSH_D_STACK,
        load_const("ARG"),
        load(Reg::D, "M"),
        PUSH_D_STACK,
        load_const("THIS"),
        load(Reg::D, "M"),
        PUSH_D_STACK,
        load_const("THAT"),
        load(Reg::D, "M"),
        PUSH_D_STACK,
        // Set ARG to SP-5-n_args
        load_const("SP",),
        load(Reg::D, "M"),
        load_const(5),
        load(Reg::D, "D-A"), // D = SP - 5
        load_const(n_args),
        load(Reg::D, "D-A"), // D = (SP - 5) - n_args
        load_const("ARG"),
        load(Reg::M, "D"),
        // Set LCL to SP
        load_const("SP"),
        load(Reg::D, "M"),
        load_const("LCL"),
        load(Reg::M, "D"),
        load_const(func_label),
        "0;JMP\n",
        label(return_addr)
    )
}
