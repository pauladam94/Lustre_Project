use ls_types::Range;

pub trait ToRange {
    fn to_range(&self) -> Range;
}
