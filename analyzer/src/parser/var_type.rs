use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum InnerVarType {
//     Unit,
//     Int,
//     Float,
//     Bool,
//     Char,
//     String,
//     // Maybe this should be modified
//     Tuple(Vec<InnerVarType>),
//     Array(Box<InnerVarType>),
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct VarType {
//     inner: InnerVarType,
//     initialized: bool,
// }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarType {
    Unit,
    Int,
    Float,
    Bool,
    Char,
    String,
    // Maybe this should be modified
    Pre(Box<VarType>),
    Tuple(Vec<VarType>),
    Array(Box<VarType>),
}

impl VarType {
    pub fn contains_pre(&self) -> bool {
        match self {
            VarType::Unit
            | VarType::Int
            | VarType::Float
            | VarType::Bool
            | VarType::Char
            | VarType::String => false,
            VarType::Pre(_) => true,
            VarType::Tuple(var_types) => var_types
                .iter()
                .fold(false, |acc, t| acc || t.contains_pre()),
            VarType::Array(var_type) => var_type.contains_pre(),
        }
    }
    pub fn merge(self, rhs: Self) -> Option<Self> {
        if self == rhs {
            return Some(rhs);
        }

        use VarType::*;
        match (self, rhs) {
            (Pre(l), Pre(r)) => Some(Pre(Box::new(l.merge(*r)?))),
            (Pre(l), r) => Some(Pre(Box::new(l.merge(r)?))),
            (l, Pre(r)) => Some(Pre(Box::new(l.merge(*r)?))),
            (Tuple(l), Tuple(r)) => {
                let mut vec_type = vec![];
                for (lt, rt) in l.into_iter().zip(r.into_iter()) {
                    vec_type.push(lt.merge(rt)?)
                }
                Some(Tuple(vec_type))
            }
            (Array(l), Array(r)) => Some(Array(Box::new(l.merge(*r)?))),
            (_, _) => None,
        }
    }
    pub fn equal_without_pre(&self, rhs: &Self) -> bool {
        if self == rhs {
            return true;
        }

        use VarType::*;
        match (self, rhs) {
            (Pre(l), r) => l.equal_without_pre(r),
            (l, Pre(r)) => l.equal_without_pre(r),
            (_, _) => false,
        }
    }
    pub fn remove_one_pre(self) -> Self {
        match self {
            VarType::Pre(var_type) => *var_type,
            VarType::Int
            | VarType::Unit
            | VarType::Float
            | VarType::Bool
            | VarType::Char
            | VarType::String => self,
            VarType::Tuple(var_types) => {
                VarType::Tuple(var_types.into_iter().map(|t| t.remove_one_pre()).collect())
            }
            VarType::Array(var_type) => VarType::Array(Box::new(var_type.remove_one_pre())),
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
                write!(f, "(")?;
                for (i, typ) in v.iter().enumerate() {
                    write!(f, "{typ}")?;
                    if i != v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
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
