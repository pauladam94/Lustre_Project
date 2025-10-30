use crate::{interpreter::compiled_expr::CompiledExpr, parser::literal::Value};

#[derive(Debug, Clone)]
pub struct CompiledAst {
    pub vec: Vec<CompiledExpr>,
    pub info: Vec<String>,
}

impl std::fmt::Display for CompiledAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loop {{")?;
        for (i, expr) in self.vec.iter().enumerate() {
            write!(f, "\t{expr}")?;
            if i != self.vec.len() - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "}}")
    }
}

impl CompiledAst {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            info: vec![],
        }
    }

    pub fn add_expr(&mut self, expr: CompiledExpr, info: String) {
        self.vec.push(expr);
        self.info.push(info)
    }

    pub fn step(inputs: Vec<Value>) -> Vec<Value> {
        todo!()
    }
}
