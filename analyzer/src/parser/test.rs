use crate::parser::{
    ast::ast,
    double_visitor::{DoubleTogetherVisitor, ShallowEq},
    span::LSpan,
};
use colored::Colorize;
use nom::{Parser, error::ParseError};

pub fn ok_test<'a, O, E: ParseError<LSpan<'a>>, F>(mut f: F, input: &'a str)
where
    F: Parser<LSpan<'a>, Output = O, Error = E>,
    E: std::fmt::Debug,
{
    let s = LSpan::new(input);
    match f.parse(s) {
        Ok(_) => {}
        Err(err) => {
            println!("{}\n{}", ">> input :".blue(), input);
            println!("{}\n{:#?}", ">> ERROR:".red(), err);
            assert!(false);
        }
    }
}
pub fn error_test<'a, O, E: ParseError<LSpan<'a>>, F>(mut f: F, input: &'a str)
where
    F: Parser<LSpan<'a>, Output = O, Error = E>,
    O: std::fmt::Debug,
{
    let s = LSpan::new(input);
    if let Ok((rest, output)) = f.parse(s) {
        println!("{}:\n{}", ">> input".blue(), input);
        println!("{}\n{:#?}", ">> output :".red(), output);
        println!("{}\n{}", ">> rest :".red(), rest);
        assert!(false);
    }
}

pub fn error_parse(input: &str) {
    let span = LSpan::new(input);

    if let Ok((_, res)) = ast(span) {
        println!("{}\n{input}", ">> input : ".blue());
        println!("{}\n{res}", ">> result : ".red());
        assert!(false);
    }
}

pub fn ok_parse(input: &str) {
    struct Test {
        name: &'static str,
        passed: bool,
    }
    println!("\n{}\n\"{input}\"", ">> in :".blue());

    let mut tests = [
        Test {
            name: "in | parse | display == in",
            passed: false,
        },
        Test {
            name: "in | parse_rest == \"\"",
            passed: false,
        },
        Test {
            name: "in | parse | display | parse shallow== in | parse",
            passed: false,
        },
        Test {
            name: "in | parse | display | parse | display == in | parse | display",
            passed: false,
        },
    ];
    let span = LSpan::new(input);

    match ast(span) {
        Err(err) => {
            println!("\n{} parsing of \"in\": {:#?}", "FAILED".red(), err);
        }
        Ok((in_rest, in_parse)) => {
            let in_parse_display = format!("{in_parse}");
            tests[0].passed = in_parse_display == input;

            println!("{}\n{in_parse}", ">> in | parse :".blue());
            println!("{}\n{in_rest}", ">> in | parse_rest".purple());

            tests[1].passed = in_rest.fragment().is_empty();

            let span = LSpan::new(&in_parse_display);

            match ast(span) {
                Err(err) => {
                    println!(
                        "\n{} parsing of \"in | parse | display\": {}",
                        "FAILED".red(),
                        err
                    );
                }
                Ok((_, in_parse_display_parse)) => {
                    let mut shallow_eq = ShallowEq::default();
                    shallow_eq.walk(&in_parse, &in_parse_display_parse);
                    tests[2].passed = shallow_eq.is_eq();

                    // in_parse_display_parse == in_parse;
                    let in_parse_display_parse_display = format!("{in_parse_display_parse}");
                    tests[3].passed = in_parse_display_parse_display == in_parse_display;

                    println!(
                        "{}\n{in_parse_display_parse}",
                        ">> in | parse | display | parse :".blue()
                    );
                }
            }
        }
    }

    let mut one_test_failed = false;
    for (i, test) in tests.iter().enumerate() {
        if i == 0 {
            // skip first test because I don't care
            continue;
        }
        if test.passed {
            print!("{}", "PASSED: ".green());
        } else {
            one_test_failed = true;
            print!("{}", "FAILED: ".red());
        }
        println!("{}", test.name);
    }

    if one_test_failed {
        assert!(false)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::test::{error_parse, ok_parse};

    #[test]
    fn f_no_args_no_equation_ok() {
        ok_parse(
            "
node f() returns ();
let
tel
",
        );
    }
    #[test]
    fn empty_string_ok() {
        ok_parse("");
    }
    #[test]
    fn t15_ok() {
        ok_parse(
            "
node f() returns ( b : int );
let
tel
",
        );
    }
    #[test]
    fn t11_ok() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let

tel
",
        );
    }
    #[test]
    fn t13_ok() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let
    x = y;
tel
",
        );
    }
    #[test]
    fn t14_ok() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let
    x = (y + x);
tel
",
        );
    }
    #[test]
    fn t12_ok() {
        ok_parse(
            "
node f (a: int) returns (b:int) ;
let

tel
",
        );
    }

    #[test]
    fn t2_ok() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
tel
",
        );
    }

    #[test]
    fn t3_ok() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel
",
        );
    }

    #[test]
    fn t4_ok() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel

node other_function(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel
",
        );
    }

    #[test]
    fn t5_ok() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel

node other_function(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel
",
        );
    }

    #[test]
    fn t6_ok() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel

node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel
",
        );
    }
    #[test]
    fn t7() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel

node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel
",
        );
    }
    #[test]
    fn t8_ok() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel

node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel
node fu(i1 : int, i2 : int) returns (f: float);
let
    a = x * x;
    f = 2;
tel
",
        );
    }

    #[test]
    fn t9_ok() {
        ok_parse(
            "
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel
",
        )
    }

    #[test]
    fn t10_error() {
        error_parse(
            "
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = ;
tel
",
        )
    }

    #[test]
    fn t16_error() {
        error_parse(
            "
node ction( returns (f: float);
let
tel
",
        )
    }
    #[test]
    fn t17_ok() {
        ok_parse(
            "
node f(i : int, i : bool) returns (f: float);
let
    a = 1 + 1 + 1 * 1 + 1 + 1 / 1 - abc;
tel
",
        )
    }

    #[test]
    fn t18_ok() {
        ok_parse(
            "
node    fgaaaaaaa () returns       (x : int);

let      x = (1 + 1 * (1 + 1)) / 4; tel


node   f   () returns (x : int);
let x = 2; tel",
        )
    }
    #[test]
    fn func_call_1_ok() {
        ok_parse(
            "
node f(x: int, y: int, z: bool) returns (a : int);
let
    a = 12;
tel

node g() returns (x : int);
let
    x = f(1, 2, 3);
tel",
        )
    }
    #[test]
    fn func_call_2_ok() {
        ok_parse(
            "
node f(x: int, y: int, z: bool) returns (a : int);
let
    a = 12;
tel

node g() returns (x : int);
let
    y = f(x, x, f(1));
    x = f(1, 2, y);
tel",
        )
    }
    #[test]
    fn func_call_1_error() {
        error_parse(
            "
node f(x: int, y: int, z: bool) returns (a : int);
let
    a = 12;
tel

node g() returns (x : int);
let
    y = f(x, x, f(1));
    x = f(1 2 y);
tel",
        )
    }
    #[test]
    fn func_call_2_error() {
        error_parse(
            "
node f(x: int, y: int, z: bool) returns (a : int);
let
    a = 12;
tel

node g() returns (x : int);
let
    y = f(x, x, f(1);
    x = f(1, 2, y);
tel",
        )
    }

    #[test]
    fn multiple_args_one_type_1_ok() {
        ok_parse(
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
    #[test]
    fn multiple_args_one_type_2_ok() {
        ok_parse(
            "
node id(x, y, z : int, a, b : bool) returns (a, b : int);
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
    #[test]
    fn test_arrow_operator_ok() {
        ok_parse(
            "
node f() returns (z : int);
let
    z = 1 -> pre z;
tel

node g() returns (z : int);
let
    z = 1 -> 2 -> pre z;
tel
        ",
        );
    }

    #[test]
    fn arrow_plus_chain_ok() {
        ok_parse(
            "
node a() returns ();
let
	z = 1 -> (1 -> (x0 + x1));
	z = 1 -> (1 -> (x0 fby x1));
tel",
        );
    }
    #[test]
    fn arrow_chain_ok() {
        ok_parse(
            "
node a() returns ();
let
	z = 1 -> (1 -> (x0 -> x1));
	x = 1 -> 2 -> 3;
tel",
        );
    }
    #[test]
    fn chain_operator_1_ok() {
        ok_parse(
            "
node a() returns ();
let
	z = (1 -> (1 + 3) -> x0 + z) -> x1;
tel",
        );
    }
    #[test]
    fn chain_operator_2_ok() {
        ok_parse(
            "
node a() returns ();
let
	z = 1 -> (1 -> (x0 fby x1));
tel",
        );
    }
    #[test]
    fn chain_operator_3_ok() {
        ok_parse(
            "
node a() returns ();
let
	z = 1 + (1 -> (x0 fby x1));
tel",
        );
    }
}

