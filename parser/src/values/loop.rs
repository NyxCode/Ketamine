use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::{read_code_block, Value, ParsedValue};
use crate::{impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Loop(pub Vec<Parsed<Statement>>);

impl_into_enum!(Loop => Value:Loop);
impl_into_enum!(Loop => Statement:Loop);

impl Parse for Loop {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::LoopKeyword).map_err(Severity::Recoverable)?;
        let body = read_code_block(keyword.end, tokens).map_err(Severity::Fatal)?;
        Ok(Parsed {
            start: keyword.start,
            end: body.end,
            value: Loop(body.value),
        })
    }
}
