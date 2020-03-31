use crate::ast::{Ident, AST};
use crate::error::{ParseResult, ResultExt, Severity};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{parse_list, Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct List(pub Vec<Pos<AST>>);
impl_into_enum!(List => AST:List);

#[derive(Debug, Clone)]
pub struct Object(pub Vec<(Pos<Ident>, Pos<AST>)>);
impl_into_enum!(Object => AST:Object);

impl Parse for List {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let list = parse_list(
            pos,
            tokens,
            TokenValue::BracketOpen,
            TokenValue::BracketClose,
            TokenValue::Comma,
        )?;
        Ok(Pos {
            start: list.start,
            end: list.end,
            value: List(list.value),
        })
    }
}

struct KVPair(Pos<Ident>, Pos<AST>);

impl Parse for KVPair {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let ident = Ident::parse(pos, tokens)?;
        let colon = tokens
            .pop_expect(ident.end, &TokenValue::Colon)
            .into_fatal()?;
        let value = AST::parse(colon.end, tokens).into_fatal()?;
        Ok(Pos::new(ident.start, value.end, KVPair(ident, value)))
    }
}

impl Parse for Object {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let pairs: Pos<Vec<Pos<KVPair>>> = parse_list(
            pos,
            tokens,
            TokenValue::BraceOpen,
            TokenValue::BraceClose,
            TokenValue::Comma,
        )?;
        Ok(Pos {
            start: pairs.start,
            end: pairs.end,
            value: Object(
                pairs
                    .value
                    .into_iter()
                    .map(|x| (x.value.0, x.value.1))
                    .collect(),
            ),
        })
    }
}
