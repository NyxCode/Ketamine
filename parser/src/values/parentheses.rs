use crate::error::{Error, Severity};
use crate::impl_into_enum;
use crate::values::{Value, ParsedValue};
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Parentheses(pub Box<Parsed<Value>>);

impl_into_enum!(Parentheses => Value:Parentheses);

impl Parse for Parentheses {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let par_open =
            pop_expect(pos, tokens, TokenValue::ParenthesesOpen).map_err(Severity::Recoverable)?;
        let value = Value::read(pos, tokens).map_err(Severity::into_fatal)?;
        let par_close =
            pop_expect(pos, tokens, TokenValue::ParenthesesClose).map_err(Severity::Fatal)?;
        Ok(Parsed {
            start: par_open.start,
            end: par_close.end,
            value: Parentheses(Box::new(value)),
        })
    }
}