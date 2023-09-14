use std::error::Error;

use crate::ast;

use super::{eval_term, value::Value, Context};

fn binary_operation_sum(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
        _ => Err(format!(
            "Cannot call + with {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_sub(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs - rhs)),
        _ => Err(format!(
            "Cannot call - with {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_lt(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs < rhs)),
        _ => Err(format!(
            "Cannot call < with {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_eq(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs == rhs)),
        _ => Err(format!(
            "Cannot call == with {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_or(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(*lhs || *rhs)),
        _ => Err(format!(
            "Cannot call == with {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

pub fn binary_operation(context: &Context, t: ast::Binary) -> Result<Value, Box<dyn Error>> {
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
