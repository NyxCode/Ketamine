use crate::error::{Error, Severity};
use crate::values::Value;
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct TerminatedStatement(pub Value);

impl Parse for TerminatedStatement {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let value = Value::read(pos, tokens)?;
        let _semicolon =
            pop_expect(value.end, tokens, TokenValue::Semicolon).map_err(Severity::Fatal)?;
        Ok(value.map(TerminatedStatement))
    }
}
