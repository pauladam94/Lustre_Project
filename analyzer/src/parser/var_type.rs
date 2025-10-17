use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

use crate::parser::span::LSpan;
use crate::parser::white_space::ws;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VarType {
    Str,
    Int,
    Float,
    Char,
    String,
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarType::Str => write!(f, "str"),
            VarType::Int => write!(f, "int"),
            VarType::Float => write!(f, "float"),
            VarType::Char => write!(f, "char"),
            VarType::String => write!(f, "string"),
        }
    }
}

pub(crate) fn var_type(input: LSpan) -> IResult<LSpan, VarType> {
    ws(alt((
        value(VarType::Int, tag("int")),
        value(VarType::Str, tag("str")),
        value(VarType::Float, alt((tag("float"), tag("real")))),
        value(VarType::Char, tag("char")),
        value(VarType::String, tag("string")),
    )))
    .parse(input)
}
