#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Tag {
    Test,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tag::Test => write!(f, "test"),
        }
    }
}
