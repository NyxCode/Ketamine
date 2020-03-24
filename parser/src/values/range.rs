use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::Value;
use crate::{first_value_of, impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Range {
    pub from: Box<Value>,
    pub to: Box<Value>,
}

impl_into_enum!(Range, Value);

impl Parse for Range {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        first_value_of!(
            RangeBound: crate::values::BinaryOperation,
            crate::values::UnaryOperation,
            crate::values::FunctionCall,
            crate::values::Identifier,
            i64,
            f64,
            String,
        );
        let left = RangeBound::read(pos, tokens)?.map(Value::from);
        let operator =
            pop_expect(left.end, tokens, TokenValue::Range).map_err(Severity::Recoverable)?;
        let right = RangeBound::read(pos, tokens)?.map(Value::from);
        Ok(Parsed {
            start: left.start,
            end: right.end,
            value: Range {
                from: Box::new(left.value),
                to: Box::new(right.value),
            },
        })
    }
}
