use crate::interpreter::{
    compiled_ast::CompiledNode, compiled_expr::CompiledExpr, expr_index::ExprIndex,
    instant::Instant,
};
use colored::Colorize;

impl CompiledNode {
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

        // eprintln!("marked = ");
        marked.iter().enumerate().for_each(|(index, b)| {
            if *b {
                // eprintln!("\t{index} is marked");
                new_index[index] = Some(self.move_into(&mut exprs, &mut infos, index));
            }
        });
        let mut pos = 0;

        // eprintln!("Debug {}", "ALGO".purple());
        while pos < exprs.len() {
            // eprintln!(">> Algo at iteration {pos}");
            // exprs.iter().enumerate().for_each(|(i, e)| {
            //     eprintln!(
            //         "\t{}{i}   -   {e}",
            //         if i == pos { " --> " } else { "     " }
            //     );
            // });
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

        // eprintln!("new_index = ");
        // new_index.iter().enumerate().for_each(|(pos, index)| {
        //     eprintln!(
        //         "\t{pos} -> {}",
        //         match index {
        //             Some(i) => format!("{i}"),
        //             None => "None".to_string(),
        //         }
        //     );
        // });

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
            instant: Instant::INIT,
        }
    }
}
