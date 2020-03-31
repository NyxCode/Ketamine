use crate::ast::{Ident, AST};
use crate::{impl_into_enum, Pos};

#[derive(Debug, Clone)]
pub struct Call {
    pub value: Pos<Box<AST>>,
    pub args: Vec<Pos<AST>>,
}

impl_into_enum!(Call => AST:Call);
