use std::error::Error;

use crate::ast;

use super::{eval_term, extract_primitive_term_kind, Context};

fn binary_operation_sum(lhs: &ast::Term, rhs: &ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match (lhs, rhs) {
        (ast::Term::Int(l), ast::Term::Int(r)) => Ok(ast::Term::Int(ast::Integer {
            value: l.value + r.value,
            location: Default::default(),
        })),
        _ => Err(format!(
            "Cannot call + with {} and {}",
            extract_primitive_term_kind(lhs)?,
            extract_primitive_term_kind(rhs)?
        )
        .into()),
    }
}

fn binary_operation_sub(lhs: &ast::Term, rhs: &ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match (lhs, rhs) {
        (ast::Term::Int(l), ast::Term::Int(r)) => Ok(ast::Term::Int(ast::Integer {
            value: l.value - r.value,
            location: Default::default(),
        })),
        _ => Err(format!(
            "Cannot call - with {} and {}",
            extract_primitive_term_kind(lhs)?,
            extract_primitive_term_kind(rhs)?
        )
        .into()),
    }
}

fn binary_operation_lt(lhs: &ast::Term, rhs: &ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match (lhs, rhs) {
        (ast::Term::Int(l), ast::Term::Int(r)) => Ok(ast::Term::Bool(ast::Boolean {
            value: l.value < r.value,
            location: Default::default(),
        })),
        _ => Err(format!(
            "Cannot call < with {} and {}",
            extract_primitive_term_kind(lhs)?,
            extract_primitive_term_kind(rhs)?
        )
        .into()),
    }
}

fn binary_operation_eq(lhs: &ast::Term, rhs: &ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match (lhs, rhs) {
        (ast::Term::Int(l), ast::Term::Int(r)) => Ok(ast::Term::Bool(ast::Boolean {
            value: l.value == r.value,
            location: Default::default(),
        })),
        _ => Err(format!(
            "Cannot call == with {} and {}",
            extract_primitive_term_kind(lhs)?,
            extract_primitive_term_kind(rhs)?
        )
        .into()),
    }
}

fn binary_operation_or(lhs: &ast::Term, rhs: &ast::Term) -> Result<ast::Term, Box<dyn Error>> {
    match (lhs, rhs) {
        (ast::Term::Bool(l), ast::Term::Bool(r)) => Ok(ast::Term::Bool(ast::Boolean {
            value: l.value || r.value,
            location: Default::default(),
        })),
        _ => Err(format!(
            "Cannot call == with {} and {}",
            extract_primitive_term_kind(lhs)?,
            extract_primitive_term_kind(rhs)?
        )
        .into()),
    }
}

pub fn binary_operation(
    context: &mut Context,
    t: ast::Binary,
) -> Result<ast::Term, Box<dyn Error>> {
    let evaled_lhs = eval_term(context, *t.lhs)?;
    let evaled_rhs = eval_term(context, *t.rhs)?;

    match t.op {
        ast::BinaryOperator::Add => binary_operation_sum(&evaled_lhs, &evaled_rhs),
        ast::BinaryOperator::Sub => binary_operation_sub(&evaled_lhs, &evaled_rhs),
        ast::BinaryOperator::Eq => binary_operation_eq(&evaled_lhs, &evaled_rhs),
        ast::BinaryOperator::Lt => binary_operation_lt(&evaled_lhs, &evaled_rhs),
        ast::BinaryOperator::Or => binary_operation_or(&evaled_lhs, &evaled_rhs),
        _ => Ok(evaled_lhs),
    }
}
