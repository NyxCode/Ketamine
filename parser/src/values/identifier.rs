use crate::error::Severity;
use crate::{pop, Error, ErrorKind, Parsed, ParserResult, ReadParse};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Identifier(pub String);

impl ReadParse for Identifier {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        match pop(pos, tokens).map_err(Severity::Recoverable)? {
            Token {
                start,
                end,
                value: TokenValue::Identifier(ident),
            } => Ok(Parsed {
                value: Identifier(ident.clone()),
                start: *start,
                end: *end,
            }),
            Token { start, end, value } => Err(Error::range(
                *start,
                *end,
                ErrorKind::UnexpectedToken(value.clone()),
            ))
            .map_err(Severity::Recoverable),
        }
    }
}
