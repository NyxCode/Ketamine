use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::values::{Identifier, Value, ParsedValue};
use crate::{peek, pop, pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Object(pub Vec<(Parsed<Identifier>, Parsed<Value>)>);

impl Parse for Object {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
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

fn read_pairs<'a>(
    pos: usize,
    tokens: &mut &'a [Token]
) -> Result<Parsed<Vec<(Parsed<Identifier>, Parsed<Value>)>>, Error<'a>> {
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
                let kind = ErrorKind::UnexpectedToken(&other);
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

fn read_pair<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<(Parsed<Identifier>, Parsed<Value>)>, Error<'a>> {
    let ident_token = pop(pos, tokens)?;
    let ident = if let TokenValue::Identifier(ident) = &ident_token.value {
        Parsed { start: ident_token.start, end: ident_token.end, value: Identifier(ident.clone()) }
    } else {
        let kind = ErrorKind::UnexpectedToken(&ident_token.value);
        return Err(Error::range(ident_token.start, ident_token.end, kind));
    };
    let _colon = pop_expect(ident_token.end, tokens, TokenValue::Colon)?;

    let value = Value::read(pos, tokens).map_err(Severity::into_inner)?;

    Ok(Parsed {
        start: ident_token.start,
        end: value.end,
        value: (ident, value),
    })
}
