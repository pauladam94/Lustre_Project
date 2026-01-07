// use logos::{Lexer, Logos, Skip};

// use crate::ast::{ast::Ast, node::Node};

// #[derive(Logos, Debug, PartialEq, Clone)]
// #[logos(extras = (usize, usize))]
// #[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
// pub enum Token {
//     #[token("(*", skip_comments)]
//     Comments,
//     // KEYWORDS
//     #[token("node")]
//     NodeKeyWord,
//     #[token("returns")]
//     Returns,
//     #[token("let")]
//     Let,
//     #[token("tel")]
//     Tel,
//     #[token("fby")]
//     Fby,
//     #[token("pre")]
//     Pre,
//     #[token("->")]
//     Arrow,
//     // TYPE
//     #[token("int")]
//     Int,
//     #[token("float")]
//     Float,
//     #[token("char")]
//     Char,
//     #[token("bool")]
//     Bool,

//     // SYMBOLS
//     #[token(":")]
//     Colon,
//     #[token(";")]
//     SemiColon,
//     #[token(",")]
//     Comma,
//     #[token("(")]
//     ParenOpen,
//     #[token(")")]
//     ParenClose,
//     #[token("[")]
//     BracketOpen,
//     #[token("]")]
//     BracketClose,
//     #[token("{")]
//     BraceOpen,
//     #[token("}")]
//     BraceClose,
//     #[token(".")]
//     Period,
//     #[token("^")]
//     Carrot,

//     // Operator
//     #[token("+")]
//     Add,
//     #[token("-")]
//     Sub,
//     #[token("*")]
//     Mult,
//     #[token("/")]
//     Div,

//     #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
//     Number(f64),
//     #[regex("(_|[a-zA-Z])(_|[0-9]|[a-zA-Z])*", |lex| lex.slice().to_owned())]
//     Identifier(String),
// }

// fn skip_comments(lexer: &mut Lexer<Token>) -> Skip {
//     Skip {}
// }

// pub struct Tokens<'source> {
//     lexer: Lexer<'source, Token>,
//     buf: Vec<Token>,
// }
// impl<'source> Tokens<'source> {
//     fn peek(&mut self) -> Option<Token> {
//         match self.next() {
//             Some(t) => {
//                 self.buf.push(t.clone());
//                 Some(t)
//             }
//             None => None,
//         }
//     }
// }
// impl<'source> Iterator for Tokens<'source> {
//     type Item = Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.lexer.next() {
//             Some(Ok(t)) => Some(t),
//             Some(Err(())) => None,
//             None => todo!(),
//         }
//     }
// }
// impl<'source> DoubleEndedIterator for Tokens<'source> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         if self.buf.is_empty() {
//             None
//         } else {
//             let last = self.buf.last().unwrap();
//             Some(last.clone())
//         }
//     }
// }
// pub enum ParseError {
//     Incomplete(String),
//     Complete,
//     Error,
// }
// pub type Result<O, E = ParseError> = std::result::Result<O, E>;
// impl Parser<Token> for Token {
//     fn parse(self, input: &mut Tokens) -> Result<Token, ParseError> {
//         match input.peek() {
//             Some(t) => {
//                 if t == self {
//                     Ok(t)
//                 } else {
//                     Err(ParseError::Error)
//                 }
//             }
//             None => Err(ParseError::Error),
//         }
//     }
// }

// impl<O, T, const N: usize> Parser<[O; N]> for [T; N]
// where
//     T: Parser<O>,
// {
//     fn parse(self, input: &mut Tokens) -> Result<[O; N], ParseError> {
//         self.try_map(|parser| parser.parse(input))
//     }
// }
// // impl<O, T> Parser<O> for &[T]
// // where
// //     T: Parser<O>,
// // {
// //     fn parse(&self, input: &mut Tokens) -> Result<O, ParseError> {
// //         for parser in self.iter() {
// //             match parser.parse(input) {
// //                 Ok(o) => return Ok(o),
// //                 Err(_) => {}
// //             }
// //         }
// //         Err(ParseError::Error)
// //     }
// // }
// fn alt<O, const N: usize>(parsers: [impl Parser<O>; N]) -> impl Parser<O> {
//     move |input: &mut Tokens| {
//         for parser in parsers {
//             match parser.parse(input) {
//                 Ok(o) => return Ok(o),
//                 Err(_) => {}
//             }
//         }
//         Err(ParseError::Error)
//     }
// }

// impl<T, O> Parser<O> for T
// where
//     T: FnOnce(&mut Tokens) -> Result<O, ParseError>,
// {
//     fn parse(self, input: &mut Tokens) -> Result<O, ParseError> {
//         self(input)
//     }
// }

// pub trait Parser<O> {
//     fn parse(self, input: &mut Tokens) -> Result<O, ParseError>;
//     fn map_parse<S, F>(self, f: F) -> impl Parser<S>
//     where
//         F: Fn(O) -> S,
//         Self: Sized,
//     {
//         move |input: &mut Tokens| self.parse(input).map(|s| f(s))
//     }
// }

// fn lustre_parser_new(input: &str) -> Result<Ast, ParseError> {
//     let lexer = Token::lexer(input);
//     let tokens = Tokens { lexer, buf: vec![] };
//     todo!()
// }
// fn node_new(input: &mut Tokens) -> Result<Node, ParseError> {
//     use Token::*;
//     [
//         NodeKeyWord,
//         ParenOpen,
//         ParenClose,
//         Returns,
//         ParenOpen,
//         ParenClose,
//         SemiColon,
//         Let,
//         Tel,
//     ]
//     .map_parse(|[]| Node {});
//     todo!()
// }
// fn ast_new(input: &mut Tokens) -> Result<Ast, ParseError> {
//     // let a = [Token::NodeKeyWord, Token::ParenOpen, Token::ParenClose]
//     //     .map(|s| {

//     //     })
//     //     .parse(input);
//     todo!()
// }

// #[cfg(test)]
// mod test {
//     use crate::parser::tokenize::{Token, lustre_parser_new};
//     use logos::Logos;

//     #[test]
//     fn test() {
//         let input = "node f() returns (); let tel";
//         lustre_parser_new(input);
//     }
// }
