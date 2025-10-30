use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VarType {
    Unit,
    Pre(Box<VarType>),
    Int,
    Float,
    Bool,
    Char,
    String,
    Tuple(Vec<VarType>),
    Array(Box<VarType>),
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarType::Unit => write!(f, "()"),
            VarType::Int => write!(f, "int"),
            VarType::Float => write!(f, "float"),
            VarType::Bool => write!(f, "bool"),
            VarType::Char => write!(f, "char"),
            VarType::String => write!(f, "string"),
            VarType::Pre(var_type) => write!(f, "pre {}", var_type),
            VarType::Array(var_type) => write!(f, "[{}]", var_type),
            VarType::Tuple(v) => {
                for (i, typ) in v.iter().enumerate() {
                    write!(f, "{typ}")?;
                    if i != v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                Ok(())
            }
        }
    }
}

pub(crate) fn var_type(input: LSpan) -> IResult<LSpan, VarType> {
    ws(alt((
        value(VarType::Int, tag("int")),
        value(VarType::Float, alt((tag("float"), tag("real")))),
        value(VarType::Char, tag("char")),
        value(VarType::Bool, tag("bool")),
        value(VarType::String, tag("string")),
    )))
    .parse(input)
}
