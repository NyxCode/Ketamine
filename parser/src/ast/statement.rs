use crate::ast::AST;
use crate::error::ParseResult;
use crate::token_ext::TokenExt;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Statement {
    Unterminated(Box<AST>),
    Terminated(Box<AST>),
}

impl Statement {
    pub fn into_inner(self) -> Box<AST> {
        match self {
            Statement::Unterminated(inner) => inner,
            Statement::Terminated(inner) => inner,
        }
    }

    pub fn inner(&self) -> &AST {
        match self {
            Statement::Unterminated(inner) => inner,
            Statement::Terminated(inner) => inner,
        }
    }
}

pub type CodeBlock = Vec<Pos<Statement>>;

impl Parse for Statement {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut statement: Pos<AST> = crate::ast::AtomicValues::parse(pos, tokens)?.map(AST::from);

        loop {
            match tokens.peek(pos).ok() {
                Some(Pos {
                    value: TokenValue::Semicolon,
                    ..
                }) => {
                    tokens.pop_unwrap();
                    return Ok(statement.map(Box::new).map(Statement::Terminated));
                }
                None => {
                    return Ok(statement.map(Box::new).map(Statement::Unterminated));
                }
                Some(_next) => statement = AST::append(statement, tokens)?.map(Into::<AST>::into),
            };
        }
    }
}

impl Parse for Vec<Pos<Statement>> {
    fn parse<'a>(mut pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let start = pos;
        let mut statements = vec![];
        while !tokens.is_empty() {
            let statement = Statement::parse(pos, tokens)?;
            pos = statement.end;
            statements.push(statement);
        }
        Ok(Pos {
            start,
            end: pos,
            value: statements,
        })
    }
}
