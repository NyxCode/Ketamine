use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),
    Operator(Operator),

    ParenthesesOpen,
    ParenthesesClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    Semicolon,
    Colon,
    Comma,
    Dot,

    Range,

    FunctionKeyword,
    ReturnKeyword,
    BreakKeyword,
    ContinueKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    InKeyword,
    LoopKeyword,
    WhileKeyword,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn value(f: &mut Formatter<'_>, kind: &str, value: impl Display) -> FmtResult {
            write!(f, "{}<{}>", kind, value)
        }
        fn token(f: &mut Formatter<'_>, token: &str) -> FmtResult {
            write!(f, "{}", token)
        }

        match self {
            TokenValue::Integer(int) => value(f, "int", int),
            TokenValue::Float(float) => value(f, "float", float),
            TokenValue::String(string) => value(f, "string", string),
            TokenValue::Boolean(boolean) => value(f, "bool", boolean),
            TokenValue::Identifier(ident) => value(f, "identifier", ident),
            TokenValue::Operator(op) => value(f, "operator", op),
            TokenValue::ParenthesesOpen => token(f, "("),
            TokenValue::ParenthesesClose => token(f, ")"),
            TokenValue::BracketOpen => token(f, "["),
            TokenValue::BracketClose => token(f, "]"),
            TokenValue::BraceOpen => token(f, "{"),
            TokenValue::BraceClose => token(f, "}"),
            TokenValue::Semicolon => token(f, ";"),
            TokenValue::Comma => token(f, ","),
            TokenValue::Dot => token(f, "."),
            TokenValue::Colon => token(f, ":"),
            TokenValue::Range => token(f, ".."),
            TokenValue::FunctionKeyword => token(f, "function"),
            TokenValue::ReturnKeyword => token(f, "return"),
            TokenValue::BreakKeyword => token(f, "break"),
            TokenValue::ContinueKeyword => token(f, "continue"),
            TokenValue::ForKeyword => token(f, "for"),
            TokenValue::InKeyword => token(f, "in"),
            TokenValue::LoopKeyword => token(f, "loop"),
            TokenValue::WhileKeyword => token(f, "while"),
            TokenValue::IfKeyword => token(f, "if"),
            TokenValue::ElseKeyword => token(f, "else"),
        }
    }
}
