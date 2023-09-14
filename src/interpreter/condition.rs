use std::error::Error;

use crate::ast;

use super::{eval_term, Context, Value};

pub fn do_if(context: &mut Context, t: ast::If) -> Result<Value, Box<dyn Error>> {
    match eval_term(context, *t.condition.clone())? {
        Value::Boolean(condition) => {
            eval_term(context, if condition { *t.then } else { *t.otherwise })
        }
        value => Err(format!("Expected boolean, got {}", value.type_name()).into()),
    }
}
