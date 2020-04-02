use lexer::{Pos, TokenValue};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ParseResult<'a, T> = Result<Pos<T>, Pos<Severity<'a>>>;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug)]
pub enum Severity<'a> {
    Fatal(Error<'a>),
    Recoverable(Error<'a>),
}

impl<'a> Severity<'a> {
    pub fn into_inner(self) -> Error<'a> {
        match self {
            Severity::Fatal(err) => err,
            Severity::Recoverable(err) => err,
        }
    }

    pub fn into_fatal(self) -> Severity<'a> {
        Severity::Fatal(self.into_inner())
    }

    pub fn into_recoverable(self) -> Severity<'a> {
        Severity::Recoverable(self.into_inner())
    }
}

pub trait ResultExt {
    fn into_fatal(self) -> Self;
    fn into_recoverable(self) -> Self;
}

impl<'a, T> ResultExt for Result<T, Pos<Severity<'a>>> {
    fn into_fatal(self) -> Self {
        self.map_err(|err| err.map(Severity::into_fatal))
    }

    fn into_recoverable(self) -> Self {
        self.map_err(|err| err.map(Severity::into_recoverable))
    }
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug)]
pub enum Error<'a> {
    Missing(&'a str),
    Unexpected {
        unexpected: &'a TokenValue,
        expected: &'a str,
    },
}

impl<'a> Error<'a> {
    pub fn fatal(self) -> Severity<'a> {
        Severity::Fatal(self)
    }

    pub fn recoverable(self) -> Severity<'a> {
        Severity::Recoverable(self)
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Missing(missing) => write!(f, "missing {}", missing),
            Error::Unexpected {
                unexpected,
                expected,
            } => write!(f, "expected {}, got {}", expected, unexpected),
        }
    }
}
