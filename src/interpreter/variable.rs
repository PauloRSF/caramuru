use std::error::Error;

use crate::ast;

use super::{eval_term, Context};

pub fn get_variable_value(
    context: &mut Context,
    t: ast::Variable,
) -> Result<ast::Term, Box<dyn Error>> {
    context
        .get(&t.text)
        .cloned()
        .ok_or(format!("'{}' does not exist", t.text).into())
}

pub fn assign_variable(context: &mut Context, t: ast::Let) -> Result<ast::Term, Box<dyn Error>> {
    let evaled_value = eval_term(context, *t.value)?;

    context.setm(&t.name.text, &evaled_value);

    if let Some(t) = t.next {
        eval_term(context, *t)
    } else {
        Ok(evaled_value.clone())
    }
}
