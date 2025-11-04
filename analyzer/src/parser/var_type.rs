use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

#[derive(Clone, Debug, Eq)]
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

impl PartialEq for VarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Pre(l0), Self::Pre(r0)) => l0 == r0,
            (Self::Tuple(l0), Self::Tuple(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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

impl VarType {
    pub fn tuple_from_vec(vec: Vec<VarType>) -> Self {
        match &vec[..] {
            [] => Self::Unit,
            [t] => t.clone(),
            _ => Self::Tuple(vec),
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
