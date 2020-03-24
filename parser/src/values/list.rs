use crate::error::{Error, ErrorKind, Severity};
use crate::values::{Value, ParsedValue};
use crate::{peek, pop, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct List(pub Vec<Parsed<Value>>);

impl Parse for List {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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
                _other => {
                    let value = Value::read(pos, tokens).map_err(Severity::into_fatal)?;
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
                    let kind = ErrorKind::UnexpectedToken(&other.value);
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
