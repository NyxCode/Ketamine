use crate::ast::{parse_delimited_block, CodeBlock, Ident, Statement, AST};
use crate::error::{ParseResult, ResultExt};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{find_closing_delimiter, parse_list, Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Pos<Ident>>,
    pub body: CodeBlock,
}
impl_into_enum!(Function => AST:Function);

impl Parse for Function {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens
            .pop_expect(pos, &TokenValue::FunctionKeyword)
            .into_recoverable()?;
        let args = parse_list(
            keyword.end,
            tokens,
            TokenValue::ParenthesesOpen,
            TokenValue::ParenthesesClose,
            TokenValue::Comma,
        )
        .into_fatal()?;

        let body = parse_delimited_block(args.end, tokens).into_fatal()?;

        Ok(Pos {
            start: keyword.start,
            end: body.end,
            value: Function {
                params: args.value,
                body: body.value,
            },
        })
    }
}
