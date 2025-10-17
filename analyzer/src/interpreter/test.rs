use colored::Colorize;
use lustre_parser::{
    ast::{Ast, ast},
    span::Span,
};

enum Value {}

struct SimpleInterpreter {
    vars: Vec<Value>,
}

pub trait Interpreter {
    fn next(&mut self, input: &[i32]) {}
}

impl Interpreter for SimpleInterpreter {}

pub trait BuildInterpreter {
    fn build_interpreter(&self) -> impl Interpreter;
}

impl BuildInterpreter for Ast<'_> {
    fn build_interpreter(&self) -> impl Interpreter {
        SimpleInterpreter { vars: vec![] }
    }
}

pub fn ok_interpretation(input: &str) {
    use lustre_parser::test::ok_parse;

    ok_parse(input);
    let (_, build_ast) = ast(Span::new(input)).unwrap();

    // let interpreter = build_ast.interpret();
}

#[cfg(test)]
mod test {
    use crate::test::ok_interpretation;

    #[test]
    fn test_1() {
        ok_interpretation(
            "
node id(x, y : int) returns (a, b : int);
let
    a = x;
    b = y;
tel;

#[test]
node has_been_true() returns ();
let
    has_been_true(
        [false, true, false, false, false, true],
    ) ==
        [];        
        ",
        );
    }
}
