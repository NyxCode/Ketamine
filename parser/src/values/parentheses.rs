use crate::error::{Error, Severity};
use crate::impl_into_value;
use crate::values::Value;
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Parentheses(pub Box<Value>);

impl Parse for Parentheses {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let par_open =
            pop_expect(pos, tokens, TokenValue::ParenthesesOpen).map_err(Severity::Recoverable)?;
        let value = Value::read(pos, tokens).map_err(Severity::Fatal)?;
        let par_close =
            pop_expect(pos, tokens, TokenValue::ParenthesesClose).map_err(Severity::Fatal)?;
        Ok(Parsed {
            start: par_open.start,
            end: par_close.end,
            value: Parentheses(Box::new(value.value)),
        })
    }
}

impl_into_value!(Parentheses);
