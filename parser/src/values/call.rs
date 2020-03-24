use crate::error::{Error, Severity};
use crate::first_value_of;
use crate::values::{Value, ParsedValue};
use crate::{peek, pop, pop_expect, ErrorKind, Identifier, Parse, Parsed, ParserResult, impl_into_enum};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Call {
    pub receiver: Box<ParsedValue>,
    pub arguments: Vec<ParsedValue>,
}

impl_into_enum!(Call => Value:Call);

impl Parse for Call {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        first_value_of!(Receiver: crate::values::Parentheses, Identifier,);

        let receiver = Receiver::read(pos, tokens)?;

        let args = read_arguments(receiver.end, tokens)
            .map_err(Severity::Recoverable)?;

        Ok(Parsed {
            start: receiver.start,
            end: args.end,
            value: Call {
                receiver: Box::new(receiver.map(Value::from)),
                arguments: args.value,
            },
        })
    }
}

fn read_arguments<'a>(
    pos: usize,
    tokens: &mut &'a [Token]
) -> Result<Parsed<Vec<Parsed<Value>>>, Error<'a>> {
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
                let arg = Value::read(pos, tokens)
                    .map_err(Severity::into_inner)?;
                args.push(arg);
            }
            other => {
                return Err(Error::range(
                    next_token.start,
                    next_token.end,
                    ErrorKind::UnexpectedToken(&other),
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
