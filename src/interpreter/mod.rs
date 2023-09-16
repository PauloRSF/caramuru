use std::{collections::HashMap, error::Error};

use crate::ast;

mod binary_operation;
mod call;
mod condition;
mod print;
mod tuple;
mod value;
mod variable;

use self::{
    binary_operation::binary_operation,
    call::call_function,
    condition::do_if,
    print::print,
    value::Value,
    variable::{assign_variable, get_variable_value},
};

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

fn eval_term(context: &Context, term: &ast::Term) -> Result<Value, Box<dyn Error>> {
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
        ast::Term::Function(t) => Ok(Value::Function(t.clone())),
        ast::Term::Int(t) => Ok(Value::Integer(t.value)),
        ast::Term::Str(t) => Ok(Value::String(t.value.clone())),
        ast::Term::Tuple(t) => Ok(Value::Tuple(
            Box::new(eval_term(context, &t.first)?),
            Box::new(eval_term(context, &t.second)?),
        )),
    }
}

pub fn eval(ast: ast::File) -> Result<(), Box<dyn Error>> {
    eval_term(&Context::new(), &ast.expression)?;

    Ok(())
}
