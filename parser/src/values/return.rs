use crate::error::{Error, Severity};
use crate::{impl_into_enum, peek};
use crate::values::{Value, ParsedValue};
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Return(pub Box<Parsed<Value>>);

impl_into_enum!(Return => Value:Return);

impl Parse for Return {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let keyword = pop_expect(pos, tokens, TokenValue::ReturnKeyword)
            .map_err(Severity::Recoverable)?;
        let value = match peek(keyword.end, tokens) {
            Ok(token) if token.value == TokenValue::Semicolon || token.value == TokenValue::BraceClose => {
                Parsed { start: token.start, end: token.end, value: Value::Nothing }
            }
            Ok(Token { start, .. }) => {
                Value::read(*start, tokens).map_err(Severity::into_fatal)?
            }
            Err(err) => return Err(Severity::Fatal(err)),
        };
        Ok(Parsed {
            start: keyword.start,
            end: value.end,
            value: Return(Box::new(value)),
        })
    }
}
