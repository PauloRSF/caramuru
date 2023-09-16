pub mod ast;
pub mod interpreter;

use ast::File;
use miette::IntoDiagnostic;
use rinha::parser::parse_or_report;
use std::fs::read_to_string;

pub fn parse_source(file_path: &String) -> miette::Result<File> {
    let file_contents = read_to_string(file_path).into_diagnostic()?;

    // TODO: Implement actual conversion between aripiprazole's AST structure
    // and my own instead of this awful AST->JSON->AST thing
    let foreign_ast = parse_or_report(file_path, &file_contents)?;
    let serialized_ast = serde_json::to_string(&foreign_ast).into_diagnostic()?;

    serde_json::from_str::<File>(&serialized_ast).into_diagnostic()
}
