use std::{collections::HashMap, error::Error};

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

pub struct RuntimeContext {}

#[derive(Clone)]
pub struct Context {
    inner: HashMap<String, ast::Term>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&ast::Term> {
        self.inner.get(key)
    }

    pub fn setm(&mut self, key: &str, value: &ast::Term) {
        self.inner.insert(key.to_string(), value.clone());
    }

    pub fn set(&self, key: &str, value: &ast::Term) -> Self {
        let mut new = self.inner.clone();

        new.insert(key.to_string(), value.clone());

        Self { inner: new }
    }
}

fn extract_primitive_term_kind(term: &ast::Term) -> Result<String, Box<dyn Error>> {
    match term {
        ast::Term::Str(_) => Ok(String::from("String")),
        ast::Term::Int(_) => Ok(String::from("Integer")),
        ast::Term::Tuple(_) => Ok(String::from("Tuple")),
        ast::Term::Bool(_) => Ok(String::from("Boolean")),
        ast::Term::Function(_) => Ok(String::from("Function")),
        _ => Err("Should never extract type of non-primitive".into()),
    }
}

fn eval_term(context: &mut Context, term: ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match term {
        ast::Term::If(t) => do_if(context, t),
        ast::Term::First(t) => tuple::first(t),
        ast::Term::Second(t) => tuple::second(t),
        ast::Term::Print(t) => print(context, t),
        ast::Term::Call(t) => call_function(context, t),
        ast::Term::Let(t) => assign_variable(context, t),
        ast::Term::Var(t) => get_variable_value(context, t),
        ast::Term::Binary(t) => binary_operation(context, t),
        t => Ok(t),
    }
}

pub fn eval(ast: ast::File) -> Result<(), Box<dyn Error>> {
    let mut context = Context::new();

    eval_term(&mut context, ast.expression)?;

    Ok(())
}
