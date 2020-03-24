use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::first_value_of;
use crate::tree::print_value;
use crate::values::Value;
use crate::{pop, Parsed, ReadParse};
use lexer::{Operator, Token, TokenValue};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::TryFrom;

first_value_of!(
    OperandValues: crate::values::UnaryOperation,
    crate::values::Function,
    crate::values::IfExpr,
    crate::values::FunctionCall,
    crate::values::Object,
    crate::values::List,
    crate::values::Identifier,
    crate::values::Parentheses,
    String,
    bool,
    i64,
    f64,
);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    GreaterThan,
    LessThan,
    GreaterEqThan,
    LessEqThan,
}

impl ReadParse for BinaryOperator {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        let operator = match &token.value {
            TokenValue::Operator(Operator::Add) => BinaryOperator::Add,
            TokenValue::Operator(Operator::Sub) => BinaryOperator::Sub,
            TokenValue::Operator(Operator::Mul) => BinaryOperator::Mul,
            TokenValue::Operator(Operator::Div) => BinaryOperator::Div,
            TokenValue::Operator(Operator::Eq) => BinaryOperator::Eq,
            TokenValue::Operator(Operator::NotEq) => BinaryOperator::NotEq,
            TokenValue::Operator(Operator::GreaterEqThan) => BinaryOperator::GreaterEqThan,
            TokenValue::Operator(Operator::LessEqThan) => BinaryOperator::LessThan,
            TokenValue::Operator(Operator::GreaterThan) => BinaryOperator::GreaterThan,
            TokenValue::Operator(Operator::LessThan) => BinaryOperator::LessThan,
            other => {
                let kind = ErrorKind::UnexpectedToken(other.clone());
                let error = Error::range(token.start, token.end, kind);
                return Err(Severity::Recoverable(error));
            }
        };
        Ok(Parsed {
            start: token.start,
            end: token.end,
            value: operator,
        })
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Add => 1,
            BinaryOperator::Sub => 1,
            BinaryOperator::Mul => 2,
            BinaryOperator::Div => 2,
            BinaryOperator::Eq => 0,
            BinaryOperator::NotEq => 0,
            BinaryOperator::GreaterThan => 0,
            BinaryOperator::LessThan => 0,
            BinaryOperator::GreaterEqThan => 0,
            BinaryOperator::LessEqThan => 0,
        }
    }
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub lhs: Box<Value>,
    pub operator: BinaryOperator,
    pub rhs: Box<Value>,
}

impl ReadParse for BinaryOperation {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let first = {
            let lhs = OperandValues::read(pos, tokens)?.map(Into::into);
            let OperatorAndRHS(operator, rhs) = OperatorAndRHS::read(lhs.end, tokens)?.value;
            BinaryOperation {
                lhs: Box::new(lhs.value),
                operator,
                rhs: Box::new(rhs),
            }
        };

        let mut remaining = OperatorAndRHS::read_all(pos, tokens)?;
        let operation = recursive_build(first, &mut remaining.value);

        Ok(Parsed {
            start: pos,
            end: remaining.end,
            value: operation,
        })
    }
}

#[derive(Debug)]
struct OperatorAndRHS(BinaryOperator, Value);

impl ReadParse for OperatorAndRHS {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let operator = BinaryOperator::read(pos, tokens)?;
        let rhs = OperandValues::read(pos, tokens)?.map(Into::into);

        Ok(Parsed {
            start: operator.start,
            end: rhs.end,
            value: OperatorAndRHS(operator.value, rhs.value),
        })
    }
}

impl OperatorAndRHS {
    fn read_all(
        pos: usize,
        tokens: &mut &[Token],
    ) -> Result<Parsed<VecDeque<Self>>, Severity<Error>> {
        let mut result = VecDeque::new();
        let mut end = pos;
        while let Ok((parsed, remaining)) = OperatorAndRHS::try_read(pos, *tokens) {
            *tokens = remaining;
            end = parsed.end;
            result.push_back(parsed.value);
        }
        Ok(Parsed {
            start: pos,
            end,
            value: result,
        })
    }
}

fn recursive_build(left: BinaryOperation, other: &mut VecDeque<OperatorAndRHS>) -> BinaryOperation {
    match other.len() {
        0 => left,
        one_or_more => {
            let OperatorAndRHS(next_op, next_val) = other.pop_front().unwrap();
            if left.operator.precedence() > next_op.precedence() {
                let operation = BinaryOperation {
                    lhs: Box::new(Value::BinaryOperation(left)),
                    operator: next_op,
                    rhs: Box::new(next_val),
                };
                recursive_build(operation, other)
            } else {
                let right = BinaryOperation {
                    lhs: left.rhs,
                    operator: next_op,
                    rhs: Box::new(next_val),
                };
                let operation = BinaryOperation {
                    lhs: left.lhs,
                    operator: left.operator,
                    rhs: Box::new(Value::BinaryOperation(right)),
                };
                recursive_build(operation, other)
            }
        }
    }
}
