use std::error::Error;

use inkwell::values::IntValue;

use crate::ast;

use super::{compile_term, Context};

pub fn do_if<'a>(context: &'a Context<'a>, t: &'a ast::If) -> Result<IntValue<'a>, Box<dyn Error>> {
    let parent = context
        .builder
        .get_insert_block()
        .unwrap()
        .get_parent()
        .unwrap();
    let condition = compile_term(context, &t.condition)?;

    context.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        condition,
        context.inner.bool_type().const_zero(),
        "ifexpr",
    );

    let then_bb = context.inner.append_basic_block(parent, "then");
    let else_bb = context.inner.append_basic_block(parent, "else");
    let cont_bb = context.inner.append_basic_block(parent, "ifcont");

    context
        .builder
        .build_conditional_branch(condition, then_bb, else_bb);

    // build then block
    context.builder.position_at_end(then_bb);
    let then_val = compile_term(context, &t.then)?;
    context.builder.build_unconditional_branch(cont_bb);

    let then_bb = context.builder.get_insert_block().unwrap();

    // build else block
    context.builder.position_at_end(else_bb);
    let else_val = compile_term(context, &t.otherwise)?;
    context.builder.build_unconditional_branch(cont_bb);

    let else_bb = context.builder.get_insert_block().unwrap();

    // emit merge block
    context.builder.position_at_end(cont_bb);

    let phi = context.builder.build_phi(context.inner.f64_type(), "iftmp");

    phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);

    Ok(phi.as_basic_value().into_int_value())
}
