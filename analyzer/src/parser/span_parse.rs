use nom::IResult;
use crate::parser::span::LSpan;

pub trait SpanParse
where
    Self: Sized,
{
    fn span_parse<'a>(input: LSpan<'a>) -> IResult<LSpan<'a>, Self>;
}
