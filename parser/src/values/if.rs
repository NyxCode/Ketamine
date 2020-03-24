use crate::error::{Error, Severity, AddErrorCtx};
use crate::statements::Statement;
use crate::values::{read_code_block, Value, ParsedValue};
use crate::{pop_expect, Parse, Parsed, impl_into_enum};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct If {
    pub condition: Box<Parsed<Value>>,
    pub body: Vec<Parsed<Statement>>,
    pub else_if_exprs: Vec<ElseIf>,
    pub else_expr: Option<Vec<Parsed<Statement>>>,
}

impl_into_enum!(If => Value:If);
impl_into_enum!(If => Statement:If);

#[derive(Debug)]
pub struct ElseIf {
    pub condition: Box<Parsed<Value>>,
    pub body: Vec<Parsed<Statement>>,
}

impl Parse for If {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let (condition, body) = If::read_main_branch(pos, tokens)
            .ctx("parsing 'if'")?;
        let else_ifs = read_else_ifs(body.end, tokens)?;
        let mut try_read_else = *tokens;
        let else_expr = match read_else(else_ifs.end, &mut try_read_else) {
            Ok(parsed) => {
                *tokens = try_read_else;
                Some(parsed)
            }
            Err(Severity::Recoverable(..)) => None,
            Err(fatal @ Severity::Fatal(..)) => return Err(fatal),
        };

        let end = else_expr
            .as_ref()
            .map(|expr| expr.end)
            .unwrap_or(else_ifs.end);

        Ok(Parsed {
            start: condition.start,
            end,
            value: If {
                condition: Box::new(condition),
                body: body.value,
                else_if_exprs: else_ifs.value,
                else_expr: else_expr.map(|expr| expr.value),
            },
        })
    }
}

impl If {
    fn read_main_branch<'a>(
        pos: usize,
        tokens: &mut &'a [Token],
    ) -> Result<(Parsed<Value>, Parsed<Vec<Parsed<Statement>>>), Severity<'a>> {
        let keyword = pop_expect(pos, tokens, TokenValue::IfKeyword)
            .map_err(Severity::Recoverable)
            .ctx("parsing 'if'-keyword")?;
        let condition = Value::read(pos, tokens)
            .map_err(Severity::into_fatal)
            .ctx("parsing 'if'-condition")?;
        let body = read_code_block(condition.start, tokens)
            .map_err(Severity::Fatal)
            .ctx("parsing 'if'-body")?;
        Ok((condition, body))
    }
}

fn read_else_ifs<'a>(
    pos: usize,
    tokens: &mut &'a [Token],
) -> Result<Parsed<Vec<ElseIf>>, Severity<'a>> {
    let mut end = pos;
    let mut else_ifs = vec![];

    loop {
        let mut try_tokens = *tokens;
        match read_else_if(end, &mut try_tokens) {
            Ok(parsed) => {
                end = parsed.end;
                *tokens = try_tokens;
                else_ifs.push(parsed.value);
            }
            Err(err @ Severity::Fatal(..)) => return Err(err),
            Err(Severity::Recoverable(..)) => break,
        }
    }

    Ok(Parsed {
        start: pos,
        end,
        value: else_ifs,
    })
}

fn read_else_if<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<ElseIf>, Severity<'a>> {
    let else_keyword =
        pop_expect(pos, tokens, TokenValue::ElseKeyword).map_err(Severity::Recoverable)?;

    let (condition, body) = If::read_main_branch(pos, tokens)?;
    Ok(Parsed {
        start: else_keyword.start,
        end: body.end,
        value: ElseIf {
            condition: Box::new(condition),
            body: body.value,
        },
    })
}

fn read_else<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Vec<Parsed<Statement>>>, Severity<'a>> {
    let else_keyword =
        pop_expect(pos, tokens, TokenValue::ElseKeyword).map_err(Severity::Recoverable)?;

    let body = read_code_block(else_keyword.end, tokens).map_err(Severity::Fatal)?;

    Ok(Parsed {
        start: else_keyword.start,
        end: body.end,
        value: body.value,
    })
}
