use crate::parser::span::LSpan;
use nom::IResult;

pub trait _SpanParse
where
    Self: Sized,
{
    fn span_parse<'a>(input: LSpan<'a>) -> IResult<LSpan<'a>, Self>;
}
