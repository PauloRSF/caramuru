use std::error::Error;

use crate::ast;

use super::{eval_term, value::Value, Context};

pub fn do_if(context: &Context, t: &ast::If) -> Result<Value, Box<dyn Error>> {
    match eval_term(context, &t.condition.clone())? {
        Value::Boolean(true) => eval_term(context, &t.then),
        Value::Boolean(false) => eval_term(context, &t.otherwise),
        value => Err(format!("Expected boolean, got {}", value.type_name()).into()),
    }
}
