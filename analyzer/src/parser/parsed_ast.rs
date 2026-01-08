use crate::parser::parsed_node::ParsedNode;

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedAst {
    pub(crate) nodes: Vec<ParsedNode>,
}

impl std::fmt::Display for ParsedAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            write!(f, "{node}")?;
            if i != self.nodes.len() - 1 {
                write!(f, "\n\n")?;
            } else {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
