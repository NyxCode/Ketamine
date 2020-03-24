use crate::error::{Error, Severity};
use crate::impl_into_enum;
use crate::statements::Statement;
use crate::values::{read_code_block, Value};
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct While {
    pub condition: Box<Value>,
    pub body: Vec<Statement>,
}

impl_into_enum!(While, Value);
impl_into_enum!(While, Statement);

impl Parse for While {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::WhileKeyword).map_err(Severity::Recoverable)?;
        let condition = Value::read(pos, tokens).map_err(Severity::Fatal)?;
        let body = read_code_block(pos, tokens).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: keyword.start,
            end: body.end,
            value: While {
                condition: Box::new(condition.value),
                body: body.value,
            },
        })
    }
}
