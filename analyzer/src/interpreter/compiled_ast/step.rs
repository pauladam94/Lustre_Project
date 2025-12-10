use crate::{
    interpreter::{compiled_ast::CompiledNode, compiled_expr::CompiledExpr},
    parser::literal::Value,
};
use colored::Colorize;

impl CompiledNode {
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
        // eprintln!(
        //     "{} >>\n{}\n",
        //     "COMPILE".blue(),
        //     Self {
        //         exprs: vec.clone(),
        //         infos: info.clone(),
        //         inputs: inputs_index.clone(),
        //         outputs: outputs_index.clone(),
        //         values: values.clone(),
        //         instant: *instant,
        //     }
        // );
        let mut res = vec![];
        for output in outputs_index.iter() {
            res.push(values[*output].clone().unwrap());
        }

        self.instant.step();
        res
    }
}



