use crate::software::{compiler::JackCompiler, compiler_utils::*};
use concat_string::concat_string;
use std::io::Write;
use strum_macros::{EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, EnumString, IntoStaticStr, Eq, PartialEq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    Constant,
    Argument,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl From<Keyword> for Segment {
    fn from(value: Keyword) -> Self {
        match value {
            Keyword::Var => Segment::Local,
            Keyword::Static => Segment::Static,
            Keyword::Arg => Segment::Argument,
            Keyword::Field => Segment::This,
            _ => panic!("invalid segment identifier: {:?}", value),
        }
    }
}

impl JackCompiler {
    pub fn push_name(&mut self, name: &str) {
        match name {
            "false" | "null" => writeln!(self.output, "push constant 0").unwrap(),
            "true" => writeln!(self.output, "push constant 0\nnot").unwrap(),
            "this" => writeln!(self.output, "push pointer 0").unwrap(),
            _ => {
                let var = self.symbol_table.get(name).unwrap_or_else(|| {
                    panic!("Variable name '{name}' does not exist in the symbol table")
                });
                writeln!(
                    self.output,
                    "{}",
                    concat_string! {
                    "push ",
                    Into::<&str>::into(var.segment),
                    " ",
                    var.index.to_string()}
                )
                .unwrap()
            }
        }

        self.output.flush().unwrap();
    }

    pub fn push_seg(&mut self, segment: Segment, index: usize) {
        writeln!(
            self.output,
            "{}",
            concat_string! {
            "push ",
            Into::<&str>::into(segment),
            " ",
            index.to_string()}
        )
        .unwrap();

        self.output.flush().unwrap();
    }

    pub fn pop_seg(&mut self, segment: Segment, index: usize) {
        let out_string = concat_string! {
            "pop ",
            Into::<&str>::into(segment),
            " ",
            index.to_string()
        };

        writeln!(self.output, "{out_string}").unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_function(&mut self, name: &str, arg_count: usize) {
        let out_string = concat_string! {
            "function ",
            self.class_name,
            ".",
            name,
            " ",
            arg_count.to_string()
        };
        writeln!(self.output, "{out_string}").unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_function_call(&mut self, obj: &str, func_name: &str, arg_count: usize) {
        let out_string = concat_string! {
            "call ",
            obj,
            ".",
            func_name,
            " ",
            arg_count.to_string()
        };

        writeln!(self.output, "{out_string}").unwrap();
        self.output.flush().unwrap()
    }

    pub fn write_return(&mut self) {
        writeln!(self.output, "return").unwrap();
    }

    pub fn write_operators(&mut self, ops: &[Token]) {
        use Symbol::*;
        for op in ops.iter().rev() {
            match *op {
                Token::Symbol(Asterisk) => self.write_function_call("Math", "multiply", 2),
                Token::Symbol(FwdSlash) => self.write_function_call("Math", "divide", 2),
                Token::Symbol(Plus) => writeln!(self.output, "add").unwrap(),
                Token::Symbol(Minus) => writeln!(self.output, "sub").unwrap(),
                Token::Symbol(GreaterThan) => writeln!(self.output, "gt").unwrap(),
                Token::Symbol(LessThan) => writeln!(self.output, "lt").unwrap(),
                Token::Symbol(Equals) => writeln!(self.output, "eq").unwrap(),
                Token::Symbol(And) => writeln!(self.output, "and").unwrap(),
                Token::Symbol(Pipe) => writeln!(self.output, "or").unwrap(),
                _ => panic!("invalid operator: {op:?}"),
            }
        }
        self.output.flush().unwrap();
    }

    pub fn write_negate(&mut self) {
        writeln!(self.output, "neg").unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_not(&mut self) {
        writeln!(self.output, "not").unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_if(&mut self, label_number: usize) {
        writeln!(self.output, "if-goto L{}", label_number).unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_else(&mut self, label_number: usize) {
        writeln!(self.output, "goto L{}", label_number).unwrap();
        self.output.flush().unwrap();
    }

    pub fn write_label(&mut self, label_number: usize) {
        writeln!(self.output, "label L{label_number}").unwrap();
        self.output.flush().unwrap();
    }
}
