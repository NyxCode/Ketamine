use crate::ast::AST;
use crate::error::{Error, ParseResult, ResultExt};
use crate::token_ext::TokenExt;
use crate::{impl_into_enum, Parse, Token};
use lexer::{Pos, TokenValue};
use std::convert::TryFrom;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy)]
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

impl BinaryOperator {
    pub fn precedence(self) -> u8 {
        use BinaryOperator::*;
        match self {
            Eq | NotEq | GreaterThan | LessThan | GreaterEqThan | LessEqThan => 0,
            Add | Sub => 1,
            Mul | Div => 2,
        }
    }

    pub fn verb(self) -> (&'static str, &'static str) {
        match self {
            BinaryOperator::Add => ("add", "to"),
            BinaryOperator::Sub => ("subtract", "from"),
            BinaryOperator::Mul => ("multiply", "with"),
            BinaryOperator::Div => ("divide", "by"),
            _ => ("compare", "to"),
        }
    }
}

impl TryFrom<&TokenValue> for BinaryOperator {
    type Error = ();

    fn try_from(value: &TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Add => Ok(BinaryOperator::Add),
            TokenValue::Sub => Ok(BinaryOperator::Sub),
            TokenValue::Mul => Ok(BinaryOperator::Mul),
            TokenValue::Div => Ok(BinaryOperator::Div),
            TokenValue::Eq => Ok(BinaryOperator::Eq),
            TokenValue::NotEq => Ok(BinaryOperator::NotEq),
            TokenValue::GreaterThan => Ok(BinaryOperator::GreaterThan),
            TokenValue::LessThan => Ok(BinaryOperator::LessThan),
            TokenValue::GreaterEqThan => Ok(BinaryOperator::GreaterEqThan),
            TokenValue::LessEqThan => Ok(BinaryOperator::LessEqThan),
            _ => Err(()),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub lhs: Pos<Box<AST>>,
    pub op: Pos<BinaryOperator>,
    pub rhs: Pos<Box<AST>>,
}
impl_into_enum!(BinaryOperation => AST:BinaryOperation);

impl BinaryOperation {
    pub fn from_left_to_right(
        left: Pos<AST>,
        op: Pos<BinaryOperator>,
        right: Pos<AST>,
    ) -> Pos<BinaryOperation> {
        let start = left.start;
        let end = right.end;
        let value = match right.value {
            AST::BinaryOperation(BinaryOperation {
                lhs: lhs2,
                op: op2,
                rhs: rhs2,
            }) if op.value.precedence() > op2.value.precedence() => BinaryOperation {
                lhs: Pos {
                    start: left.start,
                    end: lhs2.end,
                    value: Box::new(AST::BinaryOperation(BinaryOperation {
                        lhs: left.map(Box::new),
                        op,
                        rhs: lhs2,
                    })),
                },
                op: op2,
                rhs: rhs2,
            },
            other => BinaryOperation {
                lhs: left.map(Box::new),
                op,
                rhs: Pos::new(right.start, right.end, Box::new(other)),
            },
        };
        Pos { start, end, value }
    }
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Minus,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct UnaryOperation {
    pub op: Pos<UnaryOperator>,
    pub value: Pos<Box<AST>>,
}
impl_into_enum!(UnaryOperation => AST:UnaryOperation);

impl Parse for UnaryOperation {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let op_token = tokens.pop(pos)?;
        let op = match &op_token.value {
            TokenValue::Negate => UnaryOperator::Negate,
            TokenValue::Sub => UnaryOperator::Minus,
            unexpected => {
                return Err(Pos {
                    start: op_token.start,
                    end: op_token.end,
                    value: Error::Unexpected {
                        unexpected,
                        expected: "unary operator",
                    }
                    .recoverable(),
                })
            }
        };
        let op = Pos::new(op_token.start, op_token.end, op);
        let value = AST::parse_atomic(op_token.end, tokens)
            .into_fatal()?
            .map(Box::new);
        Ok(Pos {
            start: op_token.start,
            end: value.end,
            value: UnaryOperation { op, value },
        })
    }
}
