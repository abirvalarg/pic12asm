use std::fmt;
use std::error::Error;

#[macro_export]
macro_rules! str_err {
    ( $name:ident, $msg:expr ) => {
        #[derive(Debug)]
        struct $name;
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, $msg)
            }
        }
        impl std::error::Error for $name {}
    };
}

#[derive(Debug)]
pub struct SyntaxError {
    pub line: usize
}
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error at line {}", self.line)
    }
}
impl Error for SyntaxError {}

#[derive(Debug)]
pub struct UndefinedLabel(pub String);
impl fmt::Display for UndefinedLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label `{}` is not defined", self.0)
    }
}
impl Error for UndefinedLabel {}

#[derive(Debug)]
pub struct InvalidInstruction(pub String, pub usize);
impl fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid instruction `{}` on line {}", self.0, self.1)
    }
}
impl Error for InvalidInstruction {}

#[derive(Debug)]
pub struct BadOpArgs(pub usize);
impl fmt::Display for BadOpArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong amount of operands on line {}", self.0)
    }
}
impl Error for BadOpArgs {}

#[derive(Debug)]
pub struct DoesnotFit(pub usize);
impl fmt::Display for DoesnotFit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instruction on line {} doesn't fit in memory bank", self.0)
    }
}
impl Error for DoesnotFit {}
