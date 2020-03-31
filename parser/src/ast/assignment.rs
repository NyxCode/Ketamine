use crate::ast::AST;
use crate::impl_into_enum;
use crate::Pos;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub receiver: Pos<Box<AST>>,
    pub value: Pos<Box<AST>>,
}
impl_into_enum!(Assignment => AST:Assignment);
