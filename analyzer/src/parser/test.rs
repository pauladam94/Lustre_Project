use crate::parser::span::LSpan;
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
            panic!();
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
        panic!();
    }
}
