#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instant {
    Initial,
    NonInitial,
}

impl std::fmt::Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instant::Initial => write!(f, "init"),
            Instant::NonInitial => write!(f, "not init"),
        }
    }
}
impl Instant {
    pub const INIT: Self = Self::Initial;

    pub fn step(&mut self) {
        if let Self::Initial = self {
            *self = Self::NonInitial;
        }
    }
    pub fn is_init(&self) -> bool {
        self == &Self::Initial
    }
}
