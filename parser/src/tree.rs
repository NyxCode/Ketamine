use crate::ast::{
    Assignment, BinaryOperation, Break, Call, Continue, FieldAccess, ForLoop, Function, Ident, If,
    IfBranch, Index, List, Object, Parentheses, Range, Return, Statement, UnaryOperation,
    WhileLoop, AST,
};
use ptree::TreeBuilder;
use std::ops::Deref;

pub trait TreeDisplay {
    fn display(&self, tree: &mut TreeBuilder);
}

impl TreeDisplay for Ident {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.add_empty_child(format!("identifier: {}", &self.0));
    }
}

impl TreeDisplay for Return {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("return".to_owned());
        if let Some(val) = &self.0 {
            val.value.deref().display(tree);
        }
        tree.end_child();
    }
}
impl TreeDisplay for Break {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("break".to_owned());
        if let Some(val) = &self.0 {
            val.value.deref().display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for Continue {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.add_empty_child("continue".to_owned());
    }
}

impl TreeDisplay for Assignment {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("assignment".to_owned());
        tree.begin_child("receiver".to_owned());
        self.receiver.value.display(tree);
        tree.end_child();
        tree.begin_child("value".to_owned());
        self.value.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for FieldAccess {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("field_access".to_owned());
        tree.begin_child("value".to_owned());
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child("field".to_owned());
        self.field.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Index {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("index".to_owned());
        tree.begin_child("value".to_owned());
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child("index".to_owned());
        self.index.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Function {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("function".to_owned());
        tree.begin_child("args".to_owned());
        for arg in &self.params {
            arg.value.display(tree);
        }
        tree.end_child();
        tree.begin_child("body".to_owned());
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for IfBranch {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("condition".to_owned());
        self.condition.value.display(tree);
        tree.end_child();
        tree.begin_child("body".to_owned());
        for body_statement in &self.body {
            body_statement.value.display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for If {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("if".to_owned());
        self.if_branch.value.display(tree);
        for else_if in &self.else_if_branches {
            tree.begin_child("else_if".to_owned());
            else_if.value.display(tree);
            tree.end_child();
        }
        if let Some(else_) = &self.else_branch {
            tree.begin_child("else".to_owned());
            for statement in else_ {
                statement.value.display(tree);
            }
            tree.end_child();
        }
        tree.end_child();
    }
}

impl TreeDisplay for BinaryOperation {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("binary_operation".to_owned());
        tree.begin_child("lhs".to_owned());
        self.lhs.value.display(tree);
        tree.end_child();
        tree.add_empty_child(format!("operator: {:?}", self.op.value));
        tree.begin_child("rhs".to_owned());
        self.rhs.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Parentheses {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("parentheses".to_owned());
        self.0.value.display(tree);
        tree.end_child();
    }
}

impl TreeDisplay for Call {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("call".to_owned());
        tree.begin_child("value".to_owned());
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child("args".to_owned());
        for arg in &self.args {
            arg.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for List {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("list".to_owned());
        for element in &self.0 {
            element.value.display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for Object {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("object".to_owned());
        for (k, v) in &self.0 {
            tree.begin_child("entry".to_owned());
            k.value.display(tree);
            v.value.display(tree);
            tree.end_child();
        }
        tree.end_child();
    }
}

impl TreeDisplay for Range {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("range".to_owned());
        tree.begin_child("from".to_owned());
        self.from.value.display(tree);
        tree.end_child();
        tree.begin_child("to".to_owned());
        self.to.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for WhileLoop {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("while".to_owned());
        tree.begin_child("condition".to_owned());
        self.condition.value.display(tree);
        tree.end_child();
        tree.begin_child("body".to_owned());
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for ForLoop {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("for".to_owned());
        tree.begin_child("binding".to_owned());
        self.binding.value.display(tree);
        tree.end_child();
        tree.begin_child("iterator".to_owned());
        self.iterator.value.display(tree);
        tree.end_child();

        tree.begin_child("body".to_owned());
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for UnaryOperation {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child("unary_operation".to_owned());
        tree.add_empty_child(format!("operator: {:?}", self.op.value));
        tree.begin_child("value".to_owned());
        self.value.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for AST {
    fn display(&self, tree: &mut TreeBuilder) {
        match self {
            AST::Int(int) => {
                tree.add_empty_child(format!("integer: {}", int));
            }
            AST::Float(float) => {
                tree.add_empty_child(format!("float: {}", float));
            }
            AST::Bool(boolean) => {
                tree.add_empty_child(format!("boolean: {}", boolean));
            }
            AST::String(string) => {
                tree.add_empty_child(format!("string: {:?}", string));
            }
            AST::Ident(ident) => ident.display(tree),
            AST::Return(ret) => ret.display(tree),
            AST::Break(break_) => break_.display(tree),
            AST::Continue(cont) => cont.display(tree),
            AST::Assignment(assignment) => assignment.display(tree),
            AST::FieldAccess(access) => access.display(tree),
            AST::Index(index) => index.display(tree),
            AST::Function(function) => function.display(tree),
            AST::If(if_expr) => if_expr.display(tree),
            AST::BinaryOperation(op) => op.display(tree),
            AST::UnaryOperation(op) => op.display(tree),
            AST::Parentheses(par) => par.display(tree),
            AST::Call(call) => call.display(tree),
            AST::List(list) => list.display(tree),
            AST::Object(obj) => obj.display(tree),
            AST::Range(range) => range.display(tree),
            AST::ForLoop(for_loop) => for_loop.display(tree),
            AST::WhileLoop(while_loop) => while_loop.display(tree),
        };
    }
}

impl TreeDisplay for Statement {
    fn display(&self, tree: &mut TreeBuilder) {
        match self {
            Statement::Unterminated(ast) => {
                tree.begin_child("unterminated".to_owned());
                ast.display(tree);
                tree.end_child();
            }
            Statement::Terminated(ast) => {
                tree.begin_child("terminated".to_owned());
                ast.display(tree);
                tree.end_child();
            }
        }
    }
}
