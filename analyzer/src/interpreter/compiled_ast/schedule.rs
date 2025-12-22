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
    pub fn bfs(&self, start: usize, done: &mut [bool], pile: &mut Vec<usize>) {
        if done[start] {
            return;
        }
        done[start] = true;
        for neighbour in self.exprs[start].get_neighbours() {
            if let CompiledExpr::Pre { .. } = self.exprs[neighbour] {
                continue;
            }
            self.bfs(neighbour, done, pile);
        }

        pile.push(start);
    }
    pub fn schedule(&self) -> Self {
        eprintln!("{}", ">> SCHEDULE".blue());
        use CompiledExpr::*;
        let number_expression = self.len();

        let mut new_index: Vec<Option<ExprIndex>> = vec![None; self.len()];
        let mut exprs = vec![];
        let mut infos = vec![];

        let mut done = vec![false; number_expression];
        let mut pile: Vec<ExprIndex> = vec![];
        // We do a BFS on the CompiledNode
        for index in 0..self.exprs.len() {
            self.bfs(index, &mut done, &mut pile);
        }

        // Rearrange the expression according to the `pile` vector

        for (new, past) in pile.iter().enumerate() {
            new_index[*past] = Some(new);
        }

        eprintln!("> PILE : ");
        for (i, index) in pile.iter().enumerate() {
            eprintln!("\t{i} - {index}");
        }
        eprintln!("> New Index : ");
        for (i, index) in new_index.iter().enumerate() {
            if let Some(index) = index {
                eprintln!("\t{i} - {index}");
            }
        }

        for past in pile.iter() {
            self.move_into(&mut exprs, &mut infos, *past);
        }

        // Modify the
        for pos in 0..exprs.len() {
            match &mut exprs[pos] {
                Pre { src: index }
                | UnaryOp { rhs: index, .. }
                | Variable(index) => {
                    *index = new_index[*index].unwrap();
                }
                BinOp { lhs, op: _, rhs } => {
                    *lhs = new_index[*lhs].unwrap();
                    *rhs = new_index[*rhs].unwrap();
                }
                // Array(items) | Tuple(items) => {
                //     items.iter_mut().for_each(|i| *i = new_index[*i].unwrap());
                // }
                If { cond, yes, no } => {
                    *cond = new_index[*cond].unwrap();
                    *yes = new_index[*yes].unwrap();
                    *no = new_index[*no].unwrap();
                }
                Input | Output | Lit(_) => {}
            }
        }

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
