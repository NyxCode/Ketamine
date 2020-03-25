mod assignment;
mod terminated;
mod unterminated;

use crate::error::{ParserResult, Severity, Error};
use crate::values::{For, If, Loop, While};
use crate::{first_value_of, impl_into_enum};
use crate::{Parse, Parsed};
pub use assignment::*;
use lexer::Token;
pub use terminated::*;
pub use unterminated::*;

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    If(If),
    For(For),
    While(While),
    Loop(Loop),
    TerminatedStatement(TerminatedStatement),
    UnterminatedStatement(UnterminatedStatement),
}

impl_into_enum!(TerminatedStatement => Statement:TerminatedStatement);
impl_into_enum!(UnterminatedStatement => Statement:UnterminatedStatement);

impl Parse for Statement {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        first_value_of!(
            Statements:
            TerminatedStatement,
            If,
            For,
            While,
            Loop,
            Assignment,
            UnterminatedStatement,
        );

        Statements::read(pos, tokens).map(|parsed| parsed.map(Into::into))
    }
}

impl Statement {
    pub fn read_all<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Vec<Parsed<Statement>>, Error<'a>> {
        let mut statements = vec![];
        while !tokens.is_empty() {
            statements.push(Statement::read(pos, tokens).map_err(Severity::into_inner)?);
        }
        Ok(statements)
    }
}
