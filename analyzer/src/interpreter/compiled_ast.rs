use crate::{
    interpreter::{compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::literal::Value,
};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CompiledNode {
    vec: Vec<CompiledExpr>,
    info: VecDeque<String>,
    inputs: Vec<ExprIndex>,
    outputs: Vec<ExprIndex>,
    values: Vec<Option<Value>>,
    instant: u64,
}

impl std::fmt::Display for CompiledNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instant = {}", self.instant)?;
        writeln!(f, "loop {{")?;
        for ((i, expr), value) in self.vec.iter().enumerate().zip(self.values.iter()) {
            write!(
                f,
                "\t{i:<3} -   {:<10} >> {} // {:10}",
                format!("{expr}"),
                match value {
                    Some(v) => &format!("{v}"),
                    None => "None",
                },
                self.info[i]
            )?;
            if i != self.vec.len() - 1 {
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
            vec: Vec::new(),
            info: VecDeque::new(),
            inputs: vec![],
            outputs: vec![],
            values: vec![],
            instant: 0,
        }
    }
    pub fn offset_index(&mut self, offset: usize) {
        for expr in self.vec.iter_mut() {
            expr.offset_index(offset);
        }
        for expr in self.inputs.iter_mut() {
            expr.offset_index(offset);
        }
        for expr in self.outputs.iter_mut() {
            expr.offset_index(offset);
        }
    }
    pub fn insert_expr(&mut self, index: usize, expr: CompiledExpr) {
        self.vec.insert(index, expr);
        self.values.push(None);
    }
    pub fn insert_info(&mut self, index: usize, info: String) {
        self.info.insert(index, info);
        self.values.push(None);
    }
    pub fn set_inputs(&mut self, inputs: Vec<ExprIndex>) {
        self.inputs = inputs;
    }
    pub fn set_outputs(&mut self, outputs: Vec<ExprIndex>) {
        self.outputs = outputs;
    }
    pub fn replace_expr(&mut self, expr: CompiledExpr, index: ExprIndex) {
        self.vec[index.to_usize()] = expr;
    }
    pub fn back_index(&self) -> ExprIndex {
        ExprIndex::new(self.vec.len())
    }
    pub fn push_back_expr(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        self.vec.push(expr);
        self.info.push_back(info);
        self.values.push(None);
        ExprIndex::new(self.vec.len() - 1)
    }

    pub fn add_info(&mut self, index: ExprIndex, info: String) {
        self.info[index.to_usize()] = format!("{} - {}", info, self.info[index.to_usize()]);
    }
}

impl CompiledNode {
    pub fn reset(&mut self) {
        self.instant = 0;
    }
    pub fn step(&mut self, inputs: Vec<Value>) -> Vec<Value> {
        // println!("{} >>\n{}\n", "COMPILE".blue(), self);
        let Self {
            vec,
            info,
            inputs: inputs_index,
            outputs: outputs_index,
            values,
            instant,
        } = self;
        for (index, val) in inputs_index.iter().zip(inputs.into_iter()) {
            values[index.to_usize()] = Some(val);
        }
        for (pos, expr) in vec.iter().enumerate().rev() {
            values[pos] = expr.compute(values, instant);
        }
        // println!(
        //     "{} >>\n{}\n",
        //     "COMPILE".blue(),
        //     Self {
        //         vec: vec.clone(),
        //         info: info.clone(),
        //         inputs: inputs_index.clone(),
        //         outputs: outputs_index.clone(),
        //         values: values.clone(),
        //         instant: instant.clone(),
        //     }
        // );
        let mut res = vec![];
        for output in outputs_index.iter() {
            res.push(values[output.to_usize()].clone().unwrap());
        }

        self.instant += 1;
        return res;
    }
}
