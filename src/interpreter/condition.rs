use std::error::Error;

use crate::ast;

use super::{eval_term, Context};

fn extract_boolean(term: &ast::Term) -> Result<ast::Boolean, Box<dyn Error>> {
    match term {
        ast::Term::Bool(t) => Ok(t.clone()),
        _ => Err("Expected boolean, got XXXXX".into()),
    }
}

pub fn do_if(context: &mut Context, t: ast::If) -> Result<ast::Term, Box<dyn Error>> {
    let evaled_cond = eval_term(context, *t.condition.clone())?;
    let value = extract_boolean(&evaled_cond)?.value;

    eval_term(context, if value { *t.then } else { *t.otherwise })
}
