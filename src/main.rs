use std::{env::args, error::Error};

use caramuru::{interpreter::eval, parse_file};

fn main() -> Result<(), Box<dyn Error>> {
    let source_file_path = args().nth(1).ok_or("You need to provide a file name")?;

    let file = parse_file(&source_file_path)?;

    eval(file)
}
