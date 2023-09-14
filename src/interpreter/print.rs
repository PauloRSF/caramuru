use std::error::Error;

use crate::ast;

use super::{eval_term, Context, Value};

pub fn print(context: &mut Context, t: ast::Print) -> Result<Value, Box<dyn Error>> {
    println!("{}", eval_term(context, *t.value)?);

    Ok(Value::Boolean(true))
}
