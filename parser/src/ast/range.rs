use crate::ast::AST;

use crate::impl_into_enum;
use crate::Pos;

#[derive(Debug, Clone)]
pub struct Range {
    pub from: Pos<Box<AST>>,
    pub to: Pos<Box<AST>>,
}
impl_into_enum!(Range => AST:Range);
