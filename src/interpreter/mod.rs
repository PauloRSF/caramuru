use std::{collections::HashMap, error::Error, fmt::Display};

use crate::ast;

mod binary_operation;
mod call;
mod condition;
mod print;
mod tuple;
mod variable;

use self::{
    binary_operation::binary_operation,
    call::call_function,
    condition::do_if,
    print::print,
    variable::{assign_variable, get_variable_value},
};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(u32),
    Tuple(Box<Value>, Box<Value>),
    Boolean(bool),
    Function(ast::Function),
}

impl Value {
    fn type_name(&self) -> &str {
        match self {
            Value::Integer(_) => "integer",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Tuple(..) => "tuple",
            Value::Function(_) => "function",
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

#[derive(Clone)]
pub struct Context {
    inner: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    pub fn add(&self, key: &str, value: &Value) -> Self {
        let mut new = self.inner.clone();

        new.insert(key.to_string(), value.clone());

        Self { inner: new }
    }
}

fn eval_term(context: &Context, term: ast::Term) -> Result<Value, Box<dyn Error>> {
    match term {
        ast::Term::If(t) => do_if(context, t),
        ast::Term::First(t) => tuple::first(context, t),
        ast::Term::Second(t) => tuple::second(context, t),
        ast::Term::Print(t) => print(context, t),
        ast::Term::Call(t) => call_function(context, t),
        ast::Term::Let(t) => assign_variable(context, t),
        ast::Term::Var(t) => get_variable_value(context, t),
        ast::Term::Binary(t) => binary_operation(context, t),
        ast::Term::Bool(t) => Ok(Value::Boolean(t.value)),
        ast::Term::Function(t) => Ok(Value::Function(t)),
        ast::Term::Int(t) => Ok(Value::Integer(t.value)),
        ast::Term::Str(t) => Ok(Value::String(t.value)),
        ast::Term::Tuple(t) => Ok(Value::Tuple(
            Box::new(eval_term(context, *t.first)?),
            Box::new(eval_term(context, *t.second)?),
        )),
    }
}

pub fn eval(ast: ast::File) -> Result<(), Box<dyn Error>> {
    eval_term(&Context::new(), ast.expression)?;

    Ok(())
}
