use crate::diagnostic::ToRange;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use lsp_types::Range;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many0_count;
use nom::multi::many1;
use nom::multi::many1_count;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{
    character::complete::{char, one_of},
    combinator::opt,
    sequence::preceded,
};

#[derive(Clone, Debug, PartialEq, Copy)]
pub(crate) enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TestLiteral {
    txt: Span,
    val: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl ToRange for Literal {
    fn to_range(&self) -> Range {
        todo!()
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{i}"),
            Literal::Float(fl) => write!(f, "{fl}"),
            Literal::Bool(b) => write!(f, "{b}"),
        }
    }
}

pub(crate) fn identifier(input: LSpan) -> IResult<LSpan, Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .map(|s| Span::new(s))
    .parse(input)
}

pub(crate) fn integer(input: LSpan) -> IResult<LSpan, i64> {
    recognize(many1_count(digit1))
        .map_res(|s: LSpan| s.parse::<i64>())
        .parse(input)
}

fn bool_parse(input: LSpan) -> IResult<LSpan, bool> {
    alt((value(true, tag("true")), value(false, tag("false")))).parse(input)
}
fn decimal(input: LSpan) -> IResult<LSpan, LSpan> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))
        .parse(input)
}

fn float(input: LSpan) -> IResult<LSpan, f64> {
    alt((
        // Case one: .42
        recognize((
            char('.'),
            decimal,
            opt((one_of("eE"), opt(one_of("+-")), decimal)),
        )), // Case two: 42e42 and 42.42e42
        recognize((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        )), // Case three: 42. and 42.42
        recognize((decimal, char('.'), opt(decimal))),
    ))
    .map_res(|s| s.parse::<f64>())
    .parse(input)
}

pub(crate) fn literal(input: LSpan) -> IResult<LSpan, Literal> {
    alt((
        float.map(|f| Literal::Float(f)),
        integer.map(|i| Literal::Integer(i)),
        bool_parse.map(|b| Literal::Bool(b)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        literal::{bool_parse, identifier, integer, literal},
        test::{error_test, ok_test},
        white_space::ws,
    };

    #[test]
    fn basic_identifier_0() {
        ok_test(identifier, "asvb");
        // ok_test(identifier, "a2_b2");
        // ok_test(identifier, "a_cjdncjdncdj");
        // error_test(identifier, "2a_c");
        // error_test(identifier, "_a_cjncdj");
        // error_test(identifier, "124a_cdj");
    }
    #[test]
    fn basic_identifier_2() {
        ok_test(identifier, "asvb");
        ok_test(identifier, "a2_b2");
        ok_test(identifier, "a_cjdncjdncdj");
        error_test(identifier, "2a_c");
        error_test(identifier, "_a_cjncdj");
        error_test(identifier, "124a_cdj");
    }
    #[test]
    fn basic_integer() {
        ok_test(integer, "23");
        ok_test(integer, "23232392439832");
        error_test(integer, "?!!2134");
        error_test(integer, "abc2134");
    }

    #[test]
    fn basic_bool() {
        ok_test(bool_parse, "true");
        ok_test(bool_parse, "false");
        error_test(bool_parse, "ffalse");
        error_test(bool_parse, "atrue");

        ok_test(ws(bool_parse), "  true ");
        ok_test(
            ws(bool_parse),
            "
            false ",
        );
        ok_test(ws(bool_parse), " falsefalse");
        error_test(ws(bool_parse), " tttrue");
        error_test(ws(bool_parse), "ffalse");
    }

    #[test]
    fn bool_literal() {
        ok_test(ws(literal), "  true ");
        ok_test(
            ws(literal),
            "
            false ",
        );
        ok_test(ws(literal), "falsefalse");
        error_test(ws(literal), " tttrue");
        error_test(ws(literal), "ffalse");
    }

    #[test]
    fn float_literal() {
        ok_test(literal, "0.2345");
        error_test(literal, "abc0.2");
    }
}
