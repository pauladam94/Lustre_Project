#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferLen {
    Unknown,
    Known(usize),
}

impl InferLen {
    pub fn merge(self, other: Self) -> Option<Self> {
        use InferLen::*;
        match (self, other) {
            (Unknown, Unknown) => Some(Unknown),
            (Unknown, Known(len)) | (Known(len), Unknown) => Some(Known(len)),
            (Known(t1), Known(t2)) if t1 == t2 => Some(Known(t1)),
            _ => None,
        }
    }
}

impl std::fmt::Display for InferLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferLen::Unknown => {
                write!(f, "Unknown Length")
            }
            InferLen::Known(len) => {
                write!(f, "{len}")
            }
        }
    }
}
