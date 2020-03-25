use crate::statements::Statement;
use crate::values::{BinaryOperation, Break, Continue, For, Function, Call, Identifier, If, List, Loop, Object, Parentheses, Range, Return, UnaryOperation, Value, While, FieldAccess};

use std::fmt::{Display};
use crate::Parsed;

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
        print_ident(level + 2, &parameter.value);
    }
    print(level + 1, "body");
    print_code(level + 2, &function.body[..])
}

fn print_call(level: usize, call: &Call) {
    print(level, "call");
    print(level + 1, "receiver");
    print_value(level + 2, &call.receiver.value);
    print(level + 1, "arguments");
    for argument in call.arguments.iter() {
        print_value(level + 2, &argument.value);
    }
}

fn print_object(level: usize, obj: &Object) {
    print(level, "object");
    for (ident, value) in &obj.0 {
        print(level + 1, "entry");
        print_ident(level + 2, &ident.value);
        print_value(level + 2, &value.value);
    }
}

fn print_list(level: usize, list: &List) {
    print(level, "list");
    for entry in &list.0 {
        print_value(level + 1, &entry.value);
    }
}

fn print_if(level: usize, if_expr: &If) {
    print(level, "if");
    print(level + 1, "condition");
    print_value(level + 2, &if_expr.condition.value);
    print(level + 1, "body");
    print_code(level + 2, &if_expr.body[..]);
    for else_if in &if_expr.else_if_exprs {
        print(level + 1, "else_if");
        print(level + 2, "condition");
        print_value(level + 3, &else_if.condition.value);
        print(level + 2, "body");
        print_code(level + 3, &else_if.body[..]);
    }
    if let Some(else_expr) = &if_expr.else_expr {
        print(level + 1, "else");
        print_code(level + 2, &else_expr);
    }
}

fn print_unary_op(level: usize, op: &UnaryOperation) {
    print(level, "unary_operation");
    print(level + 1, "operator");
    print(level + 2, format!("unary_operator<{:?}>", op.operator));
    print(level + 1, "operand");
    print_value(level + 2, &op.operand.value);
}

fn print_binary_op(level: usize, op: &BinaryOperation) {
    print(level, "binary_operation");
    print(level + 1, "lhs");
    print_value(level + 2, &op.lhs.value);
    print(level + 1, "operator");
    print(level + 2, format!("binary_operator<{:?}>", op.operator.value));
    print(level + 1, "rhs");
    print_value(level + 2, &op.rhs.value);
}

fn print_parentheses(level: usize, par: &Parentheses) {
    print(level, "parentheses");
    print_value(level + 1, &par.0.value)
}

fn print_for(level: usize, for_expr: &For) {
    print(level, "for");
    print(level + 1, "binding");
    print_ident(level + 2, &for_expr.binding);
    print(level + 1, "in");
    print_value(level + 2, &for_expr.iterator.value);
    print(level + 1, "body");
    print_code(level + 2, &for_expr.body);
}

fn print_range(level: usize, range: &Range) {
    print(level, "range");
    print(level + 1, "from");
    print_value(level + 2, &range.from.value);
    print(level + 1, "to");
    print_value(level + 2, &range.to.value);
}

fn print_while(level: usize, while_loop: &While) {
    print(level, "while");
    print(level + 1, "condition");
    print_value(level + 2, &while_loop.condition.value);
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

fn print_field_access(level: usize, statement: &FieldAccess) {
    print(level, "field_access");
    print(level + 1, "lhs");
    print_value(level + 2, &statement.receiver.value);
    let mut level = level + 1;
    for nexted in &statement.fields {
        print(level, "field");
        print_ident(level + 1, &nexted.value);
        level += 2;
    }

}

fn print_statement(level: usize, statement: &Statement) {
    match statement {
        Statement::Assignment(assignment) => {
            print(level, "assignment");
            print(level + 1, "receiver");
            print_value(level + 2, &assignment.target.value);
            print(level + 1, "value");
            print_value(level + 2, &assignment.value.value)
        }
        Statement::If(expr) => print_if(level, &expr),
        Statement::For(expr) => print_for(level, &expr),
        Statement::TerminatedStatement(value) => {
            print_value(level, &value.0);
        }
        Statement::UnterminatedStatement(value) => {
            print(level, "unterminated");
            print_value(level + 1, &value.0)
        }
        Statement::While(while_loop) => print_while(level, &while_loop),
        Statement::Loop(loop_) => print_loop(level, &loop_),
    }
}

pub fn print_value(level: usize, value: &Value) {
    match value {
        Value::Integer(int) => print(level, format!("int<{}>", int)),
        Value::Float(float) => print(level, format!("float<{}>", float)),
        Value::String(string) => print(level, format!("string<{}>", &string)),
        Value::Identifier(ident) => print_ident(level, &ident),
        Value::Nothing => print(level, "nothing<>"),
        Value::Function(function) => print_function(level, &function),
        Value::Call(call) => print_call(level, &call),
        Value::Object(object) => print_object(level, &object),
        Value::List(list) => print_list(level, &list),
        Value::Boolean(boolean) => print(level, format!("boolean<{}>", boolean)),
        Value::If(if_expr) => print_if(level, &if_expr),
        Value::UnaryOperation(op) => print_unary_op(level, &op),
        Value::BinaryOperation(op) => print_binary_op(level, &op),
        Value::Parentheses(par) => print_parentheses(level, &par),
        Value::For(expr) => print_for(level, &expr),
        Value::Range(range) => print_range(level, &range),
        Value::While(while_loop) => print_while(level, &while_loop),
        Value::Loop(loop_) => print_loop(level, &loop_),
        Value::Break(value) => print_instruction(level, "break", &value.0.value),
        Value::Return(value) => print_instruction(level, "return", &value.0.value),
        Value::Continue(value) => print_instruction(level, "continue", &value.0.value),
        Value::FieldAccess(access) => print_field_access(level, access)
    }
}

pub fn print_code(level: usize, code: &[Parsed<Statement>]) {
    for statement in code {
        print_statement(level, &statement.value);
    }
}
