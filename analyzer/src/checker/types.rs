use crate::{
    diagnostic::ToRange,
    parser::{
        ast::Ast,
        expression::{BinOp, Expr, UnaryOp},
        literal::Literal,
        node::Node,
        span::{Ident, Span},
        var_type::VarType,
    },
};
use lsp_types::{Diagnostic, DiagnosticSeverity};
use std::collections::HashMap;

struct FunctionType {
    inputs: HashMap<Ident, VarType>,
    outputs: HashMap<Ident, VarType>,
    vars: HashMap<Ident, VarType>,
}

#[derive(Default)]
struct CheckerInfo {
    nodes_types: HashMap<Ident, FunctionType>,
    local_types: HashMap<Ident, Option<VarType>>,
    current_node: Ident,

    stop: bool,
    diagnostics: Vec<Diagnostic>,
}
impl CheckerInfo {
    fn set_current_node(&mut self, name: &Ident) {
        self.current_node = name.clone();
    }

    fn push_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }
}

impl CheckerInfo {
    fn get_type_equation(
        &mut self,
        node: &Node,
        expr: &Expr,
    ) -> Option<VarType> {
        match expr {
            Expr::BinOp {
                lhs,
                op:
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Div
                    | BinOp::Mult
                    | BinOp::Eq
                    | BinOp::Neq
                    | BinOp::Fby,
                rhs,
            } => {
                let lt = self.get_type_equation(node, lhs)?;
                let rt = self.get_type_equation(node, rhs)?;
                if lt == rt { Some(lt) } else { None }
            }
            Expr::BinOp {
                lhs,
                op: BinOp::Arrow,
                rhs,
            } => {
                let lt = self.get_type_equation(node, lhs)?;
                let rt = self.get_type_equation(node, rhs)?;
                todo!()
            }
            Expr::UnaryOp {
                op: UnaryOp::Inv | UnaryOp::Pre | UnaryOp::Not,
                rhs,
            } => {
                let t = self.get_type_equation(node, rhs)?;
                return Some(t);
            }
            Expr::Variable(s) => match self.local_types.get(s) {
                // type of variable found
                Some(Some(t)) => Some(t.clone()),
                // Type of Variable not found yet
                Some(None) => self.get_type_var(node, s),
                // Variable not defined
                None => {
                    self.push_diagnostic(Diagnostic {
                        message: format!("No Equation found for '{}'", s),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: s.to_range(),
                        ..Default::default()
                    });
                    None
                }
            },
            Expr::Lit(Literal::Bool(_)) => Some(VarType::Bool),
            Expr::Lit(Literal::Integer(_)) => Some(VarType::Int),
            Expr::Lit(Literal::Float(_)) => Some(VarType::Float),
            Expr::Array(arr) => {
                let mut t = None;
                for e in arr.iter() {
                    match &t {
                        None => {
                            t = self.get_type_equation(node, e);
                        }
                        Some(t2) => {
                            let t1 = self.get_type_equation(node, e)?;
                            if &t1 != t2 {
                                return None;
                            }
                        }
                    }
                }
                Some(VarType::Array(Box::new(t?)))
            }
            Expr::FCall { name, args } => {
                todo!()
            }
        }
    }

    fn get_type_var(&mut self, node: &Node, var: &Span) -> Option<VarType> {
        for (name, expr) in node.let_bindings.iter() {
            if name == var {
                return self.get_type_equation(node, expr);
            }
        }
        self.push_diagnostic(Diagnostic {
            message: format!(
                "No equation found for '{}' (inside get_type_var)",
                var
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            range: var.to_range(),
            ..Default::default()
        });
        None
    }

    /// Setup Local Type in the Checker
    /// - Check that variables are not defined twice in equations
    fn setup_local_types(&mut self, node: &Node) {
        // insert all inputs types
        for (name, t) in node.inputs.iter() {
            self.local_types.insert(name.clone(), Some(t.clone()));
        }

        for (name, expr) in node.let_bindings.iter() {
            if self.local_types.contains_key(name) {
                self.diagnostics.push(Diagnostic {
                    message: format!(
                        "Equation for '{}' already defined.",
                        name
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                });
            } else {
                self.local_types.insert(name.clone(), None);
            }
        }
        for (name, t) in node.outputs.iter() {
            self.local_types.insert(name.clone(), Some(t.clone()));
        }
    }

    fn check_node(&mut self, node: &Node) {
        self.set_current_node(&node.name);

        self.local_types.clear();
        self.setup_local_types(node);

        for (out, t) in node.outputs.iter() {
            match &self.get_type_var(node, out) {
                Some(t2) => {
                    if t != t2 {
                        self.push_diagnostic(Diagnostic {
                            message: format!(
                                "'{}' is supposed to be of type '{}', found '{}'.",
                                out, t, t2
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            range: out.to_range(),
                            ..Default::default()
                        });
                    }
                }
                None => {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Error while checking the type of '{}'.",
                            out,
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: out.to_range(),
                        ..Default::default()
                    });
                }
            }
        }

        // Call function that check type of a variable
        // error 1: no equation for the variable / not input
        // error 2: no the type expected
        // No variables are defined two times in equations

        // Functions cannot be called inside their own body

        // toutes les expressions ont le bon type
    }

    fn get_nodes_types(&mut self, x: &Ast) {
        for node in x.nodes.iter() {
            let mut func = FunctionType {
                inputs: HashMap::new(),
                outputs: HashMap::new(),
                vars: HashMap::new(),
            };
            for (name, t) in node.inputs.iter() {
                if func.inputs.contains_key(name) {
                    self.push_diagnostic(Diagnostic {
                        message: format!("Input name already used."),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: name.to_range(),
                        ..Default::default()
                    })
                } else {
                    func.inputs.insert(name.clone(), t.clone());
                }
            }
            for (name, t) in node.outputs.iter() {
                func.inputs.insert(name.clone(), t.clone());
                if func.outputs.contains_key(name) {
                    self.push_diagnostic(Diagnostic {
                        message: format!("Output name already used."),
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
                    self.push_diagnostic(Diagnostic {
                        message: format!("Output name already used."),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: name.to_range(),
                        ..Default::default()
                    })
                } else {
                    func.vars.insert(name.clone(), t.clone());
                }
            }
            self.nodes_types.insert(node.name.clone(), func);
        }
    }

    fn check_ast(&mut self, x: &Ast) {
        self.get_nodes_types(x);

        for node in x.nodes.iter() {
            self.check_node(node);
        }
    }
}

impl Ast {
    pub fn check(&self) -> Vec<Diagnostic> {
        let mut checker = CheckerInfo::default();
        checker.check_ast(self);
        checker.diagnostics
    }
}
