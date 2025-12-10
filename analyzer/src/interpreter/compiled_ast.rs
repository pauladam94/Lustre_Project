use crate::{
    interpreter::{compiled_expr::CompiledExpr, expr_index::ExprIndex, instant::Instant},
    parser::literal::Value,
};

pub mod schedule;
pub mod step;

#[derive(Debug, Clone)]
pub struct CompiledNode {
    exprs: Vec<CompiledExpr>,
    infos: Vec<String>,
    inputs: Vec<ExprIndex>,
    outputs: Vec<ExprIndex>,
    values: Vec<Option<Value>>,
    instant: Instant,
}

impl std::fmt::Display for CompiledNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instant = {}", self.instant)?;
        writeln!(f, "loop {{")?;
        let width = self
            .exprs
            .iter()
            .map(|e| format!("{e}").len())
            .max()
            .unwrap();
        for ((i, expr), value) in self.exprs.iter().enumerate().zip(self.values.iter()) {
            write!(
                f,
                "\t{i:<3} -   {:<width$} >> {} // {:10}",
                format!("{expr}"),
                match value {
                    Some(v) => &format!("{v}"),
                    None => "None",
                },
                self.infos[i]
            )?;
            if i != self.exprs.len() - 1 {
                writeln!(f)?;
            }
        }
        writeln!(f, "\n}}")?;

        write!(f, "input = [")?;
        for (i, input) in self.inputs.iter().enumerate() {
            write!(f, "{input}")?;
            if i != self.inputs.len() - 1 {
                write!(f, ", ")?;
            }
        }
        writeln!(f, "]")?;
        write!(f, "output = [")?;
        for (i, output) in self.outputs.iter().enumerate() {
            write!(f, "{output}")?;
            if i != self.outputs.len() - 1 {
                write!(f, ", ")?;
            }
        }
        writeln!(f, "]")
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
            exprs: Vec::new(),
            infos: Vec::new(),
            inputs: vec![],
            outputs: vec![],
            values: vec![],
            instant: Instant::INIT,
        }
    }

    pub fn len(&self) -> usize {
        self.exprs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert_expr(&mut self, index: usize, expr: CompiledExpr) {
        self.exprs.insert(index, expr);
        self.values.push(None);
    }
    pub fn insert_info(&mut self, index: usize, info: String) {
        self.infos.insert(index, info);
        self.values.push(None);
    }
    pub fn set_inputs(&mut self, inputs: Vec<ExprIndex>) {
        self.inputs = inputs;
    }
    pub fn set_outputs(&mut self, outputs: Vec<ExprIndex>) {
        self.outputs = outputs;
    }
    pub fn set_values_to_none(&mut self) {
        self.values = vec![None; self.len()];
    }
    pub fn replace_expr(&mut self, expr: CompiledExpr, index: ExprIndex) {
        self.exprs[index] = expr;
    }
    pub fn back_index(&self) -> ExprIndex {
        self.exprs.len()
    }
    pub fn push_back_expr_core(
        exprs: &mut Vec<CompiledExpr>,
        infos: &mut Vec<String>,
        expr: CompiledExpr,
        info: String,
    ) -> ExprIndex {
        exprs.push(expr);
        infos.push(info);
        exprs.len() - 1
    }
    pub fn push_expr_core(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        CompiledNode::push_back_expr_core(&mut self.exprs, &mut self.infos, expr, info)
    }

    /// Memoisation of compilation, if we push something already compiled this
    /// we return the one already compiled
    pub fn push_expr(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        if expr != CompiledExpr::Output
                && expr != CompiledExpr::Input // maybe not useful
                && let Some(i) = self.exprs.iter().position(|x| x == &expr)
        {
            i
        } else {
            self.push_expr_core(expr, info)
        }
    }

    pub fn add_info(&mut self, index: ExprIndex, info: String) {
        self.infos[index] = format!("{} - {}", info, self.infos[index]);
    }
}
