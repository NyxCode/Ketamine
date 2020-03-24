#![feature(try_trait)]

use crate::error::{Error, ErrorKind, ParserResult, Severity};


use crate::values::{Identifier, Value};
use lexer::{Token, TokenValue, tokenize};
use std::fmt::Debug;
use crate::statements::Statement;
use std::time::Instant;


mod error;
mod macros;
mod statements;
mod tree;
mod values;

#[test]
fn test() {
    let input = r#"
        fib = function(n) {
            if n < 3 {
                1
            } else {
                fib(n - 2) + fib(n - 1)
            }
        };
    "#;
    let start = Instant::now();
    let tokens = match tokenize(input) {
        Ok(tokens) => tokens,
        Err(err) => {
            report::report(input, err.location(), err.location(), err);
            return;
        }
    };
    let mut tokens = &tokens[..];
    let parsed =  Statement::read_all(tokens[0].start, &mut tokens);
    println!("Parsing took {}ms", Instant::now().duration_since(start).as_millis());
    match parsed {
        Err(err) => report::report(input, err.start, err.end, err),
        Ok(x) => {
            println!("{:?}", x);
            //assert_eq!(&format!("{:?}", x), r#"[Assignment(Assignment { target: Identifier(Identifier("x")), value: FunctionCall(FunctionCall { receiver: Identifier(Identifier("print")), arguments: [BinaryOperation(BinaryOperation { lhs: Integer(1), operator: Add, rhs: BinaryOperation(BinaryOperation { lhs: Integer(1), operator: Mul, rhs: Integer(1) }) })] }) })]"#);
            tree::print_code(0, &x[..]);
        }
    };
}

#[derive(Debug)]
pub struct Parsed<T: Debug> {
    pub value: T,
    pub start: usize,
    pub end: usize,
}

impl<T: Debug> Parsed<T> {
    fn map<O: Debug>(self, map: impl FnOnce(T) -> O) -> Parsed<O> {
        Parsed {
            start: self.start,
            end: self.end,
            value: map(self.value),
        }
    }
}

fn peek<'a>(pos: usize, tokens: &[Token]) -> Result<&Token, Error<'a>> {
    let token = tokens
        .get(0)
        .ok_or_else(|| Error::position(pos, ErrorKind::UnexpectedEOF))?;
    Ok(token)
}

fn pop<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<&'a Token, Error<'a>> {
    let token = peek(pos, tokens)?;
    *tokens = &tokens[1..];
    Ok(token)
}

fn pop_expect<'a>(
    pos: usize,
    tokens: &mut &'a [Token],
    expect: TokenValue,
) -> Result<&'a Token, Error<'a>> {
    let token = pop(pos, tokens)?;
    token_expect(token, &expect)?;
    Ok(token)
}

fn token_expect<'a>(token: &'a Token, expect: &TokenValue) -> Result<(), Error<'a>> {
    if &token.value != expect {
        Err(Error::range(
            token.start,
            token.end,
            ErrorKind::UnexpectedToken(&token.value),
        ))
    } else {
        Ok(())
    }
}

fn find_closing_brace(pos: usize, tokens: &[Token]) -> Result<usize, Error> {
    let mut count = 1;
    for (idx, token) in tokens.iter().enumerate() {
        match token.value {
            TokenValue::BraceOpen => count += 1,
            TokenValue::BraceClose => count -= 1,
            _ => (),
        };
        if count == 0 {
            return Ok(idx);
        }
    }

    Err(Error::position(pos, ErrorKind::Unbalanced))
}

trait Parse: Sized + Debug {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>>;
    fn try_read(pos: usize, tokens: &[Token]) -> Result<(Parsed<Self>, &[Token]), Severity> {
        let mut tokens = tokens;
        let parsed = Self::read(pos, &mut tokens)?;
        Ok((parsed, tokens))
    }
}
