use crate::error::{Error, ParserResult, Severity};
use crate::statements::{Statement, TerminatedStatement};
use crate::values::{read_code_block, Value};
use crate::{find_closing_brace, peek, peek_expect, pop, pop_expect, Parse, Parsed};
use lexer::{tokenize, Token, TokenValue};

#[derive(Debug)]
pub struct IfExpr {
    pub condition: Box<Value>,
    pub body: Vec<Statement>,
    pub else_if_exprs: Vec<ElseIf>,
    pub else_expr: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct ElseIf {
    pub condition: Box<Value>,
    pub body: Vec<Statement>,
}

impl Parse for IfExpr {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let main_branch = IfExpr::read_main_branch(pos, tokens)?;
        let (condition, body) = main_branch.value;
        let else_ifs = read_else_ifs(main_branch.end, tokens)?;
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
            start: main_branch.start,
            end,
            value: IfExpr {
                condition: Box::new(condition),
                body,
                else_if_exprs: else_ifs.value,
                else_expr: else_expr.map(|expr| expr.value),
            },
        })
    }
}

impl IfExpr {
    fn read_main_branch(
        pos: usize,
        tokens: &mut &[Token],
    ) -> Result<Parsed<(Value, Vec<Statement>)>, Severity<Error>> {
        let keyword =
            pop_expect(pos, tokens, TokenValue::IfKeyword).map_err(Severity::Recoverable)?;
        let condition = Value::read(pos, tokens).map_err(Severity::Fatal)?;
        let body = read_code_block(condition.start, tokens).map_err(Severity::Fatal)?;
        Ok(Parsed {
            start: keyword.start,
            end: body.end,
            value: (condition.value, body.value),
        })
    }
}

fn read_else_ifs(
    pos: usize,
    tokens: &mut &[Token],
) -> Result<Parsed<Vec<ElseIf>>, Severity<Error>> {
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

fn read_else_if(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<ElseIf>, Severity<Error>> {
    let else_keyword =
        pop_expect(pos, tokens, TokenValue::ElseKeyword).map_err(Severity::Recoverable)?;

    let branch = IfExpr::read_main_branch(pos, tokens)?;
    Ok(Parsed {
        start: else_keyword.start,
        end: branch.end,
        value: ElseIf {
            condition: Box::new(branch.value.0),
            body: branch.value.1,
        },
    })
}

fn read_else(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Vec<Statement>>, Severity<Error>> {
    let else_keyword =
        pop_expect(pos, tokens, TokenValue::ElseKeyword).map_err(Severity::Recoverable)?;

    let body = read_code_block(else_keyword.end, tokens).map_err(Severity::Fatal)?;

    Ok(Parsed {
        start: else_keyword.start,
        end: body.end,
        value: body.value,
    })
}
