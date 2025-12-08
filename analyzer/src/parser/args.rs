use crate::parser::var_type::var_type;
use crate::parser::{
    literal::identifier,
    span::{Ident, LSpan},
    var_type::VarType,
    white_space::ws,
};
use nom::Parser;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, bytes::tag, sequence::separated_pair};

fn arg(input: LSpan) -> IResult<LSpan, (Vec<Ident>, VarType)> {
    separated_pair(
        (
            many0(terminated(ws(identifier), ws(tag(",")))),
            ws(identifier),
        )
            .map(|(mut v, x)| {
                v.push(x);
                v
            }),
        ws(tag(":")),
        ws(var_type),
    )
    .parse(input)
}

pub(crate) fn args(input: LSpan) -> IResult<LSpan, Vec<(Ident, VarType)>> {
    (many0(terminated(ws(arg), ws(tag(",")))), opt(ws(arg)))
        .map(|(l, v)| {
            let mut res = vec![];
            for (names, t) in l {
                for name in names {
                    res.push((name, t.clone()))
                }
            }
            if let Some((names, t)) = v {
                for name in names {
                    res.push((name, t.clone()))
                }
            }
            res
        })
        .parse(input)
}
