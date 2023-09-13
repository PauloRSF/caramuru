use std::{env::args, error::Error, fs::read_to_string};

use caramuru::{ast::File, interpreter::eval};

fn main() -> Result<(), Box<dyn Error>> {
    let input = args().nth(1).ok_or("You need to provide a file name")?;

    let ast_string = read_to_string(input)?;

    let file: File = serde_json::from_str(&ast_string)?;

    eval(file)
}
