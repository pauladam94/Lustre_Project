use crate::diagnostic::ToRange;
use crate::parser::span::LSpan;
use crate::parser::span::Span;
use crate::parser::var_type::VarType;
use lsp_types::Range;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
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

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unit,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
}

impl Value {
    pub fn tuple_from_vec(v: Vec<Value>) -> Self {
        if v.len() == 1 {
            v[0].clone()
        } else {
            Value::Tuple(v)
        }
    }
    pub fn unwrap_array(inputs: Vec<Value>) -> Option<Vec<Vec<Value>>> {
        let mut res = vec![];
        for input in inputs.into_iter() {
            match input {
                Value::Array(values) => res.push(values),
                _ => return None,
            }
        }
        Some(res)
    }

    pub fn get_type(&self) -> VarType {
        match self {
            Value::Unit => VarType::Unit,
            Value::Integer(_) => VarType::Int,
            Value::Float(_) => VarType::Float,
            Value::Bool(_) => VarType::Bool,
            Value::Tuple(v) => {
                if v.is_empty() {
                    VarType::Unit
                } else {
                    VarType::Tuple(v.iter().map(|v| v.get_type()).collect())
                }
            }
            Value::Array(v) => {
                // Check more things maybe
                if v.is_empty() {
                    VarType::Array(Box::new(VarType::Unit))
                } else {
                    VarType::Array(Box::new(v[0].get_type()))
                }
            }
        }
    }
}

impl ToRange for Value {
    fn to_range(&self) -> Range {
        todo!()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(fl) => write!(f, "{fl}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Tuple(vec) => {
                if vec.is_empty() {
                    write!(f, "{}", VarType::Unit)
                } else {
                    write!(f, "(")?;
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{val}")?;
                        if i != vec.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, ")")
                }
            }
            Value::Array(vec) => {
                if vec.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(f, "[")?;
                    for (i, val) in vec.iter().enumerate() {
                        write!(f, "{val}")?;
                        if i != vec.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }
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
    value((), (tag("("), multispace0, tag(")"))).parse(input)
}

pub(crate) fn literal(input: LSpan) -> IResult<LSpan, Value> {
    alt((
        unit.map(|_| Value::Unit),
        float.map(Value::Float),
        integer.map(Value::Integer),
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
        // ok_test(identifier, "a2_b2");
        // ok_test(identifier, "a_cjdncjdncdj");
        // error_test(identifier, "2a_c");
        // error_test(identifier, "_a_cjncdj");
        // error_test(identifier, "124a_cdj");
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
