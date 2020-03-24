use crate::{Token, TokenValue};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Eq,
    NotEq,
    GreaterThan,
    LessThan,
    GreaterEqThan,
    LessEqThan,
    Negate,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Operator {
    fn read_single_char(input: &str) -> Option<Operator> {
        if input.is_empty() {
            return None;
        }
        match &input[..1] {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "=" => Some(Operator::Assign),
            "<" => Some(Operator::LessThan),
            ">" => Some(Operator::GreaterThan),
            "!" => Some(Operator::Negate),
            _ => None,
        }
    }

    fn read_two_chars(input: &str) -> Option<Operator> {
        if input.len() < 2 {
            return None;
        }
        match &input[..2] {
            ">=" => Some(Operator::GreaterEqThan),
            "<=" => Some(Operator::LessEqThan),
            "==" => Some(Operator::Eq),
            "!=" => Some(Operator::NotEq),
            _ => None,
        }
    }

    pub(crate) fn read_greedy(offset: usize, input: &str) -> Option<Token> {
        let (op, len) = Operator::read_two_chars(input)
            .map(|op| (op, 2))
            .or_else(|| Operator::read_single_char(input).map(|op| (op, 1)))?;

        Some(Token {
            start: offset,
            end: offset + len,
            value: TokenValue::Operator(op),
        })
    }
}
