use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::values::Value;
use crate::{peek, pop, pop_expect, Parsed, ReadParse};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct List(pub Vec<Value>);

impl ReadParse for List {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let open_bracket =
            pop_expect(pos, tokens, TokenValue::BracketOpen).map_err(Severity::Recoverable)?;
        let mut values = vec![];
        let close_bracket = loop {
            match &peek(pos, *tokens).map_err(Severity::Fatal)? {
                Token {
                    value: TokenValue::BracketClose,
                    end,
                    ..
                } => {
                    *tokens = &tokens[1..];
                    break *end;
                }
                other => {
                    let value = Value::read(pos, tokens).map_err(Severity::Fatal)?.value;
                    values.push(value);
                }
            }
            match pop(pos, tokens).map_err(Severity::Fatal)? {
                Token {
                    value: TokenValue::BracketClose,
                    end,
                    ..
                } => {
                    break *end;
                }
                Token {
                    value: TokenValue::Comma,
                    ..
                } => (),
                other => {
                    let kind = ErrorKind::UnexpectedToken(other.value.clone());
                    return Err(Error::range(other.start, other.end, kind))
                        .map_err(Severity::Fatal);
                }
            }
        };
        Ok(Parsed {
            start: open_bracket.start,
            end: close_bracket,
            value: List(values),
        })
    }
}
