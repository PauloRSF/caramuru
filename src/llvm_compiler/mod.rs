use std::error::Error;

use inkwell::{builder::Builder, context, module::Module, values::IntValue};

mod condition;

use crate::ast;

use self::condition::do_if;

pub struct Context<'ctx> {
    inner: &'ctx context::Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Context<'ctx> {
    fn new(inner: &'ctx context::Context) -> Self {
        Self {
            inner,
            module: inner.create_module("foo"),
            builder: inner.create_builder(),
        }
    }
}

fn compile_term<'a>(
    context: &'a Context<'a>,
    term: &'a ast::Term,
) -> Result<IntValue<'a>, Box<dyn Error>> {
    match term {
        ast::Term::If(t) => do_if(context, t),
        // ast::Term::First(t) => tuple::first(context, t),
        // ast::Term::Second(t) => tuple::second(context, t),
        // ast::Term::Print(t) => print(context, t),
        // ast::Term::Call(t) => call_function(context, t),
        // ast::Term::Let(t) => assign_variable(context, t),
        // ast::Term::Var(t) => get_variable_value(context, t),
        // ast::Term::Binary(t) => binary_operation(context, t),
        // ast::Term::Bool(t) => Ok(Value::Boolean(t.value)),
        // ast::Term::Function(t) => Ok(Value::Function(t.clone())),
        // ast::Term::Int(t) => Ok(Value::Integer(t.value)),
        // ast::Term::Str(t) => Ok(Value::String(t.value.clone())),
        // ast::Term::Tuple(t) => Ok(Value::Tuple(
        //     Box::new(compile_term(context, &t.first)?),
        //     Box::new(compile_term(context, &t.second)?),
        // )),
        _ => Ok(context.inner.i64_type().const_zero()),
    }
}

pub fn compile(ast: ast::File) -> Result<(), Box<dyn Error>> {
    let inner = context::Context::create();
    let context = Context::new(&inner);

    // compile_term(&context, &ast.expression)?;

    Ok(())
}

// use inkwell::builder::Builder;
// use inkwell::context::Context;
// use inkwell::execution_engine::{ExecutionEngine, JitFunction};
// use inkwell::module::{Linkage, Module};
// use inkwell::types::BasicMetadataTypeEnum;
// use inkwell::values::BasicMetadataValueEnum;
// use inkwell::{AddressSpace, OptimizationLevel};
// use std::error::Error;

// /// Convenience type alias for the `sum` function.
// ///
// /// Calling this is innately `unsafe` because there's no guarantee it doesn't
// /// do `unsafe` operations internally.
// type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

// struct CodeGen<'ctx> {
//     context: &'ctx Context,
//     module: Module<'ctx>,
//     builder: Builder<'ctx>,
//     execution_engine: ExecutionEngine<'ctx>,
// }

// impl<'ctx> CodeGen<'ctx> {
//     fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
//         let i64_type = self.context.i64_type();
//         let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
//         let function = self.module.add_function("sum", fn_type, None);
//         let basic_block = self.context.append_basic_block(function, "entry");

//         self.builder.position_at_end(basic_block);

//         let x = function.get_nth_param(0)?.into_int_value();
//         let y = function.get_nth_param(1)?.into_int_value();
//         let z = function.get_nth_param(2)?.into_int_value();

//         let abc;
//         let qwe;

//         unsafe {
//             abc = self.builder.build_global_string("%s", "abc_format");
//             qwe = self.builder.build_global_string("Hellora\n", "argp");
//         }

//         let str_type = self.context.i8_type().ptr_type(AddressSpace::default());
//         let printf_type = i64_type.fn_type(&[BasicMetadataTypeEnum::PointerType(str_type)], true);
//         let printf = self
//             .module
//             .add_function("printf", printf_type, Some(Linkage::External));

//         self.builder.build_call(
//             printf,
//             &[
//                 BasicMetadataValueEnum::PointerValue(abc.as_pointer_value()),
//                 BasicMetadataValueEnum::PointerValue(qwe.as_pointer_value()),
//             ],
//             "",
//         );

//         let sum = self.builder.build_int_add(x, y, "sum");
//         let sum = self.builder.build_int_add(sum, z, "sum");

//         self.builder.build_return(Some(&sum));

//         // self.module.print_to_stderr();

//         unsafe { self.execution_engine.get_function("sum").ok() }
//     }
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let context = Context::create();
//     let module = context.create_module("sum");
//     let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
//     let codegen = CodeGen {
//         context: &context,
//         module,
//         builder: context.create_builder(),
//         execution_engine,
//     };

//     let sum = codegen
//         .jit_compile_sum()
//         .ok_or("Unable to JIT compile `sum`")?;

//     let x = 1u64;
//     let y = 2u64;
//     let z = 3u64;

//     unsafe {
//         println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
//     }

//     Ok(())
// }
