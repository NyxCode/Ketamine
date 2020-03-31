use crate::{Pos, TokenValue};
use std::fmt::{Display, Formatter, Result as FmtResult};

fn read_single_char(input: &str) -> Option<TokenValue> {
    if input.is_empty() {
        return None;
    }
    match &input[..1] {
        "+" => Some(TokenValue::Add),
        "-" => Some(TokenValue::Sub),
        "*" => Some(TokenValue::Mul),
        "/" => Some(TokenValue::Div),
        "=" => Some(TokenValue::Assign),
        "<" => Some(TokenValue::LessThan),
        ">" => Some(TokenValue::GreaterThan),
        "!" => Some(TokenValue::Negate),
        _ => None,
    }
}

fn read_two_chars(input: &str) -> Option<TokenValue> {
    if input.len() < 2 {
        return None;
    }
    match &input[..2] {
        ">=" => Some(TokenValue::GreaterEqThan),
        "<=" => Some(TokenValue::LessEqThan),
        "==" => Some(TokenValue::Eq),
        "!=" => Some(TokenValue::NotEq),
        _ => None,
    }
}

pub(crate) fn read_operator(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    let (op, len) = read_two_chars(input)
        .map(|op| (op, 2))
        .or_else(|| read_single_char(input).map(|op| (op, 1)))?;

    Some(Pos {
        start: offset,
        end: offset + len,
        value: op,
    })
}
