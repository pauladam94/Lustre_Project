use crate::{
    interpreter::{compiled_expr::CompiledExpr, expr_index::ExprIndex},
    parser::literal::Value,
};
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct CompiledNode {
    exprs: Vec<CompiledExpr>,
    infos: Vec<String>,
    inputs: Vec<ExprIndex>,
    outputs: Vec<ExprIndex>,
    values: Vec<Option<Value>>,
    instant: u64,
}

impl std::fmt::Display for CompiledNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instant = {}", self.instant)?;
        writeln!(f, "loop {{")?;
        for ((i, expr), value) in self.exprs.iter().enumerate().zip(self.values.iter()) {
            write!(
                f,
                "\t{i:<3} -   {:<10} >> {} // {:10}",
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
            instant: 0,
        }
    }
    pub fn move_into(
        &self,
        exprs: &mut Vec<CompiledExpr>,
        infos: &mut Vec<String>,
        index: usize,
    ) -> ExprIndex {
        CompiledNode::push_back_expr_core(
            exprs,
            infos,
            self.exprs[index].clone(),
            self.infos[index].clone(),
        )
    }
    pub fn schedule(&self) -> Self {
        use CompiledExpr::*;
        let number_expression = self.len();

        let mut marked = vec![true; number_expression];

        // marked the node with the link to them
        // the node with marked[node] == true
        // are the top level node
        for expr in self.exprs.iter() {
            match expr {
                Get { src: index } | UnaryOp { rhs: index, .. } | Variable(index) => {
                    marked[*index] = false;
                }
                BinOp { lhs, op: _, rhs } => {
                    marked[*lhs] = false;
                    marked[*rhs] = false;
                }
                Array(items) | Tuple(items) => {
                    items.iter().for_each(|index| marked[*index] = true);
                }
                _ => {}
            }
        }

        // done store the index in self.exprs that have been treated
        let mut new_index: Vec<Option<ExprIndex>> = vec![None; self.len()];
        let mut exprs = vec![];
        let mut infos = vec![];

        // mut infos;
        eprintln!("marked = ");
        marked.iter().enumerate().for_each(|(index, b)| {
            if *b {
                eprintln!("\t{index} is marked");
                new_index[index] = Some(self.move_into(&mut exprs, &mut infos, index));
            }
        });
        let mut pos = 0;

        eprintln!("Debug {}", "ALGO".purple());
        while pos < exprs.len() {
            eprintln!(">> Algo at iteration {pos}");
            exprs.iter().enumerate().for_each(|(i, e)| {
                eprintln!(
                    "\t{}{i}   -   {e}",
                    if i == pos { " --> " } else { "     " }
                );
            });
            match exprs[pos].clone() {
                Set { src: index }
                | Get { src: index }
                | UnaryOp { rhs: index, .. }
                | Variable(index) => {
                    if new_index[index].is_none() {
                        new_index[index] = Some(self.move_into(&mut exprs, &mut infos, index));
                    }
                }
                BinOp { lhs, op: _, rhs } => {
                    if new_index[lhs].is_none() {
                        new_index[lhs] = Some(self.move_into(&mut exprs, &mut infos, lhs));
                    }
                    if new_index[rhs].is_none() {
                        new_index[rhs] = Some(self.move_into(&mut exprs, &mut infos, rhs));
                    }
                }
                Array(items) | Tuple(items) => items.into_iter().for_each(|e| {
                    if new_index[e].is_none() {
                        new_index[e] = Some(self.move_into(&mut exprs, &mut infos, e));
                    }
                }),
                _ => {}
            }
            match &mut exprs[pos] {
                Set { src: index }
                | Get { src: index }
                | UnaryOp { rhs: index, .. }
                | Variable(index) => {
                    *index = new_index[*index].unwrap();
                }
                BinOp { lhs, op: _, rhs } => {
                    *lhs = new_index[*lhs].unwrap();
                    *rhs = new_index[*rhs].unwrap();
                }
                Array(items) | Tuple(items) => {
                    items.iter_mut().for_each(|i| *i = new_index[*i].unwrap());
                }
                _ => {}
            }
            pos += 1;
        }

        eprintln!("new_index = ");
        new_index.iter().enumerate().for_each(|(pos, index)| {
            eprintln!(
                "\t{pos} -> {}",
                match index {
                    Some(i) => format!("{i}"),
                    None => "None".to_string(),
                }
            );
        });

        let values = vec![None; exprs.len()];
        let outputs = self
            .outputs
            .iter()
            .map(|index| new_index[*index].unwrap())
            .collect();
        let inputs = self
            .inputs
            .iter()
            .map(|index| new_index[*index].unwrap())
            .collect();

        CompiledNode {
            exprs,
            infos,
            inputs,
            outputs,
            values,
            instant: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.exprs.len()
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
    pub fn push_back_expr(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        CompiledNode::push_back_expr_core(&mut self.exprs, &mut self.infos, expr, info)
    }
    pub fn push_back_expr_memo(&mut self, expr: CompiledExpr, info: String) -> ExprIndex {
        // NO Memoisation for now
        if true {
            // Memoisation of compilation, if we push something already compiled this
            // we return the one already compiled
            if expr != CompiledExpr::Output
                && expr != CompiledExpr::Input // maybe not useful
                && let Some(i) = self.exprs.iter().position(|x| x == &expr)
            {
                // eprintln!("PUSH (already here at {i}) : {expr}");
                i
            } else {
                // eprintln!("PUSH (not already here) : {expr}");
                self.push_back_expr(expr, info)
            }
        } else {
            self.push_back_expr(expr, info)
        }
    }

    pub fn add_info(&mut self, index: ExprIndex, info: String) {
        self.infos[index] = format!("{} - {}", info, self.infos[index]);
    }

    pub fn step(&mut self, inputs: Vec<Value>) -> Vec<Value> {
        // println!("{} >>\n{}\n", "COMPILE".blue(), &self);
        let Self {
            exprs: vec,
            infos: info,
            inputs: inputs_index,
            outputs: outputs_index,
            values,
            instant,
        } = self;
        for (index, val) in inputs_index.iter().zip(inputs.into_iter()) {
            values[*index] = Some(val);
        }
        for (pos, expr) in vec.iter().enumerate().rev() {
            if expr == &CompiledExpr::Input {
                continue;
            }
            values[pos] = expr.compute_one_step(values, instant);
        }
        eprintln!(
            "{} >>\n{}\n",
            "COMPILE".blue(),
            Self {
                exprs: vec.clone(),
                infos: info.clone(),
                inputs: inputs_index.clone(),
                outputs: outputs_index.clone(),
                values: values.clone(),
                instant: *instant,
            }
        );
        let mut res = vec![];
        for output in outputs_index.iter() {
            res.push(values[*output].clone().unwrap());
        }

        self.instant += 1;
        res
    }
}
