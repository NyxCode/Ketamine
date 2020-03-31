use crate::ast::AST;
use crate::error::{ParseResult, ResultExt};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{find_closing_delimiter, Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct Parentheses(pub Pos<Box<AST>>);

impl_into_enum!(Parentheses => AST:Parentheses);

impl Parse for Parentheses {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let par_open = tokens
            .pop_expect(pos, &TokenValue::ParenthesesOpen)
            .into_recoverable()?;
        let par_close_idx = find_closing_delimiter(
            par_open.end,
            tokens,
            &TokenValue::ParenthesesOpen,
            &TokenValue::ParenthesesClose,
            1,
        )
        .into_fatal()?;
        let mut value_tokens = &tokens[..par_close_idx];
        let value = AST::parse(par_open.end, &mut value_tokens).into_fatal()?;
        assert!(value_tokens.is_empty());
        *tokens = &tokens[par_close_idx..];
        let par_close = tokens
            .pop_expect(value.end, &TokenValue::ParenthesesClose)
            .into_fatal()?;

        Ok(Pos {
            start: par_open.start,
            end: par_close.end,
            value: Parentheses(value.map(Box::new)),
        })
    }
}
