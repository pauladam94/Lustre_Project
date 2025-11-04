#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExprIndex(u32);

impl std::fmt::Display for ExprIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ExprIndex {
    pub fn new(i: usize) -> Self {
        Self(i as u32)
    }
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}
