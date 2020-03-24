use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::Value;
use crate::{pop_expect, Parsed, ParserResult, ReadParse};
use lexer::{Operator, Token, TokenValue};

#[derive(Debug)]
pub struct Assignment {
    pub target: Value,
    pub value: Value,
}

impl ReadParse for Assignment {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let lhs = Value::read(tokens[0].start, tokens).map_err(Severity::Recoverable)?;
        let assign = pop_expect(lhs.end, tokens, TokenValue::Operator(Operator::Assign))
            .map_err(Severity::Recoverable)?;
        let rhs = Value::read(assign.end, tokens).map_err(Severity::Fatal)?;
        let semicolon =
            pop_expect(rhs.end, tokens, TokenValue::Semicolon).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: lhs.start,
            end: semicolon.end,
            value: Assignment {
                target: lhs.value,
                value: rhs.value,
            },
        })
    }
}
