use crate::parser::span::LSpan;
use nom::IResult;

pub trait SpanParse
where
    Self: Sized,
{
    fn span_parse<'a>(input: LSpan<'a>) -> IResult<LSpan<'a>, Self>;
}
