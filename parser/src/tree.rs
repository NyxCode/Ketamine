use crate::{Statement, AST, Return, Assignment, FieldAccess, Ident, Index, Function, If, IfBranch};
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
        for arg in &self.args {
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
        self.if_branch.display(tree);
        for else_if in &self.else_if_branches {
            tree.begin_child(format!("else_if"));
            else_if.display(tree);
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

impl TreeDisplay for AST {
    fn display(&self, tree: &mut TreeBuilder) {
        match self {
            AST::Int(int) => {
                tree.add_empty_child(format!("integer: {}", int));
            },
            AST::Float(float) => {
                tree.add_empty_child(format!("float: {}", float));
            },
            AST::Bool(boolean) => {
                tree.add_empty_child(format!("boolean: {}", boolean));
            },
            AST::String(string) => {
                tree.add_empty_child(format!("string: {:?}", string));
            },
            AST::Ident(ident) => ident.display(tree),
            AST::Return(ret) => ret.display(tree),
            AST::Assignment(assignment) => assignment.display(tree),
            AST::FieldAccess(access) => access.display(tree),
            AST::Index(index) => index.display(tree),
            AST::Function(function) => function.display(tree),
            AST::If(if_expr) => if_expr.display(tree)
        };
    }
}

impl TreeDisplay for Statement {
    fn display(&self, tree: &mut TreeBuilder) {
        println!("lel");
        match self {
            Statement::Unterminated(ast) => {
                tree.begin_child(format!("unterminated"));
                ast.display(tree);
                tree.end_child();
            },
            Statement::Terminated(ast) => {
                tree.begin_child(format!("terminated"));
                ast.display(tree);
                tree.end_child();
            },
        }
    }
}