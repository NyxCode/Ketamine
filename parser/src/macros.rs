use crate::error::{ParseResult, Severity};
use crate::{ast::AST, Parse, Token, TryParse};
use lexer::Pos;
use std::fmt::{Debug, Formatter, Result as FmtResult};

pub enum ParseEither<A, B> {
    A(A),
    B(B),
}

impl<A, B> Debug for ParseEither<A, B>
where
    A: Debug,
    B: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseEither::A(a) => write!(f, "ParseEither::A({:?})", a),
            ParseEither::B(b) => write!(f, "ParseEither::B({:?})", b),
        }
    }
}

impl<A: Parse, B: Parse> Parse for ParseEither<A, B> {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        match A::try_parse(pos, tokens) {
            Ok(a) => Ok(a.map(ParseEither::A)),
            Err(
                err
                @
                Pos {
                    value: Severity::Fatal(..),
                    ..
                },
            ) => Err(err),
            Err(Pos {
                value: Severity::Recoverable(..),
                ..
            }) => match B::try_parse(pos, tokens) {
                Ok(b) => Ok(b.map(ParseEither::B)),
                Err(err) => Err(err),
            },
        }
    }
}

impl<A, B> From<ParseEither<A, B>> for AST
where
    A: Into<AST>,
    B: Into<AST>,
{
    fn from(either: ParseEither<A, B>) -> Self {
        match either {
            ParseEither::A(a) => a.into(),
            ParseEither::B(b) => b.into(),
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
macro_rules! impl_into_enum {
    ($t:ty => $enum:ty : $variant:ident) => {
        impl From<$t> for $enum {
            fn from(v: $t) -> Self {
                <$enum>::$variant(v)
            }
        }
    };
}
