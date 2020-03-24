use crate::error::{Error, ParserResult, Severity};
use crate::values::Value;
use crate::{pop_expect, Parsed, ReadParse};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct ValueStatement(pub Value);

impl ReadParse for ValueStatement {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let value = Value::read(pos, tokens).map_err(Severity::Recoverable)?;
        let semicolon =
            pop_expect(value.end, tokens, TokenValue::Semicolon).map_err(Severity::Fatal)?;
        Ok(value.map(ValueStatement))
    }
}
