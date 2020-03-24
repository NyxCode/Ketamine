use lexer::{LexingError, TokenValue};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    pub start: usize,
    pub end: usize,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    LexingError,
    UnexpectedEOF,
    UnexpectedToken(TokenValue),
    Unbalanced,
}

#[derive(Debug)]
pub enum Severity<E> {
    Recoverable(E),
    Fatal(E),
}

impl<E> Severity<E> {
    pub fn into_inner(self) -> E {
        match self {
            Severity::Recoverable(err) => err,
            Severity::Fatal(err) => err,
        }
    }

    pub fn into_recoverable(self) -> Self {
        Severity::Recoverable(self.into_inner())
    }

    pub fn into_fatal(self) -> Self {
        Severity::Fatal(self.into_inner())
    }
}

impl Error {
    pub fn position(pos: usize, kind: ErrorKind) -> Self {
        Error {
            start: pos,
            end: pos + 1,
            kind,
        }
    }

    pub fn range(start: usize, end: usize, kind: ErrorKind) -> Self {
        Error { start, end, kind }
    }
}

pub type ParserResult<T> = Result<T, Error>;

impl From<LexingError> for Error {
    fn from(err: LexingError) -> Self {
        Error::position(err.location(), ErrorKind::LexingError)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::LexingError => write!(f, "unknown token"),
            ErrorKind::UnexpectedEOF => write!(f, "unexpected end of input"),
            ErrorKind::UnexpectedToken(token) => write!(f, "unexpected {}", token),
            ErrorKind::Unbalanced => write!(f, "unbalanced parenthesis"),
        }
    }
}
