use crate::parser::span::LSpan;
use crate::parser::white_space::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerVarType {
    Unit,
    Int,
    Float,
    Bool,
    Char,
    String,
    // Maybe this should be modified
    Tuple(Vec<InnerVarType>),
    Array(Box<InnerVarType>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VarType {
    pub inner: InnerVarType,
    pub initialized: bool,
}

impl VarType {
    pub fn uninitialized(&mut self) {
        self.initialized = false;
    }
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    pub fn is_not_initialized(&self) -> bool {
        !self.initialized
    }
    pub fn merge(self, mut rhs: Self) -> Option<Self> {
        if self == rhs {
            Some(rhs)
        } else if self.inner == rhs.inner {
            if self.initialized == rhs.initialized {
                Some(rhs)
            } else {
                rhs.uninitialized();
                Some(rhs)
            }
        } else {
            None
        }
    }
    pub fn equal_without_pre(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
    }
    pub fn equal_array_of(&self, lhs: &Self) -> bool {
        match &self.inner {
            InnerVarType::Array(inner_var_type) => **inner_var_type == lhs.inner,
            _ => false,
        }
    }
    pub fn remove_one_pre(self) -> Self {
        Self {
            inner: self.inner,
            initialized: true,
        }
    }
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.initialized {
            write!(f, "{}", self.inner)
        } else {
            write!(f, "pre {}", self.inner)
        }
    }
}

impl std::cmp::PartialEq<InnerVarType> for VarType {
    fn eq(&self, other: &InnerVarType) -> bool {
        &self.inner == other
    }
}
impl std::cmp::PartialEq<VarType> for InnerVarType {
    fn eq(&self, other: &VarType) -> bool {
        self == &other.inner
    }
}

impl std::fmt::Display for InnerVarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InnerVarType::Unit => write!(f, "()"),
            InnerVarType::Int => write!(f, "int"),
            InnerVarType::Float => write!(f, "float"),
            InnerVarType::Bool => write!(f, "bool"),
            InnerVarType::Char => write!(f, "char"),
            InnerVarType::String => write!(f, "string"),
            // InnerVarType::Pre(var_type) => write!(f, "pre {}", var_type),
            InnerVarType::Array(var_type) => write!(f, "[{}]", var_type),
            InnerVarType::Tuple(v) => {
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
            [] => Self {
                initialized: true,
                inner: InnerVarType::Unit,
            },
            [t] => t.clone(),
            _ => Self {
                initialized: true,
                inner: InnerVarType::Tuple(vec.into_iter().map(|x| x.inner).collect()),
            },
        }
    }
}
pub(crate) fn var_type(input: LSpan) -> IResult<LSpan, VarType> {
    ws(alt((
        value(
            VarType {
                initialized: true,
                inner: InnerVarType::Int,
            },
            tag("int"),
        ),
        value(
            VarType {
                initialized: true,
                inner: InnerVarType::Float,
            },
            alt((tag("float"), tag("real"))),
        ),
        value(
            VarType {
                initialized: true,
                inner: InnerVarType::Char,
            },
            tag("char"),
        ),
        value(
            VarType {
                initialized: true,
                inner: InnerVarType::Bool,
            },
            tag("bool"),
        ),
        value(
            VarType {
                initialized: true,
                inner: InnerVarType::String,
            },
            tag("string"),
        ),
    )))
    .parse(input)
}
