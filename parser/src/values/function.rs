use crate::error::{Error, Severity, AddErrorCtx};
use crate::statements::Statement;
use crate::values::{Identifier, read_code_block};
use crate::{find_closing_brace, pop, pop_expect, ErrorKind, Parse, Parsed, ParserResult};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<Parsed<Identifier>>,
    pub body: Vec<Parsed<Statement>>,
}

fn parse_parameters<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Vec<Parsed<Identifier>>, Error<'a>> {
    let par_open = pop_expect(pos, tokens, TokenValue::ParenthesesOpen)?;

    let mut parameters = vec![];
    let mut accept_comma = false;
    let mut accept_value = true;
    let mut accept_close = true;

    let par_close = loop {
        let token = pop(pos, tokens)?;
        match &token.value {
            TokenValue::ParenthesesClose if accept_close => {
                break token;
            }
            TokenValue::Identifier(ident) if accept_value => {
                let ident = Parsed {
                    start: token.start,
                    end: token.end,
                    value: Identifier(ident.to_owned())
                };
                parameters.push(ident);
                accept_value = false;
                accept_close = true;
                accept_comma = true;
            }
            TokenValue::Comma if accept_comma => {
                accept_value = true;
                accept_close = false;
                accept_comma = false;
            }
            other => {
                return Err(Error::range(
                    token.start,
                    token.end,
                    ErrorKind::UnexpectedToken(&other),
                ));
            }
        }
    };

    Ok(parameters)
}

impl Parse for Function {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let keyword = pop_expect(pos, tokens, TokenValue::FunctionKeyword)
            .map_err(Severity::Recoverable)?;
        let params = parse_parameters(keyword.end, tokens)
            .map_err(Severity::Fatal)
            .ctx("parsing function parameters")?;
        let body = read_code_block(pos, tokens)
            .map_err(Severity::Fatal)
            .ctx("parsing function body")?;

        let function = Function {
            parameters: params,
            body: body.value,
        };
        Ok(Parsed {
            start: keyword.start,
            end: body.end,
            value: function,
        })
    }
}
