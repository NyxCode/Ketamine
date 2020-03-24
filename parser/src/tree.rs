use crate::statements::Statement;
use crate::values::{
    BinaryOperation, Break, Continue, ForExpr, Function, FunctionCall, Identifier, IfExpr, List,
    Loop, Object, Parentheses, Range, Return, UnaryOperation, Value, While,
};
use std::borrow::Borrow;
use std::fmt::{Display, Error, Formatter, Result as FmtResult};

fn print(level: usize, value: impl Display) {
    println!("{}|- {}", "|  ".repeat(level), value);
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
        print_code(level + 3, &else_if.body[..]);
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

fn print_for(level: usize, for_expr: &ForExpr) {
    print(level, "for");
    print(level + 1, "binding");
    print_ident(level + 2, &for_expr.binding);
    print(level + 1, "in");
    print_value(level + 2, &*for_expr.iterator);
    print(level + 1, "body");
    print_code(level + 2, &for_expr.body);
}

fn print_range(level: usize, range: &Range) {
    print(level, "range");
    print(level + 1, "from");
    print_value(level + 2, &range.from);
    print(level + 1, "to");
    print_value(level + 2, &range.to);
}

fn print_while(level: usize, while_loop: &While) {
    print(level, "while");
    print(level + 1, "condition");
    print_value(level + 2, &*while_loop.condition);
    print(level + 1, "body");
    print_code(level + 2, &while_loop.body);
}

fn print_loop(level: usize, loop_: &Loop) {
    print(level, "loop");
    print_code(level + 1, &loop_.0);
}

fn print_instruction(level: usize, name: &str, value: &Value) {
    print(level, name);
    match value {
        Value::Nothing => {}
        other => print_value(level + 1, other),
    }
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
        Statement::IfExpr(expr) => print_if(level, expr),
        Statement::ForExpr(expr) => print_for(level, expr),
        Statement::TerminatedStatement(value) => {
            print_value(level, &value.0);
        }
        Statement::UnterminatedStatement(value) => {
            print(level, "unterminated");
            print_value(level + 1, &value.0)
        }
        Statement::While(while_loop) => print_while(level, while_loop),
        Statement::Loop(loop_) => print_loop(level, loop_),
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
        Value::ForExpr(expr) => print_for(level, expr),
        Value::Range(range) => print_range(level, range),
        Value::While(while_loop) => print_while(level, while_loop),
        Value::Loop(loop_) => print_loop(level, loop_),
        Value::Break(Break(value)) => print_instruction(level, "break", value),
        Value::Return(Return(value)) => print_instruction(level, "return", value),
        Value::Continue(Continue(value)) => print_instruction(level, "continue", value),
    }
}

pub fn print_code(level: usize, code: &[Statement]) {
    for statement in code {
        print_statement(level, statement);
    }
}
