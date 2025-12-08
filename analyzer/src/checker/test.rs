use colored::Colorize;

use crate::parser::{ast::ast, span::LSpan, test::ok_parse};

/// Verify that the given lustre program :
/// - parse
/// - don't have any diagnostic from the type checker
pub fn ok_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let (diags, _) = build_ast.check();
    if !diags.is_empty() {
        eprintln!(
            "{} : diagnostics should be empty\n>>{} = {:#?}",
            ">> ERROR".red(),
            "DIAGS".blue(),
            diags
        );
        panic!()
    }
}

pub fn error_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let (diags, _) = build_ast.check();
    if diags.is_empty() {
        println!(
            "{}: error expected but no diagnos + 1;tics",
            ">> ERROR".red()
        );
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use crate::checker::test::{error_check, ok_check};

    #[test]
    fn test1_ok() {
        ok_check(
            "
node f() returns();
let
tel
",
        )
    }
    #[test]
    fn test2_ok() {
        ok_check(
            "
node f(x: int) returns(z: int);
let
    z = x;
tel
",
        )
    }
    #[test]
    fn test3_error() {
        error_check(
            "
node f(x: int, x : int) returns(z: int);
let
    z = x;
tel
",
        )
    }

    #[test]
    fn test4_error() {
        error_check(
            "
node f(x: int) returns(z: int);
let
    z = 2 + a;
tel
",
        )
    }
    #[test]
    fn test5_error() {
        error_check(
            "
node f(x: int) returns(z: int);
let
    z = 2 + true;
tel
",
        )
    }
    #[test]
    fn test6_error() {
        ok_check(
            "
node f(x: int) returns(z: int);
let
    z = x + 10;
tel
",
        )
    }
    #[test]
    fn test7_ok() {
        ok_check(
            "
node f(x: int) returns(z: int, y: bool);
let
    a = x + 5;
    z = a + x + 10;
    y = true;
tel
",
        )
    }
    #[test]
    fn test8_ok() {
        ok_check(
            "

node f(x: int) returns(z: int);
let
    a = x + 5;
    z = a + x + 10;
tel
node g(y: int) returns (x: bool);
let
    a = 11234 + f(y);
    x = f(2) == f(a);
tel
",
        )
    }
    #[test]
    fn id_ok() {
        ok_check(
            "

node id() returns(z: int);
let
    z = 0 fby z;
tel
",
        )
    }

    #[test]
    fn fibonacci_1_error() {
        error_check(
            "
node fibo() returns (x : int);
let
    x = 1 fby x + (1 fby (1 fby x)); 
tel

#[test]
node verify_5_first_value() returns (z: bool);
let
    z = fibo([(), (), (), (), ()]) == [false, true, true, true, true];        
tel
        ",
        );
    }

    #[test]
    fn fibonacci_4_error() {
        error_check(
            "
node fibo() returns (z : int);
let
	x0 = pre z;
	x1 = pre pre z;
	add = x0 + x1;
	z = 1 -> (1 -> add);
tel

#[test]
node test2() returns (z : bool);
let
	lhs = fibo([(), (), (), ()]);
	rhs = [2, 4, 6];
	z = lhs == rhs;
tel",
        );
    }
}
