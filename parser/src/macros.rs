use crate::error::{Error, Severity};
use crate::values::Value;
use crate::{Parsed, ReadParse};
use lexer::Token;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) enum ParseEither<A, B>
where
    A: ReadParse + Debug,
    B: ReadParse + Debug,
{
    A(A),
    B(B),
}

impl<A, B> From<ParseEither<A, B>> for Value
where
    A: Into<Value> + ReadParse + Debug,
    B: Into<Value> + ReadParse + Debug,
{
    fn from(either: ParseEither<A, B>) -> Self {
        match either {
            ParseEither::A(a) => a.into(),
            ParseEither::B(b) => b.into(),
        }
    }
}

impl<A, B> ReadParse for ParseEither<A, B>
where
    A: ReadParse + Debug,
    B: ReadParse + Debug,
{
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        match A::try_read(pos, *tokens) {
            Ok((parsed, rest)) => {
                *tokens = rest;
                return Ok(parsed.map(ParseEither::A));
            }
            Err(err @ Severity::Fatal(..)) => return Err(err),
            _ => (),
        }
        match B::try_read(pos, *tokens) {
            Ok((parsed, rest)) => {
                *tokens = rest;
                Ok(parsed.map(ParseEither::B))
            }
            Err(err) => Err(err),
        }
    }
}

#[macro_export]
macro_rules! first_value_of {
    ($t:ident : $($rem:ty),+ $(,)?) => {
        use crate::macros::ParseEither;
        type $t = first_value_of!(@gen $($rem,)*);
    };
    (@gen $a:ty, $($rem:ty,)+) => {
        ParseEither<$a, first_value_of!(@gen $($rem,)*)>
    };
    (@gen $a:ty,) => {
        $a
    };
}

#[macro_export]
macro_rules! impl_into_value {
    ($t:ident) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Self {
                Value::$t(v)
            }
        }
    };
    ($from:ty, $variant:ident) => {
        impl From<$from> for Value {
            fn from(v: $from) -> Self {
                Value::$variant(v)
            }
        }
    };
}
