use crate::error::{Error, ErrorKind, Severity};
use crate::first_value_of;

use crate::values::{Value, ParsedValue};
use crate::{pop, Parse, Parsed};
use lexer::{Operator, Token, TokenValue};

use std::collections::VecDeque;


first_value_of!(
    OperandValues: crate::values::UnaryOperation,
    crate::values::Function,
    crate::values::If,
    crate::values::Call,
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

impl Parse for BinaryOperator {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        let operator = match &token.value {
            TokenValue::Operator(Operator::Add) => BinaryOperator::Add,
            TokenValue::Operator(Operator::Sub) => BinaryOperator::Sub,
            TokenValue::Operator(Operator::Mul) => BinaryOperator::Mul,
            TokenValue::Operator(Operator::Div) => BinaryOperator::Div,
            TokenValue::Operator(Operator::Eq) => BinaryOperator::Eq,
            TokenValue::Operator(Operator::NotEq) => BinaryOperator::NotEq,
            TokenValue::Operator(Operator::GreaterEqThan) => BinaryOperator::GreaterEqThan,
            TokenValue::Operator(Operator::LessEqThan) => BinaryOperator::LessEqThan,
            TokenValue::Operator(Operator::GreaterThan) => BinaryOperator::GreaterThan,
            TokenValue::Operator(Operator::LessThan) => BinaryOperator::LessThan,
            other => {
                let kind = ErrorKind::UnexpectedToken(&other);
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
    pub fn precedence(self) -> u8 {
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
    pub lhs: Box<Parsed<Value>>,
    pub operator: Parsed<BinaryOperator>,
    pub rhs: Box<Parsed<Value>>,
}

impl Parse for BinaryOperation {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let first = {
            let lhs = OperandValues::read(pos, tokens)?.map(Value::from);
            let OperatorAndRHS(operator, rhs) = OperatorAndRHS::read(lhs.end, tokens)?.value;
            Parsed {
                start: lhs.start,
                end: rhs.end,
                value: BinaryOperation {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                },
            }
        };
        let mut remaining = OperatorAndRHS::read_all(pos, tokens)?;
        let operation = recursive_build(first, &mut remaining.value);

        Ok(Parsed {
            start: pos,
            end: remaining.end,
            value: operation.value,
        })
    }
}

#[derive(Debug)]
struct OperatorAndRHS(Parsed<BinaryOperator>, Parsed<Value>);

impl Parse for OperatorAndRHS {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let operator = BinaryOperator::read(pos, tokens)?;
        let rhs = OperandValues::read(pos, tokens)?.map(Into::into);

        Ok(Parsed {
            start: operator.start,
            end: rhs.end,
            value: OperatorAndRHS(operator, rhs),
        })
    }
}

impl OperatorAndRHS {
    fn read_all<'a>(
        pos: usize,
        tokens: &mut &'a [Token],
    ) -> Result<Parsed<VecDeque<Self>>, Severity<'a>> {
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

fn recursive_build(left: Parsed<BinaryOperation>, other: &mut VecDeque<OperatorAndRHS>) -> Parsed<BinaryOperation> {
    match other.len() {
        0 => left,
        _one_or_more => {
            let OperatorAndRHS(next_op, next_val) = other.pop_front().unwrap();
            if left.value.operator.value.precedence() > next_op.value.precedence() {
                let parsed = Parsed {
                    start: left.start,
                    end: next_val.end,
                    value: BinaryOperation {
                        lhs: Box::new(left.map(Value::BinaryOperation)),
                        operator: next_op,
                        rhs: Box::new(next_val),
                    },
                };
                recursive_build(parsed, other)
            } else {
                let operation = Parsed {
                    start: left.start,
                    end: next_val.end,
                    value: BinaryOperation {
                        lhs: left.value.lhs,
                        operator: left.value.operator,
                        rhs: Box::new(Parsed {
                            start: left.value.rhs.start,
                            end: next_val.end,
                            value: BinaryOperation {
                                lhs: left.value.rhs,
                                operator: next_op,
                                rhs: Box::new(next_val),
                            },
                        }.map(Value::BinaryOperation)),
                    },
                };
                recursive_build(operation, other)
            }
        }
    }
}
