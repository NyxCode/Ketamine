use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::values::{Identifier, Value};
use crate::{peek, pop, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Object(pub Vec<(Identifier, Value)>);

impl Parse for Object {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let brace_open =
            pop_expect(pos, tokens, TokenValue::BraceOpen).map_err(Severity::Recoverable)?;
        let values = read_pairs(brace_open.end, tokens).map_err(Severity::Fatal)?;
        let brace_close =
            pop_expect(values.end, tokens, TokenValue::BraceClose).map_err(Severity::Fatal)?;

        Ok(Parsed {
            start: brace_open.start,
            end: brace_close.end,
            value: Object(values.value),
        })
    }
}

fn read_pairs(pos: usize, tokens: &mut &[Token]) -> ParserResult<Parsed<Vec<(Identifier, Value)>>> {
    let start = pos;
    let mut end = start;
    let mut values = vec![];

    loop {
        match &peek(end, tokens)?.value {
            TokenValue::Identifier(..) => {
                let pair = read_pair(end, tokens)?;
                end = pair.end;
                values.push(pair.value);
            }
            TokenValue::BraceClose => {
                break;
            }
            other => {
                let kind = ErrorKind::UnexpectedToken(other.clone());
                return Err(Error::position(pos, kind));
            }
        };
        if let TokenValue::BraceClose = peek(end, tokens)?.value {
            break;
        }
        pop_expect(end, tokens, TokenValue::Comma)?;
    }

    Ok(Parsed {
        start,
        end,
        value: values,
    })
}

fn read_pair(pos: usize, tokens: &mut &[Token]) -> ParserResult<Parsed<(Identifier, Value)>> {
    let ident_token = pop(pos, tokens)?;
    let ident = if let TokenValue::Identifier(ident) = &ident_token.value {
        Identifier(ident.clone())
    } else {
        let kind = ErrorKind::UnexpectedToken(ident_token.value.clone());
        return Err(Error::range(ident_token.start, ident_token.end, kind));
    };
    let colon = pop_expect(ident_token.end, tokens, TokenValue::Colon)?;

    let value = Value::read(pos, tokens)?;

    Ok(Parsed {
        value: (ident, value.value),
        start: ident_token.start,
        end: value.end,
    })
}
