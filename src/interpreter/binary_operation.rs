use std::error::Error;

use crate::ast;

use super::{eval_term, value::Value, Context};

fn binary_operation_sum(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
        (Value::Integer(lhs), Value::String(rhs)) => Ok(Value::String(format!("{}{}", lhs, rhs))),
        (Value::String(lhs), Value::Integer(rhs)) => Ok(Value::String(format!("{}{}", lhs, rhs))),
        (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(format!("{}{}", lhs, rhs))),
        _ => Err(format!(
            "+ is unsupported for {} and {}",
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
            "- is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_mul(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs * rhs)),
        _ => Err(format!(
            "* is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_div(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs / rhs)),
        _ => Err(format!(
            "/ is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_rem(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs % rhs)),
        _ => Err(format!(
            "% is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_gt(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs > rhs)),
        _ => Err(format!(
            "> is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_gte(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs >= rhs)),
        _ => Err(format!(
            ">= is unsupported for {} and {}",
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
            "< is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_lte(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs <= rhs)),
        _ => Err(format!(
            "<= is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_eq(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs == rhs)),
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs == rhs)),
        (Value::String(lhs), Value::String(rhs)) => Ok(Value::Boolean(lhs == rhs)),
        (Value::Tuple(lhs_first, lhs_second), Value::Tuple(rhs_first, rhs_second)) => {
            match (
                binary_operation_eq(lhs_first, rhs_first)?,
                binary_operation_eq(lhs_second, rhs_second)?,
            ) {
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
                _ => Ok(Value::Boolean(false)),
            }
        }
        _ => Err(format!(
            "== is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_neq(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs != rhs)),
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs != rhs)),
        (Value::String(lhs), Value::String(rhs)) => Ok(Value::Boolean(lhs != rhs)),
        (Value::Tuple(lhs_first, lhs_second), Value::Tuple(rhs_first, rhs_second)) => {
            match (
                binary_operation_neq(lhs_first, rhs_first)?,
                binary_operation_neq(lhs_second, rhs_second)?,
            ) {
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
                _ => Ok(Value::Boolean(false)),
            }
        }
        _ => Err(format!(
            "!= is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

fn binary_operation_and(lhs_value: &Value, rhs_value: &Value) -> Result<Value, Box<dyn Error>> {
    match (lhs_value, rhs_value) {
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(*lhs && *rhs)),
        _ => Err(format!(
            "&& is unsupported for {} and {}",
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
            "|| is unsupported for {} and {}",
            lhs_value.type_name(),
            rhs_value.type_name()
        )
        .into()),
    }
}

pub fn binary_operation(context: &Context, t: &ast::Binary) -> Result<Value, Box<dyn Error>> {
    let lhs = eval_term(context, &t.lhs)?;
    let rhs = eval_term(context, &t.rhs)?;

    match t.op {
        ast::BinaryOperator::Add => binary_operation_sum(&lhs, &rhs),
        ast::BinaryOperator::Sub => binary_operation_sub(&lhs, &rhs),
        ast::BinaryOperator::Mul => binary_operation_mul(&lhs, &rhs),
        ast::BinaryOperator::Div => binary_operation_div(&lhs, &rhs),
        ast::BinaryOperator::Rem => binary_operation_rem(&lhs, &rhs),
        ast::BinaryOperator::Eq => binary_operation_eq(&lhs, &rhs),
        ast::BinaryOperator::Neq => binary_operation_neq(&lhs, &rhs),
        ast::BinaryOperator::Gt => binary_operation_gt(&lhs, &rhs),
        ast::BinaryOperator::Gte => binary_operation_gte(&lhs, &rhs),
        ast::BinaryOperator::Lt => binary_operation_lt(&lhs, &rhs),
        ast::BinaryOperator::Lte => binary_operation_lte(&lhs, &rhs),
        ast::BinaryOperator::And => binary_operation_and(&lhs, &rhs),
        ast::BinaryOperator::Or => binary_operation_or(&lhs, &rhs),
    }
}
