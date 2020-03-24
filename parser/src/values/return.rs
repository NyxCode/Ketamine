use crate::error::{Error, Severity};
use crate::impl_into_enum;
use crate::values::Value;
use crate::{pop, pop_expect, ErrorKind, Parse, Parsed, ParserResult};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Return(pub Box<Value>);

impl_into_enum!(Return, Value);

impl Parse for Return {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::ReturnKeyword).map_err(Severity::Recoverable)?;

        let next = &tokens[0];
        match &next.value {
            TokenValue::Semicolon => {
                let semicolon = pop_expect(next.start, tokens, TokenValue::Semicolon)
                    .map_err(Severity::Fatal)?;
                Ok(Parsed {
                    start: keyword.start,
                    end: semicolon.end,
                    value: Return(Box::new(Value::Nothing)),
                })
            }
            _other => {
                let value = Value::read(next.start, tokens)
                    .map_err(Severity::Fatal)?
                    .map(Box::new)
                    .map(Return);
                let _semicolon = pop_expect(value.end, tokens, TokenValue::Semicolon)
                    .map_err(Severity::Fatal)?;
                Ok(value)
            }
        }
    }
}
