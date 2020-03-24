use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::Value;
use crate::{impl_into_enum, peek, pop, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Continue(pub Box<Value>);

impl_into_enum!(Continue, Value);

impl Parse for Continue {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::ContinueKeyword).map_err(Severity::Recoverable)?;
        let (value, end) = match peek(keyword.end, tokens) {
            Ok(Token {
                value: TokenValue::Semicolon,
                start,
                end,
            }) => (Value::Nothing, *end),
            Ok(Token {
                value: TokenValue::BraceClose,
                end,
                ..
            }) => (Value::Nothing, *end),
            Ok(Token {
                value: other,
                start,
                ..
            }) => {
                let value = Value::read(*start, tokens).map_err(Severity::Fatal)?;
                (value.value, value.end)
            }
            Err(err) => return Err(Severity::Fatal(err)),
        };
        Ok(Parsed {
            start: keyword.start,
            end,
            value: Continue(Box::new(value)),
        })
    }
}
