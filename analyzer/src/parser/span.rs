use nom_locate::LocatedSpan;

pub type LSpan<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Span {
    /// The offset represents the position of the fragment relatively to
    /// the input of the parser. It starts at offset 0.
    offset: usize,
    /// The line number of the fragment relatively to the input of the
    /// parser. It starts at line 1.
    line: u32,
    /// The fragment that is spanned.
    /// The fragment represents a part of the input of the parser.
    fragment: String,
}

impl Span {
    pub fn new(input: LSpan) -> Self {
        Self {
            offset: input.location_offset(),
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
    pub fn location_offset(&self) -> usize {
        self.offset
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}
