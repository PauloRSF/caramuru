use std::error::Error;

use crate::ast;

use super::{eval_term, value::Value, Context};

pub fn first(context: &Context, t: ast::First) -> Result<Value, Box<dyn Error>> {
    match eval_term(context, *t.value)? {
        Value::Tuple(first, _) => Ok(*first),
        _ => Err("'first' called on non-tuple".into()),
    }
}

pub fn second(context: &Context, t: ast::Second) -> Result<Value, Box<dyn Error>> {
    match eval_term(context, *t.value)? {
        Value::Tuple(_, second) => Ok(*second),
        _ => Err("'second' called on non-tuple".into()),
    }
}
