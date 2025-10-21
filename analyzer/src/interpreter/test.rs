use crate::parser::{
    ast::{Ast, ast},
    span::LSpan,
    test::ok_parse,
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

impl BuildInterpreter for Ast {
    fn build_interpreter(&self) -> impl Interpreter {
        SimpleInterpreter { vars: vec![] }
    }
}

pub fn ok_interpretation(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    // let interpreter = build_ast.interpret();
}

#[cfg(test)]
mod test {
    use crate::interpreter::test::ok_interpretation;

    #[test]
    fn test_1() {
        ok_interpretation(
            "
node id(x, y : int) returns (a, b : int);
let
    a = x;
    b = y;
tel

#[test]
node has_been_true() returns (z: bool);
let
    z = has_been_true([false, true, true]) == [false, true, true];        
tel
        ",
        );
    }
}
