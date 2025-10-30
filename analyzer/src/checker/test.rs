use colored::Colorize;

use crate::parser::{ast::ast, span::LSpan, test::ok_parse};

/// Verify that the given lustre program :
/// - parse
/// - don't have any diagnostic from the type checker
pub fn ok_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let diags = build_ast.check();
    if !diags.is_empty() {
        println!(
            "{} : diagnostics should be empty\n>>{} = {:#?}",
            ">> ERROR".red(),
            "DIAGS".blue(),
            diags
        );
        assert!(false);
    }
}

pub fn error_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let diags = build_ast.check();
    if diags.is_empty() {
        println!("{}: error expected but no diagnostics", ">> ERROR".red());
        assert!(false);
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

node f(x: int) returns(z: int, y: bool);
let
    a = x + 5;
    z = a + x + 10;
    y = true;
tel
node g(y: int) returns (x: bool);
let
    a = 11234 + f(y);
    x = f(2) == f(a);
tel
",
        )
    }
}
