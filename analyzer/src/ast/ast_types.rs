use crate::{
    checker::function_type::FunctionType,
    parser::{span::Ident, var_type::VarType},
};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct AstTypes {
    node_names: Vec<Ident>,
    node_types: Vec<FunctionType>,
    local_types: Vec<HashMap<Ident, Option<VarType>>>,
}

impl AstTypes {
    pub fn new() -> Self {
        Self {
            node_names: Vec::new(),
            node_types: Vec::new(),
            local_types: Vec::new(),
        }
    }

    pub fn get_nodes_index(&self, name: &Ident) -> Option<usize> {
        self.node_names.iter().position(|s| s == name)
    }
    pub fn get_node_type(&self, name: &Ident) -> Option<&FunctionType> {
        if let Some(index) = self.get_nodes_index(name) {
            return Some(&self.node_types[index]);
        }
        None
    }
    pub fn insert_local_type(&mut self, name: &Ident, var: Ident, t: Option<VarType>) {
        if let Some(index) = self.get_nodes_index(name) {
            self.local_types[index].insert(var, t);
        }
    }
    pub fn get_type_var(&self, name: &Ident, var: &Ident) -> Option<Option<VarType>> {
        if let Some(index) = self.get_nodes_index(name) {
            match self.local_types[index].get(var) {
                Some(Some(t)) => Some(Some(t.clone())),
                Some(None) => Some(None),
                None => None,
            }
        } else {
            None
        }
    }
    pub fn contains_key_local_type(&self, name: &Ident, var: &Ident) -> bool {
        if let Some(index) = self.get_nodes_index(name) {
            if let Some(_) = self.local_types[index].get(var) {
                return true;
            }
        }
        false
    }
    pub fn node_defined(&self, name: &Ident) -> bool {
        self.get_nodes_index(name).is_some()
    }
    pub fn insert_node(&mut self, name: &Ident, func: FunctionType) {
        self.node_names.push(name.clone());
        self.node_types.push(func);
        self.local_types.push(HashMap::new());
    }
}
