use std::fmt::{Debug, Display, Error, Formatter, Result as FmtResult};
use std::ops::Deref;

pub struct Pos<T> {
    pub start: usize,
    pub end: usize,
    pub value: T,
}

impl<T> Clone for Pos<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Pos {
            start: self.start,
            end: self.end,
            value: self.value.clone(),
        }
    }
}

impl<T> Debug for Pos<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Parsed({}..{}: {:?})", self.start, self.end, self.value)
    }
}

impl<T> PartialEq for Pos<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end && self.value == other.value
    }
}

impl<T> Pos<T> {
    pub fn new(start: usize, end: usize, value: T) -> Self {
        Pos { start, end, value }
    }

    pub fn map<O>(self, map: impl FnOnce(T) -> O) -> Pos<O> {
        Pos {
            start: self.start,
            end: self.end,
            value: map(self.value),
        }
    }

    pub fn as_ref(&self) -> Pos<&T> {
        Pos {
            start: self.start,
            end: self.end,
            value: &self.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),

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
    WhileKeyword,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name())
    }
}

impl TokenValue {
    pub fn name(&self) -> &'static str {
        match self {
            TokenValue::Integer(_) => "integer",
            TokenValue::Float(_) => "float",
            TokenValue::Boolean(_) => "boolean",
            TokenValue::String(_) => "string",
            TokenValue::Identifier(_) => "identifier",
            TokenValue::ParenthesesOpen => "(",
            TokenValue::ParenthesesClose => ")",
            TokenValue::BracketOpen => "[",
            TokenValue::BracketClose => "]",
            TokenValue::BraceOpen => "{",
            TokenValue::BraceClose => "}",
            TokenValue::Semicolon => ";",
            TokenValue::Colon => ":",
            TokenValue::Comma => ",",
            TokenValue::Dot => ".",
            TokenValue::Range => "..",
            TokenValue::FunctionKeyword => "function",
            TokenValue::ReturnKeyword => "return",
            TokenValue::BreakKeyword => "break",
            TokenValue::ContinueKeyword => "continue",
            TokenValue::IfKeyword => "if",
            TokenValue::ElseKeyword => "else",
            TokenValue::ForKeyword => "for",
            TokenValue::InKeyword => "in",
            TokenValue::WhileKeyword => "while",
            TokenValue::Add => "+",
            TokenValue::Sub => "-",
            TokenValue::Mul => "*",
            TokenValue::Div => "/",
            TokenValue::Assign => "=",
            TokenValue::Eq => "==",
            TokenValue::NotEq => "!=",
            TokenValue::GreaterThan => ">",
            TokenValue::LessThan => "<",
            TokenValue::GreaterEqThan => ">=",
            TokenValue::LessEqThan => "<=",
            TokenValue::Negate => "!",
        }
    }
}
