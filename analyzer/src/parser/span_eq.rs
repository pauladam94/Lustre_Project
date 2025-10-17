pub trait SpanEq {
    fn span_eq(&self, other: Self) -> bool;
}
