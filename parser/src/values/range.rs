use crate::error::{Error, Severity};

use crate::values::{Value, ParsedValue};
use crate::{first_value_of, impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Range {
    pub from: Box<ParsedValue>,
    pub to: Box<ParsedValue>,
}

impl_into_enum!(Range => Value:Range);

impl Parse for Range {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        first_value_of!(
            RangeBound: crate::values::BinaryOperation,
            crate::values::UnaryOperation,
            crate::values::Call,
            crate::values::Identifier,
            i64,
            f64,
            String,
        );
        let left = RangeBound::read(pos, tokens)?.map(Value::from);
        let _operator =
            pop_expect(left.end, tokens, TokenValue::Range).map_err(Severity::Recoverable)?;
        let right = RangeBound::read(pos, tokens)?.map(Value::from);
        Ok(Parsed {
            start: left.start,
            end: right.end,
            value: Range {
                from: Box::new(left),
                to: Box::new(right),
            },
        })
    }
}
