use crate::error::{Error, Severity};
use crate::impl_into_enum;
use crate::statements::Statement;
use crate::values::{read_code_block, Value, ParsedValue};
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct While {
    pub condition: Box<Parsed<Value>>,
    pub body: Vec<Parsed<Statement>>,
}

impl_into_enum!(While => Value:While);
impl_into_enum!(While => Statement:While);

impl Parse for While {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::WhileKeyword).map_err(Severity::Recoverable)?;
        let condition = Value::read(pos, tokens).map_err(Severity::into_fatal)?;
        let body = read_code_block(pos, tokens).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: keyword.start,
            end: body.end,
            value: While {
                condition: Box::new(condition),
                body: body.value,
            },
        })
    }
}
