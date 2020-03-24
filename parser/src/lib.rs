#![feature(try_trait)]

use crate::error::{Error, ErrorKind, ParserResult, Severity};

use crate::statements::Statement;
use crate::values::Identifier;
use lexer::{tokenize, LexingError, Token, TokenIterator, TokenValue};
use std::fmt::Debug;

mod error;
mod field_access;
mod macros;
mod statements;
mod tree;
mod values;

#[test]
fn test() {
    let input = r#"
        fib = function(n) {
            return if n < 3 {
                1
            } else {

            }
        };

    "#;

    let tokens = match tokenize(input) {
        Ok(tokens) => tokens,
        Err(err) => {
            report::report(input, err.location(), err.location(), err);
            return;
        }
    };
    let mut tokens = &tokens[..];
    println!("{:?}", tokens);
    match Statement::read_all(&mut tokens) {
        Err(err) => report::report(input, err.start, err.end, err.kind),
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

fn peek(pos: usize, tokens: &[Token]) -> ParserResult<&Token> {
    let token = tokens
        .get(0)
        .ok_or_else(|| Error::position(pos, ErrorKind::UnexpectedEOF))?;
    Ok(token)
}

fn peek_expect(pos: usize, tokens: &[Token], expect: TokenValue) -> ParserResult<&Token> {
    let token = peek(pos, tokens)?;
    token_expect(token, &expect)?;
    Ok(token)
}

fn pop<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParserResult<&'a Token> {
    let token = peek(pos, tokens)?;
    *tokens = &tokens[1..];
    Ok(token)
}

fn pop_expect<'a>(
    pos: usize,
    tokens: &mut &'a [Token],
    expect: TokenValue,
) -> ParserResult<&'a Token> {
    let token = pop(pos, tokens)?;
    token_expect(token, &expect)?;
    Ok(token)
}

fn token_expect(token: &Token, expect: &TokenValue) -> ParserResult<()> {
    if &token.value != expect {
        Err(Error::range(
            token.start,
            token.end,
            ErrorKind::UnexpectedToken(token.value.clone()),
        ))
    } else {
        Ok(())
    }
}

fn find_closing_brace(pos: usize, tokens: &[Token]) -> ParserResult<usize> {
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

trait ReadParse: Sized + Debug {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>>;
    fn try_read(pos: usize, tokens: &[Token]) -> Result<(Parsed<Self>, &[Token]), Severity<Error>> {
        let mut tokens = tokens;
        let parsed = Self::read(pos, &mut tokens)?;
        Ok((parsed, tokens))
    }
}
