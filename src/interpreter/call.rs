use std::error::Error;

use crate::ast;

use super::{eval_term, value::Value, Context};

fn get_function_callee(context: &Context, term: &ast::Term) -> Result<String, Box<dyn Error>> {
    if let ast::Term::Str(ast::Str { value, .. }) = term {
        return Ok(value.clone());
    }

    if let ast::Term::Var(ast::Variable { text, .. }) = term {
        if let Some(Value::Function(_)) = context.get(text) {
            return Ok(text.clone());
        }
    }

    Err("Function callee is not valid".into())
}

pub fn call_function(context: &Context, t: ast::Call) -> Result<Value, Box<dyn Error>> {
    let ast::Call {
        callee, arguments, ..
    } = t;

    let callee_name = get_function_callee(&context, &callee)?;

    match context.get(&callee_name).cloned() {
        Some(Value::Function(ast::Function {
            parameters, value, ..
        })) => {
            if parameters.len() > arguments.len() {
                return Err(format!(
                    "'{}' expected {} arguments, but got {}",
                    callee_name,
                    parameters.len(),
                    arguments.len()
                )
                .into());
            }

            let evaled_args = arguments
                .iter()
                .map(|arg| eval_term(context, *arg.clone()))
                .collect::<Result<Vec<_>, _>>()?;

            let call_context = parameters
                .iter()
                .zip(evaled_args.iter())
                .fold(context.clone(), |ctx, (parameter, argument)| {
                    ctx.add(&parameter.text, argument)
                });

            eval_term(&call_context, *value)
        }
        Some(_) => Err(format!("'{}' is not callable", callee_name).into()),
        _ => Err(format!("'{}' does not exist", callee_name).into()),
    }
}
