use std::error::Error;

use crate::ast;

use super::{eval_term, Context};

pub fn print_term(term: &ast::Term) -> Result<(), Box<dyn Error>> {
    match term {
        ast::Term::Int(ast::Integer { value, .. }) => Ok(println!("{value}")),
        ast::Term::Bool(ast::Boolean { value, .. }) => Ok(println!("{value}")),
        ast::Term::Str(ast::Str { value, .. }) => Ok(println!("{value}")),
        _ => Ok(println!("[object Object]")),
    }
}

pub fn print(context: &mut Context, t: ast::Print) -> Result<ast::Term, Box<dyn Error>> {
    let evaled_term = eval_term(context, *t.value)?;

    print_term(&evaled_term)?;

    Ok(ast::Term::Bool(ast::Boolean {
        value: true,
        location: Default::default(),
    }))
}
