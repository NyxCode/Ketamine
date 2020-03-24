use crate::error::{Error, ErrorKind, Severity};
use crate::values::{Value, ParsedValue};
use crate::{pop, Parse, Parsed, impl_into_enum};
use lexer::{Operator, Token, TokenValue};

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
    Minus,
}

impl Parse for UnaryOperator {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let token = pop(pos, tokens)
            .map_err(Severity::Recoverable)?;
        let op = match &token.value {
            TokenValue::Operator(Operator::Negate) => UnaryOperator::Negate,
            TokenValue::Operator(Operator::Sub) => UnaryOperator::Minus,
            other => {
                let kind = ErrorKind::UnexpectedToken(&other);
                let err = Error::position(token.start, kind);
                return Err(Severity::Recoverable(err));
            }
        };
        Ok(Parsed {
            start: token.start,
            end: token.end,
            value: op
        })
    }
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: Parsed<UnaryOperator>,
    pub operand: Box<Parsed<Value>>,
}

impl_into_enum!(UnaryOperation => Value:UnaryOperation);

impl Parse for UnaryOperation {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let operator = UnaryOperator::read(pos, tokens)?;
        let operand = Value::read(operator.end, tokens)
            .map_err(Severity::into_fatal)?;

        Ok(Parsed {
            start: operator.start,
            end: operand.end,
            value: UnaryOperation {
                operator,
                operand: Box::new(operand),
            },
        })
    }
}
