use crate::error::{Error, ParseResult};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::AST;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;
use std::ops::Deref;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Ident(pub String);
impl_into_enum!(Ident => AST:Ident);

impl Parse for Ident {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let token = tokens.pop(pos)?;
        match token.value {
            TokenValue::Identifier(ref ident) => Ok(Pos {
                start: token.start,
                end: token.end,
                value: Ident(ident.clone()),
            }),
            ref unexpected => Err(Pos {
                start: token.start,
                end: token.end,
                value: Error::Unexpected {
                    unexpected,
                    expected: "identifier",
                }
                .recoverable(),
            }),
        }
    }
}

impl Deref for Ident {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
