use crate::{diagnostic::ToRange, token_type::TokenType};
use lsp_types::{Position, Range, SemanticToken};
use nom_locate::LocatedSpan;

pub type LSpan<'a> = LocatedSpan<&'a str>;

pub(crate) type Ident = Span;

#[derive(Debug, Clone, Default, Eq)]
pub struct Span {
    /// The offset represents the position of the fragment relatively to
    /// the input of the parser. It starts at offset 0.
    column: usize,
    /// The line number of the fragment relatively to the input of the
    /// parser. It starts at line 1.
    line: u32,
    /// The fragment that is spanned.
    /// The fragment represents a part of the input of the parser.
    fragment: String,
}

impl std::hash::Hash for Span {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fragment.hash(state);
    }
}
impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment
    }
}
pub trait PositionEnd {
    fn position_end(&self) -> Position;
}
impl PositionEnd for Span {
    fn position_end(&self) -> Position {
        Position {
            line: self.location_line() - 1,
            character: (self.get_column() + self.fragment().len()) as u32 - 1,
        }
    }
}
pub trait PositionEndNextLine {
    fn position_end_next_line(&self) -> Position;
}

impl PositionEndNextLine for Span {
    fn position_end_next_line(&self) -> Position {
        Position {
            line: self.location_line(),
            character: (self.get_column() + self.fragment().len()) as u32 - 1,
        }
    }
}
impl ToRange for LocatedSpan<&str> {
    fn to_range(&self) -> Range {
        Range {
            start: Position {
                line: self.location_line() - 1,
                character: self.get_column() as u32 - 1,
            },
            end: Position {
                line: self.location_line() - 1,
                character: (self.get_column() + self.fragment().len()) as u32 - 1,
            },
        }
    }
}
impl ToRange for Span {
    fn to_range(&self) -> Range {
        Range {
            start: Position {
                line: self.line - 1,
                character: self.column as u32 - 1,
            },
            // TODO Support for multiline Span, or not ?
            end: Position {
                line: self.line - 1,
                character: (self.column + self.fragment.len()) as u32 - 1,
            },
        }
    }
}

impl Span {
    pub fn unit() -> Self {
        Self {
            column: 0,
            line: 0,
            fragment: "()".to_string(),
        }
    }
    pub fn change_text(&mut self, fragment: String) {
        self.fragment = fragment;
    }
    pub fn eq_exact(&self, lhs: &Self) -> bool {
        self.column == lhs.column && self.line == lhs.line && self.fragment() == lhs.fragment()
    }
    pub fn to_semantic_token(&self, token_type: TokenType) -> SemanticToken {
        SemanticToken {
            delta_line: self.location_line() - 1,
            delta_start: self.get_column() as u32 - 1,
            length: self.fragment().len() as u32,
            token_type: token_type as u32,
            token_modifiers_bitset: 0,
        }
    }
}

impl Span {
    pub fn new(input: LSpan) -> Self {
        Self {
            column: input.get_column(),
            line: input.location_line(),
            fragment: input.fragment().to_string(),
        }
    }
    pub fn fragment(&self) -> String {
        self.fragment.clone()
    }
    pub fn location_line(&self) -> u32 {
        self.line
    }
    pub fn get_column(&self) -> usize {
        self.column
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}
