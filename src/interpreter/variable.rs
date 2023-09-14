use std::error::Error;

use crate::ast;

use super::{eval_term, Context, Value};

pub fn get_variable_value(context: &Context, t: ast::Variable) -> Result<Value, Box<dyn Error>> {
    context
        .get(&t.text)
        .cloned()
        .ok_or(format!("'{}' does not exist", t.text).into())
}

pub fn assign_variable(context: &Context, t: ast::Let) -> Result<Value, Box<dyn Error>> {
    let value = eval_term(context, *t.value)?;

    let updated_context = context.add(&t.name.text, &value);

    if let Some(next) = t.next {
        eval_term(&updated_context, *next)
    } else {
        Ok(value)
    }
}
