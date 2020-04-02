use crate::ast::{parse_delimited_block, CodeBlock, Statement, AST};
use crate::error::{ParseResult, ResultExt, Severity};
use crate::token_ext::TokenExt;
use crate::{find_closing_delimiter, impl_into_enum, Parse, Pos, Token};
use lexer::TokenValue;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct If {
    pub if_branch: Pos<IfBranch>,
    pub else_if_branches: Vec<Pos<IfBranch>>,
    pub else_branch: Option<Vec<Pos<Statement>>>,
}
impl_into_enum!(If => AST:If);

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct IfBranch {
    pub condition: Pos<Box<AST>>,
    pub body: CodeBlock,
}

impl Parse for IfBranch {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens
            .pop_expect(pos, &TokenValue::IfKeyword)
            .into_recoverable()?;
        let par_open = tokens.pop_expect(keyword.end, &TokenValue::ParenthesesOpen)?;
        let par_close_idx = find_closing_delimiter(
            par_open.end,
            tokens,
            &TokenValue::ParenthesesOpen,
            &TokenValue::ParenthesesClose,
            1,
        )
        .into_fatal()?;
        let mut condition_tokens = &tokens[..par_close_idx];
        let condition = AST::parse(par_open.end, &mut condition_tokens).into_fatal()?;
        assert!(condition_tokens.is_empty());
        *tokens = &tokens[par_close_idx..];
        let par_close = tokens.pop_expect(condition.end, &TokenValue::ParenthesesClose)?;

        let body = parse_delimited_block(par_close.end, tokens).into_fatal()?;

        Ok(Pos {
            start: keyword.start,
            end: body.end,
            value: IfBranch {
                condition: condition.map(Box::new),
                body: body.value,
            },
        })
    }
}

impl Parse for If {
    fn parse<'a>(mut pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let if_branch = IfBranch::parse(pos, tokens)?;
        let mut else_if_branches = vec![];

        loop {
            let mut else_if_tokens = *tokens;
            match parse_else_if(pos, &mut else_if_tokens) {
                Ok(else_if) => {
                    pos = else_if.end;
                    else_if_branches.push(else_if);
                    *tokens = else_if_tokens;
                }
                Err(Pos {
                    value: Severity::Recoverable(..),
                    ..
                }) => break,
                Err(fatal) => return Err(fatal),
            }
        }

        let mut else_tokens = *tokens;
        let else_branch = match parse_else(pos, &mut else_tokens) {
            Ok(else_branch) => {
                pos = else_branch.end;
                *tokens = else_tokens;
                Some(else_branch.value)
            }
            Err(Pos {
                value: Severity::Recoverable(..),
                ..
            }) => None,
            Err(fatal) => return Err(fatal),
        };

        Ok(Pos {
            start: if_branch.start,
            end: pos,
            value: If {
                if_branch,
                else_if_branches,
                else_branch,
            },
        })
    }
}

fn parse_else_if<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, IfBranch> {
    let else_kw = tokens
        .pop_expect(pos, &TokenValue::ElseKeyword)
        .into_recoverable()?;
    let mut if_branch = IfBranch::parse(pos, tokens)?;
    if_branch.start = else_kw.start;
    Ok(if_branch)
}

fn parse_else<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, CodeBlock> {
    let else_kw = tokens
        .pop_expect(pos, &TokenValue::ElseKeyword)
        .into_recoverable()?;
    let mut body = parse_delimited_block(else_kw.end, tokens).into_fatal()?;
    body.start = else_kw.start;
    Ok(body)
}
