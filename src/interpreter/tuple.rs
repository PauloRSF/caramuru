use std::error::Error;

use crate::ast;

pub fn first(t: ast::First) -> Result<ast::Term, Box<dyn Error>> {
    match *t.value {
        ast::Term::Tuple(t1) => Ok(*t1.first),
        _ => Err("'first' called on non-tuple".into()),
    }
}

pub fn second(t: ast::Second) -> Result<ast::Term, Box<dyn Error>> {
    match *t.value {
        ast::Term::Tuple(t1) => Ok(*t1.second),
        _ => Err("'second' called on non-tuple".into()),
    }
}
