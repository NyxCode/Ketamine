use crate::ast::AST;
use crate::{impl_into_enum, Pos};

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Call {
    pub value: Pos<Box<AST>>,
    pub args: Vec<Pos<AST>>,
}

impl_into_enum!(Call => AST:Call);
