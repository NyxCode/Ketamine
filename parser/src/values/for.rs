use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::{read_code_block, Identifier, Value, ParsedValue};
use crate::{impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct For {
    pub binding: Identifier,
    pub iterator: Box<Parsed<Value>>,
    pub body: Vec<Parsed<Statement>>,
}

impl_into_enum!(For => Value:For);
impl_into_enum!(For => Statement:For);

impl Parse for For {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let for_keyword =
            pop_expect(pos, tokens, TokenValue::ForKeyword).map_err(Severity::Recoverable)?;
        let binding = Identifier::read(pos, tokens).map_err(Severity::into_fatal)?;
        let in_keyword = pop_expect(pos, tokens, TokenValue::InKeyword).map_err(Severity::Fatal)?;
        let iterator = Value::read(pos, tokens).map_err(Severity::into_fatal)?;
        let body = read_code_block(in_keyword.end, tokens).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: for_keyword.start,
            end: body.end,
            value: For {
                binding: binding.value,
                iterator: Box::new(iterator),
                body: body.value,
            },
        })
    }
}
