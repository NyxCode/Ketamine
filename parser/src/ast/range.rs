use crate::ast::AST;
use crate::error::{ParseResult, Severity};
use crate::impl_into_enum;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug, Clone)]
pub struct Range {
    pub from: Pos<Box<AST>>,
    pub to: Pos<Box<AST>>,
}
impl_into_enum!(Range => AST:Range);
