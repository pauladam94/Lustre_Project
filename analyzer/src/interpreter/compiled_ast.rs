use crate::interpreter::{compiled_expr::CompiledExpr, expr_index::ExprIndex};

#[derive(Debug, Clone)]
pub struct CompiledNode {
    vec: Vec<CompiledExpr>,
    info: Vec<String>,
}

impl std::fmt::Display for CompiledNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "loop {{")?;
        for (i, expr) in self.vec.iter().enumerate() {
            write!(f, "\t{i} - {expr} // {}", self.info[i])?;
            if i != self.vec.len() - 1 {
                writeln!(f)?;
            }
        }
        write!(f, "\n}}")
    }
}

impl Default for CompiledNode {
    fn default() -> Self {
        Self::new()
    }
}

impl CompiledNode {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            info: vec![],
        }
    }
    pub fn replace_expr(&mut self, expr: CompiledExpr, index: ExprIndex) {
        self.vec[index.to_usize()] = expr;
    }

    pub fn add_expr(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        self.vec.push(expr);
        self.info.push(info);
        ExprIndex::new(self.vec.len() - 1)
    }

    pub fn insert_info(&mut self, index: ExprIndex, info: String) {
        self.info[index.to_usize()] = format!("{} - {}", info, self.info[index.to_usize()]);
    }
}
