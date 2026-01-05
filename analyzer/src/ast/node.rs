use crate::ast::expression::Expr;
use crate::parser::ftag::Tag;
use crate::parser::literal::Value;
use crate::parser::span::Ident;
use crate::parser::span::PositionEnd;
use crate::parser::span::Span;
use crate::parser::var_type::InnerVarType;
use crate::parser::var_type::VarType;
use lsp_types::Position;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) span_node: Span,
    pub(crate) span_returns: Span,
    pub(crate) span_let: Span,
    pub(crate) span_tel: Span,
    pub(crate) span_semicolon: Span,

    pub(crate) tag: Option<(Span, Tag)>,
    pub(crate) name: Span,
    pub(crate) inputs: Vec<(Ident, VarType)>,
    pub(crate) vars: Vec<(Ident, VarType)>,
    pub(crate) outputs: Vec<(Ident, VarType)>,
    pub(crate) let_bindings: Vec<(Ident, Expr)>,
    pub(crate) span_semicolon_equations: Vec<Span>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some((_, t)) = &self.tag {
            writeln!(f, "#[{t}]")?;
        }
        write!(f, "node {}(", self.name)?;
        if self.inputs.len() != 1 || self.inputs[0].1.inner != InnerVarType::Unit {
            for (i, (s, t)) in self.inputs.iter().enumerate() {
                write!(f, "{s} : {t}")?;
                if i != self.inputs.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, ") returns (")?;
        for (i, (s, t)) in self.outputs.iter().enumerate() {
            write!(f, "{s} : {t}")?;
            if i != self.outputs.len() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, ");")?;

        writeln!(f, "let")?;
        for (s, e) in self.let_bindings.iter() {
            writeln!(f, "\t{s} = {e};")?;
        }
        write!(f, "tel")
    }
}

impl Node {
    pub fn hint_reduced(&self) -> (Position, String) {
        (
            self.tag.as_ref().unwrap().0.position_end(),
            if self.is_only_true_equations() {
                " ✅".to_string()
            } else {
                " ❌".to_string()
            },
        )
    }
    pub fn is_test(&self) -> bool {
        self.tag.is_some() && self.outputs.len() == 1
    }
    pub fn is_only_true_equations(&self) -> bool {
        self.let_bindings.len() == 1 // it has one equation
        && self.let_bindings[0].0.fragment() == self.outputs[0].0.fragment() // the only equation is the one of the output 
        && self.let_bindings[0].1 == Expr::Lit(Value::Bool(true)) // The only equation is "= true;"
    }
    pub fn push_expr(&mut self, name: Span, expr: Expr) {
        self.let_bindings.push((name, expr));
    }
    pub fn shell_from_node(&self) -> Self {
        let Self {
            span_node,
            span_returns,
            span_let,
            span_tel,
            span_semicolon,
            tag,
            name,
            inputs,
            vars,
            outputs,
            let_bindings: _,
            span_semicolon_equations,
        } = self;

        Self {
            // todo use better patter for this
            span_node: span_node.clone(),
            span_returns: span_returns.clone(),
            span_let: span_let.clone(),
            span_tel: span_tel.clone(),
            span_semicolon: span_semicolon.clone(),
            tag: tag.clone(),
            name: name.clone(),
            inputs: inputs.clone(),
            vars: vars.clone(),
            outputs: outputs.clone(),
            let_bindings: vec![],
            span_semicolon_equations: span_semicolon_equations.clone(),
        }
    }
}