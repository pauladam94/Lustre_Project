use crate::checker::infer_types::InferLen;
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
    Array { t: Box<InnerVarType>, len: InferLen },
}

impl InnerVarType {
    pub fn merge(self, rhs: Self) -> Option<Self> {
        use InnerVarType::*;
        match (self, rhs) {
            (lhs, rhs) if lhs == rhs => Some(rhs),
            (Tuple(l1), Tuple(l2)) => {
                let mut res = vec![];
                for (t1, t2) in l1.into_iter().zip(l2.into_iter()) {
                    res.push(t1.merge(t2)?)
                }
                Some(Tuple(res))
            }
            (Array { t: t1, len: len1 }, Array { t: t2, len: len2 }) if t1 == t2 => Some(Array {
                t: t1,
                len: len1.merge(len2)?,
            }),
            _ => None,
        }
    }
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
    pub fn merge_initialization(left: bool, right: bool) -> bool {
        left && right
    }
    pub fn merge(self, rhs: Self) -> Option<Self> {
        Some(Self {
            inner: self.inner.merge(rhs.inner)?,
            initialized: VarType::merge_initialization(self.initialized, rhs.initialized),
        })
    }

    /// Check equatity of inner type without looking
    /// if there are initialized the same way
    pub fn equal_without_pre(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
    }

    pub fn equal_array_of(&self, lhs: &Self) -> bool {
        match &self.inner {
            InnerVarType::Array { t, len: _ } => **t == lhs.inner,
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
            InnerVarType::Array { t, len } => write!(f, "[{t}]^{len}"),
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
