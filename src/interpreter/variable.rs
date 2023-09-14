use std::error::Error;

use crate::ast;

use super::{eval_term, Context, Value};

pub fn get_variable_value(
    context: &mut Context,
    t: ast::Variable,
) -> Result<Value, Box<dyn Error>> {
    context
        .get(&t.text)
        .cloned()
        .ok_or(format!("'{}' does not exist", t.text).into())
}

pub fn assign_variable(context: &mut Context, t: ast::Let) -> Result<Value, Box<dyn Error>> {
    let value = eval_term(context, *t.value)?;

    context.setm(&t.name.text, &value);

    if let Some(t) = t.next {
        eval_term(context, *t)
    } else {
        Ok(value)
    }
}
