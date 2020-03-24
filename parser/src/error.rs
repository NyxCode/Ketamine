use lexer::{LexingError, TokenValue};
use std::fmt::{Formatter, Display, Result as FmtResult};

#[derive(Debug)]
pub struct Error<'a> {
    pub start: usize,
    pub end: usize,
    pub kind: ErrorKind<'a>,
    pub context: ErrorContext<'a>,
}

#[derive(Debug)]
pub enum ErrorContext<'a> {
    None,
    One(&'a str),
    Two(&'a str, &'a str),
    Three(&'a str, &'a str, &'a str),
    More(&'a str, &'a str, &'a str),
}

#[derive(Debug)]
pub enum ErrorKind<'a> {
    LexingError,
    UnexpectedEOF,
    UnexpectedToken(&'a TokenValue),
    Unbalanced,
}

#[derive(Debug)]
pub enum Severity<'a> {
    Recoverable(Error<'a>),
    Fatal(Error<'a>),
}

pub trait AddErrorCtx<'a> {
    fn ctx(self, ctx: &'a str) -> Self;
}

impl<'a> AddErrorCtx<'a> for ErrorContext<'a> {
    fn ctx(self, ctx: &'a str) -> Self {
         match self {
            ErrorContext::None => ErrorContext::One(ctx),
            ErrorContext::One(one) => ErrorContext::Two(one, ctx),
            ErrorContext::Two(one, two) => ErrorContext::Three(one, two, ctx),
            ErrorContext::Three(one, two, three) |
            ErrorContext::More(one, two, three) => ErrorContext::More(two, three, ctx),
        }
    }
}

impl<'a> AddErrorCtx<'a> for Error<'a> {
    fn ctx(mut self, ctx: &'a str) -> Self {
        self.context = self.context.ctx(ctx);
        self
    }
}

impl<'a> AddErrorCtx<'a> for Severity<'a> {
    fn ctx(self, ctx: &'a str) -> Self {
        match self {
            Severity::Recoverable(err) => Severity::Recoverable(err.ctx(ctx)),
            Severity::Fatal(err) => Severity::Fatal(err.ctx(ctx)),
        }
    }
}

impl<'a, T, E> AddErrorCtx<'a> for Result<T, E> where E: AddErrorCtx<'a> {
    fn ctx(self, ctx: &'a str) -> Self {
        self.map_err(|err| err.ctx(ctx))
    }
}

impl <'a> Severity<'a> {
    pub fn into_inner(self) -> Error<'a> {
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

impl<'a> Error<'a> {
    pub fn position(pos: usize, kind: ErrorKind<'a>) -> Self {
        Error {
            start: pos,
            end: pos + 1,
            kind,
            context: ErrorContext::None,
        }
    }

    pub fn range(start: usize, end: usize, kind: ErrorKind<'a>) -> Self {
        Error { start, end, kind, context: ErrorContext::None }
    }
}

pub type ParserResult<'a, T> = Result<T, Error<'a>>;

impl<'a> From<LexingError> for Error<'a> {
    fn from(err: LexingError) -> Self {
        Error::position(err.location(), ErrorKind::LexingError)
    }
}

impl<'a> Display for ErrorKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ErrorKind::LexingError => write!(f, "unknown token"),
            ErrorKind::UnexpectedEOF => write!(f, "unexpected end of input"),
            ErrorKind::UnexpectedToken(token) => write!(f, "unexpected {}", token),
            ErrorKind::Unbalanced => write!(f, "unbalanced parenthesis"),
        }
    }
}

impl <'a> Display for ErrorContext<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ErrorContext::None => FmtResult::Ok(()),
            ErrorContext::One(one) => write!(f, "while {}", one),
            ErrorContext::Two(one, two) => write!(f, "while {} while {}", one, two),
            ErrorContext::Three(one, two, three) => write!(f, "while {} while {} while {}", one, two, three),
            ErrorContext::More(one, two, three) => write!(f, "while {} while {} while {} while ...", one, two, three),
        }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.kind, self.context)
    }
}
