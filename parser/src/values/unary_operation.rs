use crate::error::{Error, ErrorKind, Severity};
use crate::values::Value;
use crate::{pop, Parse, Parsed};
use lexer::{Operator, Token, TokenValue};

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
    Minus,
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Value>,
}

impl Parse for UnaryOperation {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let operator_token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        let operator = match &operator_token.value {
            TokenValue::Operator(Operator::Negate) => UnaryOperator::Negate,
            TokenValue::Operator(Operator::Sub) => UnaryOperator::Minus,
            other => {
                let kind = ErrorKind::UnexpectedToken(other.clone());
                let err = Error::position(operator_token.start, kind);
                return Err(Severity::Recoverable(err));
            }
        };
        let operand = Value::read(operator_token.end, tokens).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: operator_token.start,
            end: operand.end,
            value: UnaryOperation {
                operator,
                operand: Box::new(operand.value),
            },
        })
    }
}
