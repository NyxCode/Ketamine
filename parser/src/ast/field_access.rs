use crate::ast::{Ident, AST};
use crate::impl_into_enum;
use crate::Pos;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct FieldAccess {
    pub value: Pos<Box<AST>>,
    pub field: Pos<Ident>,
}
impl_into_enum!(FieldAccess => AST:FieldAccess);
