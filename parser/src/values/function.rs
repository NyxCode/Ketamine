use crate::error::{Error, Severity};
use crate::statements::Statement;
use crate::values::Identifier;
use crate::{find_closing_brace, pop, pop_expect, ErrorKind, Parse, Parsed, ParserResult};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
}

struct FunctionParameters {
    start: usize,
    end: usize,
    parameters: Vec<Identifier>,
}

fn parse_parameters(pos: usize, tokens: &mut &[Token]) -> ParserResult<FunctionParameters> {
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
                parameters.push(Identifier(ident.to_owned()));
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
                    ErrorKind::UnexpectedToken(other.clone()),
                ));
            }
        }
    };

    Ok(FunctionParameters {
        start: par_open.start,
        end: par_close.end,
        parameters,
    })
}

impl Parse for Function {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::FunctionKeyword).map_err(Severity::Recoverable)?;
        let params = parse_parameters(keyword.end, tokens).map_err(Severity::Fatal)?;
        let brace_open =
            pop_expect(params.end, tokens, TokenValue::BraceOpen).map_err(Severity::Fatal)?;

        let brace_close_idx =
            find_closing_brace(brace_open.start, tokens).map_err(Severity::Fatal)?;
        let brace_close_pos = &tokens[brace_close_idx].end;

        let mut body_tokens = &tokens[..brace_close_idx];
        let body = Statement::read_all(&mut body_tokens).map_err(Severity::Fatal)?;
        *tokens = &tokens[(brace_close_idx + 1)..];

        let function = Function {
            parameters: params.parameters,
            body,
        };
        Ok(Parsed {
            start: keyword.start,
            end: *brace_close_pos,
            value: function,
        })
    }
}
