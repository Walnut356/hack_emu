use crate::software::vm_instructions::*;
use crate::utils::get_file_buffers;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum_macros::EnumString;

// TODO use box str instead of String?

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
pub enum Segment {
    #[strum(to_string = "SP")]
    #[strum(serialize = "constant")]
    Stack,
    #[strum(to_string = "LCL")]
    #[strum(serialize = "local")]
    Local,
    #[strum(to_string = "ARG")]
    #[strum(serialize = "argument")]
    Argument,
    #[strum(to_string = "THIS")]
    #[strum(serialize = "this")]
    This,
    #[strum(to_string = "THAT")]
    #[strum(serialize = "that")]
    That,
    #[strum(to_string = "R")]
    #[strum(serialize = "temp")]
    Temp,
    #[strum(serialize = "pointer")]
    Pointer,
    #[strum(default)]
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Instruction {
    Pop,
    Push,
    Add,
    Sub,
    Eq,
    Lt,
    Gt,
    Neg,
    Not,
    And,
    Or,

    Label,
    Goto,
    #[strum(serialize = "if-goto")]
    IfGoto,
    Function,
    Call,
    Return,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LabelCount {
    eq: usize,
    lt: usize,
    gt: usize,
    ret: HashMap<String, usize>,
}

/// Accepts a Path to a `.vm` file or folder containing multiple `.vm` files, translates the instructions to Hack
/// assembly file (`.asm`) in the same directory and returns a Path to it
pub fn vm_to_asm(path: &Path) -> PathBuf {
    let mut out_path;

    if path.is_file() {
        out_path = Path::new(path.parent().unwrap()).join(path.file_stem().unwrap());
    } else {
        out_path = Path::new(path).join(path.file_stem().unwrap());
    }

    let files = get_file_buffers(path, "vm");

    // Init output .asm file
    out_path.set_extension("asm");
    let out_file = File::create(out_path.clone()).unwrap();
    let mut output = BufWriter::new(out_file);
    write!(output, "{}", BOOTSTRAP.as_str()).unwrap();

    // helper variables for unique labels
    let mut counts = LabelCount::default();

    for (file, _f_name) in files {
        let mut lines = file.lines();

        while let Some(Ok(line)) = lines.next() {
            // record vm instruction as comment for debug purposes
            write!(output, "// {line}\n").unwrap();

            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            write!(output, "{}", parse_line(line, &mut counts)).unwrap();
        }
    }

    output.flush().unwrap();

    out_path
}

/// Parses an individual line of Hack VM code to Hack Assembly code. The resultant String is pushed to the end of the
/// supplied `output` String
pub fn parse_line(line: String, counts: &mut LabelCount) -> Box<str> {
    use Instruction::*;
    let mut temp = line.split_whitespace();
    let instr = Instruction::from_str(temp.next().unwrap())
        .expect(format!("Invalid instruction: {}", line).as_str());

    match instr {
        Pop => {
            // 2 tokens: pointer and offset
            let target =
                Segment::from_str(temp.next().expect("Pop instruction with no location")).unwrap(); // the default should mean this never fails
            let val = temp.next();

            pop(target, val).into()
        }
        Push => {
            // TODO do statics get a unique name?
            // 2 tokens: pointer and offset (or "constant" and value)
            let target =
                Segment::from_str(temp.next().expect("Push instruction with no location")).unwrap(); // the default should mean this never fails
            let val = temp.next();

            push(target, val).into()
        }
        Add => ADD.to_string().into(),
        Sub => SUB.to_string().into(),
        Eq => {
            counts.eq += 1;
            eq(counts.eq).into()
        }
        Lt => {
            counts.lt += 1;
            lt(counts.lt).into()
        }
        Gt => {
            counts.gt += 1;
            gt(counts.gt).into()
        }
        Neg => NEG.to_string().into(),
        Not => NOT.to_string().into(),
        And => AND.to_string().into(),
        Or => OR.to_string().into(),
        // Flow control
        Label => {
            // label + file name
            let l_name = temp.next().expect("Label instruction with no label name");
            assert!(
                !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                "Labels must not start with a digit. Got: {}",
                l_name
            );
            format!("({l_name})\n").into()
        }
        Goto => {
            let l_name = temp.next().expect("Jump instruction with no label");
            assert!(
                !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                "Labels must not start with a digit. Got: {}",
                l_name
            );

            jump(format!("{l_name}")).into()
        } // label + file name
        IfGoto => {
            // label + file name
            let l_name = temp.next().expect("Jump instruction with no label name");
            assert!(
                !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                "Labels must not start with a digit. Got: {}",
                l_name
            );
            jump_if_zero(format!("{l_name}")).into()
        }
        Function => {
            // function name + nVars
            let l_name = temp.next().expect("Function definition with no name");
            assert!(
                !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                "Function name must not start with a digit. Got: {}",
                l_name
            );
            let mut result = label(l_name);
            let n_vars: usize = temp
                .next()
                .expect("Function definition without nVars")
                .parse()
                .unwrap();

            for _ in 0..n_vars {
                result.push_str(&push(Segment::Stack, Some("0")))
            }
            result.into()
        }
        Call => {
            // function name + nArgs
            let l_name = temp.next().expect("Function call with no name");
            assert!(
                !l_name.chars().nth(0).unwrap().is_ascii_digit(),
                "Function name must not start with a digit. Got: {}",
                l_name
            );
            let func_name = format!("{l_name}");

            let n_args = temp.next().expect("Function Call with no Arg count");

            let c = counts.ret.entry(func_name.clone()).or_default();
            let return_addr = format!("{func_name}$ret{c}");
            let result = func_call(&func_name, &return_addr, n_args);
            *c += 1;

            result.into()
        }
        Return => func_return().into(), // 0 tokens
    }
}
