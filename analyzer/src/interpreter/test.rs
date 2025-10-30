use crate::{
    checker::test::ok_check,
    interpreter::constant_propagate::PropagateConst,
    parser::{ast::ast, expression::Expr, literal::Value, span::LSpan},
};
use colored::Colorize;

pub fn ok_interpretation(input: &str) {
    ok_check(input);
    let (_, mut build_ast) = ast(LSpan::new(input)).unwrap();

    build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), build_ast);

    for node in build_ast.nodes.iter() {
        if node.tag.is_some() {
            let equations = &node.let_bindings;

            // More than One equation
            if equations.len() != 1
                || equations[0].0.fragment() != node.outputs[0].0.fragment()
                || equations[0].1 != Expr::Lit(Value::Bool(true))
            {
                assert!(false);
            }
        }
    }

    let compile_ast = build_ast.compile(todo!());

    assert!(false);
}

#[cfg(test)]
mod test {
    use crate::interpreter::test::ok_interpretation;

    #[test]
    fn fibonacci_ok() {
        ok_interpretation(
            "
node fibo() returns (x : int);
let
    x = 1 fby x + (1 fby (1 fby x)); 
tel

#[test]
node verify_5_first_value() returns (z: bool);
let
    z = fibo([(), (), ()]) == [false, true, true];        
tel
        ",
        );
    }
    #[test]
    fn has_been_true_ok() {
        ok_interpretation(
            "
node id(x, y : int) returns (a, b : int);
let
    a = x;
    b = y;
tel

#[test]
node test() returns (z: bool);
let
    z = has_been_true([false, true, true]) == [false, true, true];        
tel
        ",
        );
    }
}
