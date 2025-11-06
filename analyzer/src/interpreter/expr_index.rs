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
    pub fn offset_index(&mut self, offset: usize) {
        *self = Self(self.0 + offset as u32);
    }
}
