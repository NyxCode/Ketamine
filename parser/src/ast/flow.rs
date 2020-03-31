use crate::ast::AST;
use crate::error::{ParseResult, ResultExt};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct Return(pub Option<Pos<Box<AST>>>);
impl_into_enum!(Return => AST:Return);

#[derive(Debug, Clone)]
pub struct Break(pub Option<Pos<Box<AST>>>);
impl_into_enum!(Break => AST:Break);

#[derive(Debug, Clone)]
pub struct Continue;
impl_into_enum!(Continue => AST:Continue);

impl Parse for Return {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens
            .pop_expect(pos, &TokenValue::ReturnKeyword)
            .into_recoverable()?;
        let next = match tokens.peek(keyword.end).ok() {
            None
            | Some(Pos {
                value: TokenValue::Semicolon,
                ..
            }) => return Ok(keyword.clone().map(|_| Return(None))),
            Some(next) => next,
        };

        let value = AST::parse(next.start, tokens)?;
        Ok(Pos::new(
            keyword.start,
            value.end,
            Return(Some(value.map(Box::new))),
        ))
    }
}

impl Parse for Break {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens
            .pop_expect(pos, &TokenValue::BreakKeyword)
            .into_recoverable()?;
        let next = match tokens.peek(keyword.end).ok() {
            None
            | Some(Pos {
                value: TokenValue::Semicolon,
                ..
            }) => return Ok(keyword.clone().map(|_| Break(None))),
            Some(next) => next,
        };

        let value = AST::parse(next.start, tokens)?;
        Ok(Pos::new(
            keyword.start,
            value.end,
            Break(Some(value.map(Box::new))),
        ))
    }
}

impl Parse for Continue {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let kw = tokens
            .pop_expect(pos, &TokenValue::ContinueKeyword)
            .into_recoverable()?;
        Ok(Pos::new(kw.start, kw.end, Continue))
    }
}
