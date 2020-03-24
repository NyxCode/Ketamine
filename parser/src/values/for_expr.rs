use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::{read_code_block, Identifier, Value};
use crate::{impl_into_enum, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct ForExpr {
    pub binding: Identifier,
    pub iterator: Box<Value>,
    pub body: Vec<Statement>,
}

impl_into_enum!(ForExpr, Value);
impl_into_enum!(ForExpr, Statement);

impl Parse for ForExpr {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let for_keyword =
            pop_expect(pos, tokens, TokenValue::ForKeyword).map_err(Severity::Recoverable)?;
        let binding = Identifier::read(pos, tokens).map_err(Severity::into_fatal)?;
        let in_keyword = pop_expect(pos, tokens, TokenValue::InKeyword).map_err(Severity::Fatal)?;
        let iterator = Value::read(pos, tokens).map_err(Severity::Fatal)?;
        let body = read_code_block(in_keyword.end, tokens).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: for_keyword.start,
            end: body.end,
            value: ForExpr {
                binding: binding.value,
                iterator: Box::new(iterator.value),
                body: body.value,
            },
        })
    }
}
