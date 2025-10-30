#[derive(Debug, Clone)]
pub struct ExprIndex(u32);

impl std::fmt::Display for ExprIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
