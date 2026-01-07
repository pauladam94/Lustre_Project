use crate::ast::literal::Value;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::white_space::ws;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::alphanumeric1,
    character::complete::digit1,
    character::complete::{char, one_of},
    combinator::opt,
    combinator::recognize,
    combinator::value,
    multi::many0,
    multi::many0_count,
    multi::many1,
    multi::many1_count,
    sequence::pair,
    sequence::preceded,
    sequence::terminated,
};

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
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
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

fn unit(input: LSpan) -> IResult<LSpan, ()> {
    value((), (ws(tag("(")), ws(tag(")")))).parse(input)
}

pub(crate) fn literal(input: LSpan) -> IResult<LSpan, Value> {
    alt((
        unit.map(|_| Value::Unit),
        float.map(Value::Float),
        integer.map(Value::Int),
        bool_parse.map(Value::Bool),
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
        ok_test(identifier, "asvb  rest");
        ok_test(identifier, "a2_b2");
        ok_test(identifier, "a_cjdncjdncdj");
        error_test(identifier, "2a_c");
        error_test(identifier, " 4a_cjncdj");
        error_test(identifier, "124a_cdj");
    }
    #[test]
    fn basic_identifier_2() {
        ok_test(identifier, "asvb  rest");
        ok_test(identifier, "a2_b2 rest");
        ok_test(identifier, "a_cjdncjdncdj rest");
        error_test(identifier, "2a_c rest");
        ok_test(identifier, "_a_cjncdj rest");
        error_test(identifier, "124a_cdj rest");
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
