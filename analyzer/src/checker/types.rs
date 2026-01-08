use crate::{
    ast::{
        ast::Ast, ast_types::AstTypes, binop::BinOp, expression::Expr, literal::Value, node::Node,
        to_range::ToRange, unary_op::UnaryOp,
    },
    checker::{function_type::FunctionType, infer_types::InferLen},
    parser::{
        span::{Ident, PositionEnd, Span},
        var_type::{InnerVarType, VarType},
    },
};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, InlayHint, InlayHintKind, InlayHintLabel, Position,
};

struct CheckerInfo<'a> {
    types: &'a mut AstTypes,
    search_stack: Vec<Span>,
    // current_node: Ident,
    diagnostics: Vec<Diagnostic>,
    hints: Vec<InlayHint>,
}

pub(crate) fn numeral_string(i: usize) -> String {
    if i == 0 {
        "1st".to_string()
    } else {
        format!("{}nd", i + 1)
    }
}
impl<'a> CheckerInfo<'a> {
    fn new(types: &'a mut AstTypes) -> CheckerInfo<'a> {
        Self {
            types: types,
            search_stack: vec![],
            // current_node: Span::default(),
            diagnostics: vec![],
            hints: vec![],
        }
    }
    fn number_diagnostics(&self) -> usize {
        self.diagnostics.len()
    }
    // fn set_current_node(&mut self, name: &Ident) {
    //     self.current_node = name.clone()
    // }

    fn push_new_search(&mut self, var: Span) {
        self.search_stack.push(var)
    }
    fn pop_search(&mut self) {
        self.search_stack.pop();
    }
    fn last_search(&self) -> Option<&Span> {
        self.search_stack.last()
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

    fn get_type_expression(&mut self, node: &Node, expr: &Expr) -> Option<VarType> {
        match expr {
            Expr::BinOp {
                lhs,
                op: BinOp::Add | BinOp::Sub | BinOp::Div | BinOp::Mult | BinOp::Fby,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_expression(node, lhs)?;
                let rt = self.get_type_expression(node, rhs)?;
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
                let lt = self.get_type_expression(node, lhs)?;
                let rt = self.get_type_expression(node, rhs)?;
                let message = format!(
                    "Got type '{}' on the left and '{}' on the right but expected to have the same type.",
                    lt, rt
                );
                match lt.merge(rt) {
                    Some(_) => Some(VarType {
                        initialized: true,
                        inner: InnerVarType::Bool,
                    }),
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
                op: BinOp::Arrow,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_expression(node, lhs)?;
                let rt = self.get_type_expression(node, rhs)?;
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
                                message: format!("Got type '{}' on the left and '{}' on the right but expected to have the same type.", lt, rt),
                                severity: Some(DiagnosticSeverity::ERROR),
                                range: span_op.to_range(),
                                ..Default::default()
                            });
                    None
                }
            }
            Expr::BinOp {
                lhs,
                op: op @ BinOp::Caret,
                span_op,
                rhs,
            } => {
                let lt = self.get_type_expression(node, lhs)?;
                let rt = self.get_type_expression(node, rhs)?;

                if rt.inner != InnerVarType::Int {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Expected type `int` on the left of '{}' but got '{}'.",
                            op, rt
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: span_op.to_range(),
                        ..Default::default()
                    });
                    return None;
                }
                let len = match rhs.get_value() {
                    Some(Value::Int(index)) => match usize::try_from(index) {
                        Ok(i) => InferLen::Known(i),
                        Err(_) => InferLen::Unknown,
                    },
                    _ => InferLen::Unknown,
                };
                Some(lt.array_of(len))
            }
            Expr::UnaryOp {
                op: op @ UnaryOp::Inv,
                span_op,
                rhs,
            } => {
                let rt = self.get_type_expression(node, rhs)?;
                match rt.inner {
                    InnerVarType::Unit | InnerVarType::Char | InnerVarType::String => {
                        self.push_diagnostic(Diagnostic {
                            message: format!(
                                "`{}` Operation not defined for `{}` type.",
                                op, rt.inner
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            range: span_op.to_range(),
                            ..Default::default()
                        });
                        None
                    }
                    InnerVarType::Int
                    | InnerVarType::Float
                    | InnerVarType::Bool
                    | InnerVarType::Tuple(_)
                    | InnerVarType::Array { t: _, len: _ } => Some(rt),
                }
            }
            Expr::UnaryOp {
                op: UnaryOp::Pre,
                span_op,
                rhs,
            } => {
                let mut t = self.get_type_expression(node, rhs)?;
                if t.is_initialized() {
                    t.uninitialized();
                    Some(t)
                } else {
                    self.push_diagnostic(Diagnostic {
                                message: "Using pre operator on a not initialized value. This cannot be recovered with any other operator.".to_string(),
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
                let t = self.get_type_expression(node, rhs)?;
                Some(t)
            }
            Expr::Variable(s) => self.get_type_var(node, s, false),
            Expr::Lit(val) => Some(val.get_type()),
            Expr::Index { expr, index } => {
                let t_index = self.get_type_expression(node, index)?;
                if t_index.inner.merge(InnerVarType::Int).is_none() {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Expected type '{}' but found '{}'.",
                            InnerVarType::Int,
                            t_index
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: index.to_range(),
                        ..Default::default()
                    });
                    return None;
                }

                let texpr = self.get_type_expression(node, expr)?;

                if let Some(Value::Int(index_value)) = index.get_value() {
                    if let Some(t) = texpr.index(index_value) {
                        Some(t)
                    } else {
                        self.push_diagnostic(Diagnostic {
                            message: format!(
                                "Cannot index at value '{}' inside '{}' of type '{}'.",
                                index_value, expr, texpr
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            // range: expr.to_range.mergeindex.to_range(),
                            ..Default::default()
                        });
                        None
                    }
                } else {
                    None
                }
            }
            Expr::Array(arr) => {
                let mut t0 = None;
                let mut initialized = true;
                for e in arr.iter() {
                    match &t0 {
                        None => {
                            let t1 = self.get_type_expression(node, e)?;
                            initialized = initialized && t1.initialized;
                            t0 = Some(t1);
                        }
                        Some(t0) => {
                            let t1 = self.get_type_expression(node, e)?;
                            initialized = initialized && t1.initialized;
                            if !t1.equal_without_pre(t0) {
                                return None;
                            }
                        }
                    }
                }
                Some(VarType {
                    initialized,
                    inner: InnerVarType::Array {
                        t: Box::new(t0?.inner),
                        len: InferLen::Known(arr.len()),
                    },
                })
            }
            Expr::Tuple(arr) => {
                let mut types = vec![];
                let mut initialized = true;

                for e in arr.iter() {
                    let t = self.get_type_expression(node, e)?;
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

            Expr::If { cond, yes, no } => {
                let tcond = self.get_type_expression(node, cond)?;
                let tyes = self.get_type_expression(node, yes)?;
                let tno = self.get_type_expression(node, no)?;

                if tcond == InnerVarType::Bool && tyes.equal_without_pre(&tno) {
                    Some(VarType {
                        initialized: tcond.initialized && tyes.initialized && tno.initialized,
                        inner: tyes.inner,
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Get the type of a given function considering that its arguments
    ///
    /// Indeed the type of a function can be lifted.
    ///
    /// In lustre writing a node of type `int -> int` like this :
    /// ```
    /// node f(x: int) returns (z: int);
    /// let
    ///     z = x + x;
    /// tel
    /// ```
    ///
    /// Can be called those three ways :
    /// - f(3) with 3 a constant int
    ///   Here we consider the type of `f : int -> int`
    /// - f(x) with x an int that is a flow of integers (same as previous case)
    /// - f([2, 3, 4, x])
    ///   Here we consider the type of `f : [int] -> [int]`
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
        // A function type call can be lifted
        // A function of type 'int -> int'
        enum FunctionCallType {
            // Always begin as unknown
            Unknown,
            // can be simply the type `flow int -> flow int`
            Simple,
            // can be lifted to a function of type '[int] -> [int]'
            Array,
        }
        let mut call_type = FunctionCallType::Unknown;
        match self.types.get_node_type(name) {
            Some(ft) => {
                // Arguments types for a Array call
                let mut args_array_length = None;
                // Cannot call function defined after the current node
                let index_current_node = &self.types.get_nodes_index(&node.name).unwrap();
                let index_called_node = &self.types.get_nodes_index(name).unwrap();
                if index_called_node > index_current_node {
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "The function `{}` is defined after the place it is being called.",
                            name
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: name.to_range(),
                        ..Default::default()
                    });
                    return None;
                } else if index_called_node == index_current_node {
                    self.push_diagnostic(Diagnostic {
                        message: format!("Cannot call recursively the same node.",),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: name.to_range(),
                        ..Default::default()
                    });
                    return None;
                }

                // fix for self being immutably borrowed then mutable borrowed
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
                // Here we are gonna determined in which call type
                // of the given node we are in.
                //
                // We begin with unkown and diverge to `simple` or `array`
                // call type after the first argument.
                //
                // Other arguments are also check to know if they match the
                // guess call type.
                for (i, (arg, (_, expected_type))) in args.iter().zip(ft.inputs.iter()).enumerate()
                {
                    match self.get_type_expression(node, arg) {
                        Some(t) => match call_type {
                            FunctionCallType::Unknown => {
                                // Always reachable because we begin with Unknown type
                                if &t == expected_type {
                                    call_type = FunctionCallType::Simple;
                                } else if t.equal_array_of(expected_type) {
                                    call_type = FunctionCallType::Array;
                                    // We now know the length of arrays for this `array` call type.
                                    // This is not true, we might have to do one more call to propagate const
                                    args_array_length = Some(t.get_length_array()?);
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
                                } else {
                                    let expected_length = args_array_length?;
                                    let given_length = t.get_length_array()?;
                                    if given_length != expected_length {
                                        self.push_diagnostic(Diagnostic {
                                            message: format!(
                                                "{} arguments of function {} is an array of length {} but expected length {}.",
                                                numeral_string(i),
                                                name,
                                                given_length,
                                                expected_length
                                             ),
                                            severity: Some(DiagnosticSeverity::ERROR),
                                            range: name.to_range(),
                                            ..Default::default()
                                        });
                                        return None;
                                    }
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
                                inner: InnerVarType::Array {
                                    t: Box::new(t.inner),
                                    len: InferLen::Known(args_array_length.unwrap()),
                                },
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

    fn search_type_var(&mut self, node: &Node, var: &Ident) -> Option<VarType> {
        if self.search_stack.contains(var) {
            self.push_diagnostic(Diagnostic {
                message: format!("Need more type information on {}", var),
                severity: Some(DiagnosticSeverity::ERROR),
                range: var.to_range(),
                ..Default::default()
            });
            return None;
        } else {
            self.push_new_search(var.clone());
        }
        let var = self.last_search().unwrap();
        for (name, expr) in node.let_bindings.iter() {
            if name == var {
                let var_type = self.get_type_expression(node, expr);
                self.types
                    .insert_local_type(&node.name, name.clone(), var_type.clone());
                self.pop_search();
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
    fn get_type_var(&mut self, node: &Node, var: &Span, should_search: bool) -> Option<VarType> {
        if should_search {
            return self.search_type_var(node, var);
        }
        match self.types.get_type_var(&node.name, var) {
            // Type already computed for this variable
            Some(Some(t)) => Some(t.clone()),
            // Variable not yet type checked
            Some(None) => self.search_type_var(node, var),
            // Variable not defined
            None => {
                self.push_diagnostic(Diagnostic {
                    message: format!("No Equation found for '{}'", var),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: var.to_range(),
                    ..Default::default()
                });
                None
            }
        }
    }

    /// Setup partial Local Type in the Checker
    /// - Check that variables are not defined twice in left of equations
    fn setup_local_types(&mut self, node: &Node) {
        // insert all inputs types
        for (name, t) in node.inputs.iter() {
            self.types
                .insert_local_type(&node.name, name.clone(), Some(t.clone()));
        }

        for (name, _) in node.let_bindings.iter() {
            if self.types.contains_key_local_type(&node.name, name) {
                self.diagnostics.push(Diagnostic {
                    message: format!("Equation for '{}' already defined.", name),
                    severity: Some(DiagnosticSeverity::ERROR),
                    range: name.to_range(),
                    ..Default::default()
                });
            } else {
                self.types.insert_local_type(&node.name, name.clone(), None);
            }
        }
        for (name, t) in node.outputs.iter() {
            self.types
                .insert_local_type(&node.name, name.clone(), Some(t.clone()));
        }
    }

    fn check_cycle_from_expr(&mut self, node: &Node, seen: &mut Vec<String>, expr: &Expr) {
        match expr {
            // In those 2 cases we cut the chase because
            // `fby` and `pre` cut temporal cycle.
            Expr::BinOp { op: BinOp::Fby, .. }
            | Expr::UnaryOp {
                op: UnaryOp::Pre, ..
            } => {}
            Expr::BinOp {
                lhs,
                op: _,
                span_op: _,
                rhs,
            } => {
                self.check_cycle_from_expr(node, seen, lhs);
                self.check_cycle_from_expr(node, seen, rhs);
            }
            Expr::UnaryOp {
                op: _,
                span_op: _,
                rhs,
            } => {
                self.check_cycle_from_expr(node, seen, rhs);
            }
            Expr::If { cond, yes, no } => {
                self.check_cycle_from_expr(node, seen, cond);
                self.check_cycle_from_expr(node, seen, yes);
                self.check_cycle_from_expr(node, seen, no);
            }
            Expr::Index { expr, index } => {
                self.check_cycle_from_expr(node, seen, expr);
                self.check_cycle_from_expr(node, seen, index);
            }
            Expr::Array(exprs)
            | Expr::Tuple(exprs)
            | Expr::FCall {
                name: _,
                args: exprs,
            } => {
                for expr in exprs {
                    self.check_cycle_from_expr(node, seen, expr);
                }
            }
            Expr::Variable(span) => {
                let var = span.fragment();
                if seen.contains(&var) {
                    let mut s = String::new();
                    for v in seen.iter() {
                        s.push_str(&format!("{v} - "));
                    }
                    self.push_diagnostic(Diagnostic {
                        message: format!(
                            "Cycle found here with variable {span} comming from {}.
                            All cycle : {s}
                            ",
                            seen.first().unwrap()
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        range: span.to_range(),
                        ..Default::default()
                    });
                } else {
                    seen.push(var);
                    self.check_cycle_from(node, seen);
                    seen.pop();
                }
            }
            Expr::Lit(_) => {}
        }
    }
    fn check_cycle_from(&mut self, node: &Node, seen: &mut Vec<String>) {
        for (name, expr) in node.let_bindings.iter() {
            if &name.fragment() == seen.last().unwrap() {
                self.check_cycle_from_expr(node, seen, expr);
            }
        }
    }
    fn check_cycle(&mut self, node: &Node) {
        let mut seen: Vec<String> = vec![];
        for (out, _) in node.outputs.iter() {
            seen.push(out.fragment());
            self.check_cycle_from(node, &mut seen);
            seen.pop();
        }
    }

    fn check_node(&mut self, node: &Node) {
        // self.set_current_node(&node.name);
        self.setup_local_types(node);

        let number_diags = self.number_diagnostics();
        self.check_cycle(node);
        // Stop Here if check_cycle has found a cycle
        if self.number_diagnostics() > number_diags {
            return;
        }

        for (out, t) in node.outputs.iter() {
            match &self.get_type_var(node, out, true) {
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
    fn get_nodes_types(&mut self, nodes: &[Node]) {
        for node in nodes.iter() {
            let (func, diags) = FunctionType::get_function_type(node);
            for diag in diags {
                self.push_diagnostic(diag);
            }
            if self.types.node_defined(&node.name) {
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
            self.types.insert_node(&node.name, func);
        }
    }

    // Push for all equations the type hint that has been
    // computed from the type inference
    fn push_type_hint_equation(&mut self, node: &Node) {
        for (var, _) in node.let_bindings.iter() {
            if let Some(t) = self.get_type_var(node, var, false) {
                self.push_hint(
                    var.position_end(),
                    format!(" : {t}"),
                    Some(InlayHintKind::TYPE),
                );
            }
        }
    }

    // Completely check the types for a given ast
    //
    // The check is modular for every node
    // given the types of all the nodes.
    fn check_ast(&mut self, nodes: &[Node]) {
        self.get_nodes_types(nodes);

        for node in nodes.iter() {
            self.check_node(node);
            self.push_type_hint_equation(node);
        }
    }
}

impl Ast {
    pub fn check(&mut self) -> (Vec<Diagnostic>, Vec<InlayHint>) {
        let Self { nodes, types } = self;
        let (diags, hints) = {
            let mut checker = CheckerInfo::new(types);
            checker.check_ast(nodes);
            (checker.diagnostics, checker.hints)
        };
        (diags, hints)
    }
}
