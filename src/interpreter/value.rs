use std::fmt::Display;

use crate::ast;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i32),
    Tuple(Box<Value>, Box<Value>),
    Boolean(bool),
    Function(ast::Function),
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Tuple(..) => "tuple",
            Value::String(..) => "string",
            Value::Boolean(..) => "boolean",
            Value::Integer(..) => "integer",
            Value::Function(..) => "function",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(value) => f.write_fmt(format_args!("{value}")),
            Value::Boolean(value) => f.write_fmt(format_args!("{value}")),
            Value::String(value) => f.write_fmt(format_args!("{value}")),
            Value::Tuple(first, second) => f.write_fmt(format_args!("({first}, {second})")),
            Value::Function(..) => f.write_fmt(format_args!("#function")),
        }
    }
}
