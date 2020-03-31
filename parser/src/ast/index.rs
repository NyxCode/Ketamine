use crate::ast::AST;
use crate::{impl_into_enum, Pos};

#[derive(Debug, Clone)]
pub struct Index {
    pub value: Pos<Box<AST>>,
    pub index: Pos<Box<AST>>,
}
impl_into_enum!(Index => AST:Index);
