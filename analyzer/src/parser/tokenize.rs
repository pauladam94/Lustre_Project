use crate::{
    ast::{ast::Ast, ast_types::AstTypes, node::Node},
    parser::span::Span,
};
use logos::{Lexer, Logos, Skip};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = FilePosition)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("(*", skip_comments)]
    Comments,
    // KEYWORDS
    #[token("node")]
    NodeKeyWord,
    #[token("returns")]
    Returns,
    #[token("let")]
    Let,
    #[token("tel")]
    Tel,
    #[token("fby")]
    Fby,
    #[token("pre")]
    Pre,
    #[token("->")]
    Arrow,
    // TYPE
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("char")]
    Char,
    #[token("bool")]
    Bool,

    // SYMBOLS
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token(".")]
    Period,
    #[token("^")]
    Carrot,

    // Operator
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
    #[regex("(_|[a-zA-Z])(_|[0-9]|[a-zA-Z])*", |lex| lex.slice().to_owned())]
    Identifier(String),
}

fn skip_comments(lexer: &mut Lexer<Token>) -> Skip {
    // lexer.fold(init, f)
    Skip {}
}

pub struct Tokens<'source> {
    lexer: Lexer<'source, Token>,
    index_buf: usize,
    buf: Vec<CompleteToken>,
}

#[derive(Clone, Debug, Default)]
pub struct FilePosition {
    line: u32,
    column: u32,
}
#[derive(Clone, Debug)]
pub struct CompleteToken {
    token: Token,
    fragment: String,
    pos: FilePosition,
}
#[derive(Debug)]
pub enum ParseError {
    Incomplete(String),
    Complete,
    Error,
}
impl<'source> Tokens<'source> {
    fn new(lexer: Lexer<'source, Token>) -> Self {
        Self {
            lexer,
            index_buf: 0,
            buf: vec![],
        }
    }
    fn peek(&mut self) -> Option<CompleteToken> {
        match self.next() {
            Some(t) => {
                self.buf.push(t.clone());
                Some(t)
            }
            None => None,
        }
    }
}
impl<'source> Iterator for Tokens<'source> {
    type Item = CompleteToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Ok(t)) => Some(Self::Item {
                token: t.clone(),
                fragment: String::from(self.lexer.slice()),
                pos: self.lexer.extras.clone(),
            }),
            Some(Err(())) => None,
            None => None,
        }
    }
}
impl<'source> DoubleEndedIterator for Tokens<'source> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            match self.buf.last() {
                Some(t) => Some(t.clone()),
                None => None,
            }
        }
    }
}

pub type Result<O, E = ParseError> = std::result::Result<O, E>;
impl Parser<CompleteToken> for Token {
    fn parse(self, input: &mut Tokens) -> Result<CompleteToken, ParseError> {
        match input.peek() {
            Some(t) => {
                if t.token == self {
                    let _ = input.next();
                    Ok(t)
                } else {
                    Err(ParseError::Error)
                }
            }
            None => Err(ParseError::Error),
        }
    }
}

impl<O, T, const N: usize> Parser<[O; N]> for [T; N]
where
    T: Parser<O>,
{
    fn parse(self, input: &mut Tokens) -> Result<[O; N], ParseError> {
        self.try_map(|parser| parser.parse(input))
    }
}

fn alt<O, const N: usize>(parsers: [impl Parser<O>; N]) -> impl Parser<O> {
    move |input: &mut Tokens| {
        for parser in parsers {
            match parser.parse(input) {
                Ok(o) => return Ok(o),
                Err(_) => {}
            }
        }
        Err(ParseError::Error)
    }
}

impl<T, O> Parser<O> for T
where
    T: FnOnce(&mut Tokens) -> Result<O, ParseError>,
{
    fn parse(self, input: &mut Tokens) -> Result<O, ParseError> {
        self(input)
    }
}

pub trait Parser<O> {
    fn parse(self, input: &mut Tokens) -> Result<O, ParseError>;
    fn map_parse<S, F>(self, f: F) -> impl Parser<S>
    where
        F: Fn(O) -> S,
        Self: Sized,
    {
        move |input: &mut Tokens| self.parse(input).map(|s| f(s))
    }
}

fn lustre_parser_new(input: &str) -> Result<Ast, ParseError> {
    let lexer = Token::lexer(input);
    let mut tokens = Tokens::new(lexer);
    ast_new(&mut tokens)
}
fn node_new(input: &mut Tokens) -> Result<Node, ParseError> {
    use Token::*;
    let toy_span = Span::new(0, 0, String::from("toy"));
    [
        NodeKeyWord, // node
        ParenOpen,   // (
        ParenClose,  // )
        Returns,     // returns
        ParenOpen,   // (
        ParenClose,  // )
        SemiColon,   // ;
        Let,         // let
        Tel,         // tel
    ]
    .map_parse(
        |[node, _, _, returns, _, _, semicolon, lettoken, telt]| Node {
            span_node: todo!(),
            span_returns: todo!(),
            span_let: todo!(),
            span_tel: todo!(),
            span_semicolon: todo!(),
            tag: todo!(),
            name: todo!(),
            inputs: todo!(),
            vars: todo!(),
            outputs: todo!(),
            let_bindings: todo!(),
            span_semicolon_equations: todo!(),
        },
    )
    .parse(input)
}
fn ast_new(input: &mut Tokens) -> Result<Ast, ParseError> {
    node_new
        .map_parse(|node| Ast {
            nodes: vec![node],
            types: AstTypes::new(),
        })
        .parse(input)
}

#[cfg(test)]
mod test {
    use crate::parser::tokenize::{Token, lustre_parser_new};
    use logos::Logos;

    #[test]
    fn test() {
        let input = "node f() returns (); let tel";
        match lustre_parser_new(input) {
            Ok(ast) => {}
            Err(e) => panic!("Error {:#?}", e),
        }
    }
}
