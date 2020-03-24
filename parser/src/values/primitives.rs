use crate::error::{Error, ErrorKind, Severity};

use crate::{pop, Parse, Parsed, impl_into_enum, values::Value};
use lexer::{Token, TokenValue};

impl Parse for bool {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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
                ErrorKind::UnexpectedToken(&token.value),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}

impl Parse for i64 {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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
                ErrorKind::UnexpectedToken(&token.value),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}

impl Parse for f64 {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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
                ErrorKind::UnexpectedToken(&token.value),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}
impl Parse for String {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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
                ErrorKind::UnexpectedToken(&token.value),
            ))
            .map_err(Severity::Recoverable)
        }
    }
}

impl_into_enum!(bool => Value:Boolean);
impl_into_enum!(i64 => Value:Integer);
impl_into_enum!(f64 => Value:Float);
impl_into_enum!(String => Value:String);