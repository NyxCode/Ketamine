use crate::error::Severity;
use crate::{pop, Error, ErrorKind, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        match pop(pos, tokens).map_err(Severity::Recoverable)? {
            Token {
                start,
                end,
                value: TokenValue::Identifier(ident),
            } => {
                Ok(Parsed {
                    value: Identifier(ident.clone()),
                    start: *start,
                    end: *end,
                })
            },
            Token { start, end, value } => Err(Error::range(
                *start,
                *end,
                ErrorKind::UnexpectedToken(&value),
            ))
            .map_err(Severity::Recoverable),
        }
    }
}
