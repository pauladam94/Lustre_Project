use crate::{
    ast::to_range::ToRange,
    parser::{literal::Value, node::Node, span::Ident, var_type::VarType},
};
use indexmap::IndexMap;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub inputs: IndexMap<Ident, VarType>,
    pub outputs: IndexMap<Ident, VarType>,
    vars: HashMap<Ident, VarType>,
}

#[derive(Debug)]
pub enum FunctionCallType {
    Simple,
    Array,
}
impl FunctionType {
    pub fn function_call_type(self, args: &Vec<Value>) -> Option<FunctionCallType> {
        let mut res = None;
        for (arg, (_, input_type)) in args.iter().zip(self.inputs.iter()) {
            let arg_type = arg.get_type();
            match res {
                None => {
                    if &arg_type == input_type {
                        res = Some(FunctionCallType::Simple);
                    } else if arg_type.equal_array_of(input_type) {
                        res = Some(FunctionCallType::Array);
                    } else {
                        return None;
                    }
                }
                Some(FunctionCallType::Simple) => {
                    if &arg_type != input_type {
                        return None;
                    }
                }
                Some(FunctionCallType::Array) => {
                    if !arg_type.equal_array_of(input_type) {
                        return None;
                    }
                }
            }
        }
        res
    }
    pub(crate) fn get_function_type(node: &Node) -> (Self, Vec<Diagnostic>) {
        let mut diags = vec![];
        let mut func = FunctionType {
            inputs: IndexMap::new(),
            outputs: IndexMap::new(),
            vars: HashMap::new(),
        };
        for (name, t) in node.inputs.iter() {
            if func.inputs.contains_key(name) {
                diags.push(Diagnostic {
                    message: "Input name already used.".to_string(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                })
            } else {
                func.inputs.insert(name.clone(), t.clone());
            }
        }
        for (name, t) in node.outputs.iter() {
            if func.outputs.contains_key(name) {
                diags.push(Diagnostic {
                    message: "Output name already used.".to_string(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                })
            } else {
                func.outputs.insert(name.clone(), t.clone());
            }
        }
        for (name, t) in node.vars.iter() {
            if func.vars.contains_key(name) {
                diags.push(Diagnostic {
                    message: "Var name already used.".to_string(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                })
            } else {
                func.vars.insert(name.clone(), t.clone());
            }
        }
        (func, diags)
    }
}
