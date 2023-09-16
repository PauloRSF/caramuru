pub mod ast;
pub mod interpreter;

use std::fs::read_to_string;

use ast::File;
use miette::IntoDiagnostic;
use rinha::parser::parse_or_report;

pub fn parse_file(file_path: &String) -> miette::Result<File> {
    read_to_string(file_path)
        .into_diagnostic()
        .and_then(|source| parse_or_report(file_path, &source).into_diagnostic())
        .map(|file_node| File::from(file_node))
}
