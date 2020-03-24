use crate::values::Value;
use crate::{Identifier, Parsed, ParserResult, impl_into_enum, Parse, pop_expect};
use lexer::{Token, TokenValue};
use crate::error::Severity;

#[derive(Debug)]
pub struct FieldAccess {
    pub receiver: Box<Parsed<Value>>,
    pub field: Parsed<Identifier>,
}

impl_into_enum!(FieldAccess => Value:FieldAccess);

impl Parse for FieldAccess {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let lhs = Value::read(pos, tokens)?;
        let dot = pop_expect(pos, tokens,TokenValue::Dot)
            .map_err(Severity::Recoverable)?;
        let rhs = Identifier::read(pos, tokens).map_err(Severity::into_fatal)?;
        Ok(Parsed {
            start: lhs.start,
            end: rhs.end,
            value: FieldAccess {
                receiver: Box::new(lhs),
                field: rhs
            }
        })
    }
}