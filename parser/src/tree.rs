use crate::statements::Statement;
use crate::values::{
    BinaryOperation, Function, FunctionCall, Identifier, IfExpr, List, Object, Parentheses,
    UnaryOperation, Value,
};
use std::borrow::Borrow;
use std::fmt::{Display, Error, Formatter, Result as FmtResult};

fn print(level: usize, value: impl Display) {
    let inset = level * 3;
    println!("{}|- {}", " ".repeat(inset), value);
}

fn print_ident(level: usize, ident: &Identifier) {
    print(level, format!("identifier<{}>", ident.0));
}

fn print_function(level: usize, function: &Function) {
    print(level, "function");
    print(level + 1, "parameters");
    for parameter in function.parameters.iter() {
        print_ident(level + 2, parameter);
    }
    print(level + 1, "body");
    print_code(level + 2, &function.body[..])
}

fn print_call(level: usize, call: &FunctionCall) {
    print(level, "call");
    print(level + 1, "receiver");
    print_value(level + 2, &call.receiver);
    print(level + 1, "arguments");
    for argument in call.arguments.iter() {
        print_value(level + 2, argument);
    }
}

fn print_object(level: usize, obj: &Object) {
    print(level, "object");
    for (ident, value) in &obj.0 {
        print(level + 1, "entry");
        print_ident(level + 2, ident);
        print_value(level + 2, value);
    }
}

fn print_list(level: usize, list: &List) {
    print(level, "list");
    for entry in &list.0 {
        print_value(level + 1, entry);
    }
}

fn print_if(level: usize, if_expr: &IfExpr) {
    print(level, "if");
    print(level + 1, "condition");
    print_value(level + 2, &*if_expr.condition);
    print(level + 1, "body");
    print_code(level + 2, &if_expr.body[..]);
    for else_if in &if_expr.else_if_exprs {
        print(level + 1, "else_if");
        print(level + 2, "condition");
        print_value(level + 3, &*else_if.condition);
        print(level + 2, "body");
        print_code(level + 3, &if_expr.body[..]);
    }
    if let Some(else_expr) = &if_expr.else_expr {
        print(level + 1, "else");
        print_code(level + 2, else_expr);
    }
}

fn print_unary_op(level: usize, op: &UnaryOperation) {
    print(level, "unary_operation");
    print(level + 1, "operator");
    print(level + 2, format!("unary_operator<{:?}>", op.operator));
    print(level + 1, "operand");
    print_value(level + 2, &*op.operand);
}

fn print_binary_op(level: usize, op: &BinaryOperation) {
    print(level, "binary_operation");
    print(level + 1, "lhs");
    print_value(level + 2, &*op.lhs);
    print(level + 1, "operator");
    print(level + 2, format!("binary_operator<{:?}>", op.operator));
    print(level + 1, "rhs");
    print_value(level + 2, &*op.rhs);
}

fn print_parentheses(level: usize, par: &Parentheses) {
    print(level, "parentheses");
    print_value(level + 1, &*par.0)
}

fn print_statement(level: usize, statement: &Statement) {
    match statement {
        Statement::Assignment(assignment) => {
            print(level, "assignment");
            print(level + 1, "receiver");
            print_value(level + 2, &assignment.target);
            print(level + 1, "value");
            print_value(level + 2, &assignment.value)
        }
        Statement::Return(ret) => {
            print(level, "return");
            print_value(level + 1, &ret.0)
        }
        Statement::ValueStatement(value) => {
            print_value(level, &value.0);
        }
    }
}

pub fn print_value(level: usize, value: &Value) {
    match value {
        Value::Integer(int) => print(level, format!("int<{}>", int)),
        Value::Float(float) => print(level, format!("float<{}>", float)),
        Value::String(string) => print(level, format!("string<{}>", string)),
        Value::Identifier(ident) => print_ident(level, ident),
        Value::Nothing => print(level, "nothing<>"),
        Value::Function(function) => print_function(level, function),
        Value::FunctionCall(call) => print_call(level, call),
        Value::Object(object) => print_object(level, object),
        Value::List(list) => print_list(level, list),
        Value::Boolean(boolean) => print(level, format!("boolean<{}>", boolean)),
        Value::IfExpr(if_expr) => print_if(level, if_expr),
        Value::UnaryOperation(op) => print_unary_op(level, op),
        Value::BinaryOperation(op) => print_binary_op(level, op),
        Value::Parentheses(par) => print_parentheses(level, par),
    }
}

pub fn print_code(level: usize, code: &[Statement]) {
    for statement in code {
        print_statement(level, statement);
    }
}
