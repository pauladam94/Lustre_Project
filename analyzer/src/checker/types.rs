use crate::{
    diagnostic::ToRange,
    parser::{
        ast::Ast,
        binop::BinOp,
        expression::Expr,
        literal::Value,
        node::Node,
        span::{Ident, PositionEnd, Span},
        unary_op::UnaryOp,
        var_type::{InnerVarType, VarType},
    },
};
use indexmap::IndexMap;
use lsp_types::{
    Diagnostic, DiagnosticSeverity, InlayHint, InlayHintKind, InlayHintLabel, Position,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FunctionType {
    inputs: IndexMap<Ident, VarType>,
    outputs: IndexMap<Ident, VarType>,
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

#[derive(Default)]
struct CheckerInfo {
    nodes_types: HashMap<Ident, FunctionType>,
    local_types: HashMap<Ident, Option<VarType>>,
    current_node: Ident,
    diagnostics: Vec<Diagnostic>,
    hints: Vec<InlayHint>,
}

impl CheckerInfo {
    fn set_current_node(&mut self, name: &Ident) {
        self.current_node = name.clone()
    }

    fn push_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag)
    }
    fn push_hint(&mut self, position: Position, label: String, kind: Option<InlayHintKind>) {
        self.hints.push(InlayHint {
            position,
            label: InlayHintLabel::String(label),
            kind,
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        })
    }
}

pub(crate) fn numeral_string(i: usize) -> String {
    if i == 0 {
        "1st".to_string()
    } else {
        format!("{}nd", i + 1)
    }
}
impl CheckerInfo {
    fn get_type_equation(&mut self, node: &Node, expr: &Expr) -> Option<VarType> {
        match expr {
            Expr::BinOp {
                lhs,
                op: BinOp::Add | BinOp::Sub | BinOp::Div | BinOp::Mult | BinOp::Fby,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_equation(node, lhs)?;
                let rt = self.get_type_equation(node, rhs)?;
                let message = format!(
                    "Got type '{}' on the left and '{}' on the right\n but expected to have the same type.",
                    lt, rt
                );
                match lt.merge(rt) {
                    Some(t) => Some(t),
                    None => {
                        self.push_diagnostic(Diagnostic {
                            message,
                            severity: Some(DiagnosticSeverity::ERROR),
                            range: span_op.to_range(),
                            ..Default::default()
                        });
                        None
                    }
                }
            }
            Expr::BinOp {
                lhs,
                op: BinOp::Eq | BinOp::Neq | BinOp::Or | BinOp::And,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_equation(node, lhs)?;
                let rt = self.get_type_equation(node, rhs)?;
                if lt == rt {
                    Some(VarType {
                        initialized: true,
                        inner: InnerVarType::Bool,
                    })
                } else {
                    self.push_diagnostic(Diagnostic {
                                message: format!("Got type '{}' on the left and '{}' on the right\n but expected to have the same type.", lt, rt),
                                severity: Some(DiagnosticSeverity::ERROR),
                                range: span_op.to_range(),
                                ..Default::default()
                            });
                    None
                }
            }
            Expr::BinOp {
                lhs,
                op: BinOp::Arrow,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_equation(node, lhs)?;
                let rt = self.get_type_equation(node, rhs)?;
                if lt.is_not_initialized() {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Got type '{}' which is not initialized at first instant.",
                            lt
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: span_op.to_range(),
                        ..Default::default()
                    });
                    return None;
                }
                if lt.equal_without_pre(&rt) {
                    Some(rt.remove_one_pre())
                } else {
                    self.push_diagnostic(Diagnostic {
                                message: format!("Got type '{}' on the left and '{}' on the right\n but expected to have the same type.", lt, rt),
                                severity: Some(DiagnosticSeverity::ERROR),
                                range: span_op.to_range(),
                                ..Default::default()
                            });
                    None
                }
            }
            Expr::UnaryOp {
                op: UnaryOp::Inv,
                span_op: _,
                rhs,
            } => self.get_type_equation(node, rhs),
            Expr::UnaryOp {
                op: UnaryOp::Pre,
                span_op,
                rhs,
            } => {
                let mut t = self.get_type_equation(node, rhs)?;
                if t.is_initialized() {
                    (&mut t).uninitialized();
                    Some(t)
                } else {
                    self.push_diagnostic(Diagnostic {
                                message: format!("Using pre operator on a not initialized value. This cannot be recovered with any other operator."),
                                severity: Some(DiagnosticSeverity::ERROR),
                                range: span_op.to_range(),
                                ..Default::default()
                            });
                    None
                }
            }
            Expr::UnaryOp {
                op: UnaryOp::Not,
                span_op: _,
                rhs,
            } => {
                let t = self.get_type_equation(node, rhs)?;
                Some(t)
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
            Expr::Lit(val) => Some(val.get_type()),
            Expr::Array(arr) => {
                let mut t0 = None;
                let mut initialized = true;
                for e in arr.iter() {
                    match &t0 {
                        None => {
                            let t1 = self.get_type_equation(node, e)?;
                            initialized = initialized && t1.initialized;
                            t0 = Some(t1);
                        }
                        Some(t0) => {
                            let t1 = self.get_type_equation(node, e)?;
                            initialized = initialized && t1.initialized;
                            if !t1.equal_without_pre(t0) {
                                return None;
                            }
                        }
                    }
                }
                Some(VarType {
                    initialized,
                    inner: InnerVarType::Array(Box::new(t0?.inner)),
                })
            }
            Expr::Tuple(arr) => {
                let mut types = vec![];
                let mut initialized = true;

                for e in arr.iter() {
                    let t = self.get_type_equation(node, e)?;
                    initialized = initialized && t.initialized;
                    types.push(t.inner);
                }

                Some(VarType {
                    initialized,
                    inner: InnerVarType::Tuple(types),
                })
            }
            Expr::FCall { name, args } => {
                let args = if args.is_empty() {
                    &vec![Expr::Lit(Value::Unit)]
                } else {
                    args
                };
                self.get_type_function(node, name, args)
            }

            Expr::If {
                cond: _,
                yes: _,
                no: _,
            } => todo!(), // TODO
        }
    }

    fn get_type_function(&mut self, node: &Node, name: &Span, args: &Vec<Expr>) -> Option<VarType> {
        if name == &node.name {
            self.push_diagnostic(Diagnostic {
                message: "Recursive function call are not allowed".to_string(),
                severity: Some(DiagnosticSeverity::ERROR),
                range: name.to_range(),
                ..Default::default()
            });
            return None;
        }
        // A function call can be lifted
        // A function of type 'int -> int'
        // can be lifted to a function of type '[int] -> [int]'
        enum FunctionCallType {
            Unknown,
            Simple,
            Array,
        }
        let mut call_type = FunctionCallType::Unknown;
        match self.nodes_types.get(name) {
            Some(ft) => {
                let ft = ft.clone();
                if ft.inputs.len() != args.len() {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Expected {} arguments for function '{}' but got {} arguments.",
                            ft.inputs.len(),
                            name,
                            args.len()
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: name.to_range(),
                        ..Default::default()
                    });
                    return None;
                }
                for (i, (arg, (_, expected_type))) in args.iter().zip(ft.inputs.iter()).enumerate()
                {
                    match self.get_type_equation(node, arg) {
                        Some(t) => match call_type {
                            FunctionCallType::Unknown => {
                                // Always reachable because
                                if &t == expected_type {
                                    call_type = FunctionCallType::Simple;
                                } else if t.equal_array_of(expected_type) {
                                    call_type = FunctionCallType::Array;
                                } else {
                                    self.push_diagnostic(Diagnostic {
                                        message: format!(
                                            "{} arguments of function {} of type '{}' but expected '{}' or ['{}']",
                                            numeral_string(i),
                                            name,
                                            t,
                                            expected_type,
                                            expected_type
                                        ),
                                        severity: Some(DiagnosticSeverity::ERROR),
                                        range: name.to_range(),
                                        ..Default::default()
                                    });
                                    return None;
                                }
                            }
                            FunctionCallType::Simple => {
                                if &t != expected_type {
                                    self.push_diagnostic_call(name, i, expected_type, t);
                                    return None;
                                }
                            }
                            FunctionCallType::Array => {
                                if !t.equal_array_of(expected_type) {
                                    self.push_diagnostic(Diagnostic {
            message: format!(
                "{} arguments of function {} of type '{}' but expected ['{}']",
                numeral_string(i),
                name,
                t,
                expected_type
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            range: name.to_range(),
            ..Default::default()
        });
                                    return None;
                                }
                            }
                        },
                        None => {
                            self.push_diagnostic(Diagnostic {
                                message: format!(
                                    "{} arguments of function {} does not type check.",
                                    numeral_string(i),
                                    name,
                                ),
                                severity: Some(DiagnosticSeverity::ERROR),
                                range: name.to_range(),
                                ..Default::default()
                            });
                            return None;
                        }
                    }
                }
                match call_type {
                    FunctionCallType::Unknown => unreachable!(),
                    FunctionCallType::Simple => Some(VarType::tuple_from_vec(
                        ft.outputs.values().cloned().collect(),
                    )),
                    FunctionCallType::Array => Some(VarType::tuple_from_vec(
                        ft.outputs
                            .values()
                            .cloned()
                            .map(|t| VarType {
                                initialized: true,
                                inner: InnerVarType::Array(Box::new(t.inner)),
                            })
                            .collect(),
                    )),
                }
            }
            None => {
                self.push_diagnostic(Diagnostic {
                    message: format!("Function '{}' never defined.", name,),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                });
                None
            }
        }
    }

    fn push_diagnostic_call(&mut self, name: &Span, i: usize, expected_type: &VarType, t: VarType) {
        self.push_diagnostic(Diagnostic {
            message: format!(
                "{} arguments of function {} of type '{}' but expected '{}'",
                numeral_string(i),
                name,
                t,
                expected_type
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            range: name.to_range(),
            ..Default::default()
        });
    }

    fn get_type_var(&mut self, node: &Node, var: &Span) -> Option<VarType> {
        for (name, expr) in node.let_bindings.iter() {
            if name == var {
                let var_type = self.get_type_equation(node, expr);
                self.local_types.insert(name.clone(), var_type.clone());
                return var_type;
            }
        }
        self.push_diagnostic(Diagnostic {
            message: format!("No equation found for '{}'", var),
            severity: Some(DiagnosticSeverity::ERROR),
            range: var.to_range(),
            ..Default::default()
        });
        None
    }

    /// Setup partial Local Type in the Checker
    /// - Check that variables are not defined twice in equations
    fn setup_local_types(&mut self, node: &Node) {
        // insert all inputs types
        for (name, t) in node.inputs.iter() {
            self.local_types.insert(name.clone(), Some(t.clone()));
        }

        for (name, _) in node.let_bindings.iter() {
            if self.local_types.contains_key(name) {
                self.diagnostics.push(Diagnostic {
                    message: format!("Equation for '{}' already defined.", name),
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
                            "Error while checking the type of '{}', expected : '{}'.",
                            out, t
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: out.to_range(),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // Get the type of each nodes definition
    fn get_nodes_types(&mut self, x: &Ast) {
        for node in x.nodes.iter() {
            let (func, diags) = FunctionType::get_function_type(node);
            for diag in diags {
                self.push_diagnostic(diag);
            }
            if self.nodes_types.contains_key(&node.name) {
                self.push_diagnostic(Diagnostic {
                    message: format!(
                        "Function name '{}' already defined in this file.",
                        node.name
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: node.name.to_range(),
                    ..Default::default()
                });
                continue;
            }
            self.nodes_types.insert(node.name.clone(), func);
        }
    }

    fn check_ast(&mut self, x: &Ast) {
        self.get_nodes_types(x);

        for node in x.nodes.iter() {
            self.check_node(node);
            for (var, _) in node.let_bindings.iter() {
                match self.local_types.get(var) {
                    Some(Some(t)) => {
                        self.push_hint(
                            var.position_end(),
                            format!(" : {t}"),
                            Some(InlayHintKind::TYPE),
                        );
                    }
                    Some(None) | None => {}
                }
            }
        }
    }
}

impl Ast {
    pub fn check(&self) -> (Vec<Diagnostic>, Vec<InlayHint>) {
        let mut checker = CheckerInfo::default();
        checker.check_ast(self);
        (checker.diagnostics, checker.hints)
    }
}
