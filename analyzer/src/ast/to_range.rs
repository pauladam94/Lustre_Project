use lsp_types::Range;

pub trait ToRange {
    fn to_range(&self) -> Range;
}

pub trait Merge {
    fn merge(self, other: Self) -> Self;
}
