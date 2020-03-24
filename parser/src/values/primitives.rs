use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::values::Value;
use crate::{pop, Parsed, ReadParse};
use lexer::{Token, TokenValue};

impl ReadParse for bool {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        if let TokenValue::Boolean(boolean) = &token.value {
            Ok(Parsed {
                start: token.start,
                end: token.end,
                value: *boolean,
            })
        } else {
            Err(Error::range(
                token.start,
                token.end,
                ErrorKind::UnexpectedToken(token.value.clone()),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}

impl ReadParse for i64 {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        if let TokenValue::Integer(int) = &token.value {
            Ok(Parsed {
                start: token.start,
                end: token.end,
                value: *int,
            })
        } else {
            Err(Error::range(
                token.start,
                token.end,
                ErrorKind::UnexpectedToken(token.value.clone()),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}

impl ReadParse for f64 {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        if let TokenValue::Float(float) = &token.value {
            Ok(Parsed {
                start: token.start,
                end: token.end,
                value: *float,
            })
        } else {
            Err(Error::range(
                token.start,
                token.end,
                ErrorKind::UnexpectedToken(token.value.clone()),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}
impl ReadParse for String {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let token = pop(pos, tokens).map_err(Severity::Recoverable)?;
        if let TokenValue::String(string) = &token.value {
            Ok(Parsed {
                start: token.start,
                end: token.end,
                value: string.to_owned(),
            })
        } else {
            Err(Error::range(
                token.start,
                token.end,
                ErrorKind::UnexpectedToken(token.value.clone()),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}
