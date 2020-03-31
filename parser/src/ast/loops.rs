use crate::ast::{parse_delimited_block, CodeBlock, Ident, AST};
use crate::error::{ParseResult, ResultExt};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub binding: Pos<Ident>,
    pub iterator: Pos<Box<AST>>,
    pub body: CodeBlock,
}
impl_into_enum!(ForLoop => AST:ForLoop);

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Pos<Box<AST>>,
    pub body: CodeBlock,
}
impl_into_enum!(WhileLoop => AST:WhileLoop);

impl Parse for ForLoop {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let for_kw = tokens
            .pop_expect(pos, &TokenValue::ForKeyword)
            .into_recoverable()?;
        let par_open = tokens
            .pop_expect(for_kw.end, &TokenValue::ParenthesesOpen)
            .into_fatal()?;
        let binding = Ident::parse(par_open.end, tokens).into_fatal()?;
        let in_kw = tokens
            .pop_expect(binding.end, &TokenValue::InKeyword)
            .into_fatal()?;
        let iterator = AST::parse(in_kw.end, tokens).into_fatal()?;
        let par_close = tokens
            .pop_expect(iterator.end, &TokenValue::ParenthesesClose)
            .into_fatal()?;
        let body = parse_delimited_block(par_close.end, tokens).into_fatal()?;

        Ok(Pos {
            start: for_kw.start,
            end: body.end,
            value: ForLoop {
                binding,
                iterator: iterator.map(Box::new),
                body: body.value,
            },
        })
    }
}

impl Parse for WhileLoop {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let while_kw = tokens
            .pop_expect(pos, &TokenValue::WhileKeyword)
            .into_recoverable()?;
        let par_open = tokens
            .pop_expect(while_kw.end, &TokenValue::ParenthesesOpen)
            .into_fatal()?;
        let condition = AST::parse(par_open.end, tokens).into_fatal()?;
        let par_close = tokens
            .pop_expect(condition.end, &TokenValue::ParenthesesClose)
            .into_fatal()?;
        let body = parse_delimited_block(par_close.end, tokens).into_fatal()?;

        Ok(Pos {
            start: while_kw.start,
            end: body.end,
            value: WhileLoop {
                condition: condition.map(Box::new),
                body: body.value,
            },
        })
    }
}
