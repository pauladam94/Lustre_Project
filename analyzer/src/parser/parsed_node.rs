use crate::ast::{expression::Expr, ftag::Tag};
use crate::parser::{span::Ident, span::Span, var_type::InnerVarType, var_type::VarType};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParsedNode {
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
    /// Let Bindings accept definition like this :
    /// ```
    /// let
    ///     (x, y) = (2, 3);
    /// tel
    /// ```  
    pub(crate) let_bindings: Vec<(Vec<Ident>, Expr)>,
    pub(crate) span_semicolon_equations: Vec<Span>,
}

impl std::fmt::Display for ParsedNode {
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
            if s.len() == 0 {
                writeln!(f, "\t{} = {e};", s[0])?;
            } else {
                write!(f, "\t(")?;
                for (i, name) in s.iter().enumerate() {
                    write!(f, "{name}")?;
                    if i != s.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                writeln!(f, ")= {e};")?;
            }
        }
        write!(f, "tel")
    }
}
