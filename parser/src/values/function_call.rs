use crate::error::{Error, Severity};
use crate::first_value_of;
use crate::values::Value;
use crate::{peek, pop, pop_expect, ErrorKind, Identifier, Parse, Parsed, ParserResult};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct FunctionCall {
    pub receiver: Box<Value>,
    pub arguments: Vec<Value>,
}

impl Parse for FunctionCall {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        first_value_of!(Receiver: crate::values::Parentheses, Identifier,);

        let (receiver, rest) = Receiver::try_read(pos, *tokens)?;
        *tokens = rest;

        let args = read_arguments(receiver.end, tokens).map_err(Severity::Recoverable)?;

        Ok(Parsed {
            value: FunctionCall {
                receiver: Box::new(receiver.value.into()),
                arguments: args.value,
            },
            start: receiver.start,
            end: args.end,
        })
    }
}

fn read_arguments(pos: usize, tokens: &mut &[Token]) -> ParserResult<Parsed<Vec<Value>>> {
    let par_open = pop_expect(pos, tokens, TokenValue::ParenthesesOpen)?;

    let mut args = vec![];
    let mut accept_close = true;
    let mut accept_arg = true;
    let mut accept_comma = false;

    let par_close = loop {
        let next_token = peek(pos, tokens)?;
        match &next_token.value {
            TokenValue::ParenthesesClose if accept_close => {
                break pop(pos, tokens)?;
            }
            TokenValue::Comma if accept_comma => {
                pop(pos, tokens)?;
                accept_comma = false;
                accept_close = false;
                accept_arg = true;
            }
            _other if accept_arg => {
                accept_comma = true;
                accept_close = true;
                accept_arg = false;
                let arg = Value::read(pos, tokens)?;
                args.push(arg.value);
            }
            other => {
                return Err(Error::range(
                    next_token.start,
                    next_token.end,
                    ErrorKind::UnexpectedToken(other.clone()),
                ));
            }
        }
    };

    Ok(Parsed {
        value: args,
        start: par_open.start,
        end: par_close.end,
    })
}
