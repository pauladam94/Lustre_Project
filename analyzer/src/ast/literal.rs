use crate::ast::to_range::ToRange;
use crate::checker::infer_types::InferLen;
use crate::parser::span::ZERORANGE;
use crate::parser::var_type::InnerVarType;
use crate::parser::var_type::VarType;
use lsp_types::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
}

impl Value {
    pub fn tuple_from_vec(v: Vec<Value>) -> Self {
        if v.len() == 1 {
            v[0].clone()
        } else {
            Value::Tuple(v)
        }
    }
    pub fn unwrap_array(inputs: Vec<Value>) -> Option<Vec<Vec<Value>>> {
        let mut res = vec![];
        for input in inputs.into_iter() {
            match input {
                Value::Array(values) => res.push(values),
                _ => return None,
            }
        }
        Some(res)
    }

    pub fn get_inner_type(&self) -> InnerVarType {
        match self {
            Value::Unit => InnerVarType::Unit,
            Value::Int(_) => InnerVarType::Int,
            Value::Float(_) => InnerVarType::Float,
            Value::Bool(_) => InnerVarType::Bool,
            Value::Tuple(v) => {
                if v.is_empty() {
                    InnerVarType::Unit
                } else {
                    InnerVarType::Tuple(v.iter().map(|v| v.get_type().inner).collect())
                }
            }
            Value::Array(v) => {
                let len = InferLen::Known(v.len());
                // Check more things maybe
                if v.is_empty() {
                    InnerVarType::Array {
                        t: Box::new(InnerVarType::Unit),
                        len,
                    }
                } else {
                    InnerVarType::Array {
                        t: Box::new(v[0].get_type().inner),
                        len,
                    }
                }
            }
        }
    }
    pub fn get_type(&self) -> VarType {
        VarType {
            initialized: true,
            inner: self.get_inner_type(),
        }
    }
}

impl ToRange for Value {
    fn to_range(&self) -> Range {
        ZERORANGE // todo better
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(fl) => write!(f, "{fl}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Tuple(vec) => {
                if vec.is_empty() {
                    write!(f, "{}", InnerVarType::Unit)
                } else {
                    write!(f, "(")?;
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{val}")?;
                        if i != vec.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, ")")
                }
            }
            Value::Array(vec) => {
                if vec.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(f, "[")?;
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{val}")?;
                        if i != vec.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    }
}
