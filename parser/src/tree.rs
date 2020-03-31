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
        tree.begin_child(format!("return"));
        if let Some(val) = &self.0 {
            val.value.deref().display(tree);
        }
        tree.end_child();
    }
}
impl TreeDisplay for Break {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("break"));
        if let Some(val) = &self.0 {
            val.value.deref().display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for Continue {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.add_empty_child(format!("continue"));
    }
}

impl TreeDisplay for Assignment {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("assignment"));
        tree.begin_child(format!("receiver"));
        self.receiver.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("value"));
        self.value.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for FieldAccess {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("field_access"));
        tree.begin_child(format!("value"));
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("field"));
        self.field.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Index {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("index"));
        tree.begin_child(format!("value"));
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("index"));
        self.index.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Function {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("function"));
        tree.begin_child(format!("args"));
        for arg in &self.params {
            arg.value.display(tree);
        }
        tree.end_child();
        tree.begin_child(format!("body"));
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for IfBranch {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("condition"));
        self.condition.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("body"));
        for body_statement in &self.body {
            body_statement.value.display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for If {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("if"));
        self.if_branch.value.display(tree);
        for else_if in &self.else_if_branches {
            tree.begin_child(format!("else_if"));
            else_if.value.display(tree);
            tree.end_child();
        }
        if let Some(else_) = &self.else_branch {
            tree.begin_child(format!("else"));
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
        tree.begin_child(format!("binary_operation"));
        tree.begin_child(format!("lhs"));
        self.lhs.value.display(tree);
        tree.end_child();
        tree.add_empty_child(format!("operator: {:?}", self.op.value));
        tree.begin_child(format!("rhs"));
        self.rhs.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for Parentheses {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("parentheses"));
        self.0.value.display(tree);
        tree.end_child();
    }
}

impl TreeDisplay for Call {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("call"));
        tree.begin_child(format!("value"));
        self.value.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("args"));
        for arg in &self.args {
            arg.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for List {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("list"));
        for element in &self.0 {
            element.value.display(tree);
        }
        tree.end_child();
    }
}

impl TreeDisplay for Object {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("object"));
        for (k, v) in &self.0 {
            tree.begin_child(format!("entry"));
            k.value.display(tree);
            v.value.display(tree);
            tree.end_child();
        }
        tree.end_child();
    }
}

impl TreeDisplay for Range {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("range"));
        tree.begin_child(format!("from"));
        self.from.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("to"));
        self.to.value.display(tree);
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for WhileLoop {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("while"));
        tree.begin_child(format!("condition"));
        self.condition.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("body"));
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for ForLoop {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("for"));
        tree.begin_child(format!("binding"));
        self.binding.value.display(tree);
        tree.end_child();
        tree.begin_child(format!("iterator"));
        self.iterator.value.display(tree);
        tree.end_child();

        tree.begin_child(format!("body"));
        for statement in &self.body {
            statement.value.display(tree);
        }
        tree.end_child();
        tree.end_child();
    }
}

impl TreeDisplay for UnaryOperation {
    fn display(&self, tree: &mut TreeBuilder) {
        tree.begin_child(format!("unary_operation"));
        tree.add_empty_child(format!("operator: {:?}", self.op.value));
        tree.begin_child(format!("value"));
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
                tree.begin_child(format!("unterminated"));
                ast.display(tree);
                tree.end_child();
            }
            Statement::Terminated(ast) => {
                tree.begin_child(format!("terminated"));
                ast.display(tree);
                tree.end_child();
            }
        }
    }
}
