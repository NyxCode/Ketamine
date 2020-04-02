use crate::ast::AST;

use crate::impl_into_enum;
use crate::Pos;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Range {
    pub from: Pos<Box<AST>>,
    pub to: Pos<Box<AST>>,
}
impl_into_enum!(Range => AST:Range);
