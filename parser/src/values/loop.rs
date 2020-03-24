use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::{read_code_block, Value};
use crate::{impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Loop(pub Vec<Statement>);

impl_into_enum!(Loop, Value);
impl_into_enum!(Loop, Statement);

impl Parse for Loop {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
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
