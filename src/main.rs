mod cli;
mod file_io;
mod formats;

use std::error::Error;
use std::path::Path;

fn run(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    //let input_path = Path::new("input.txt");
    //let output_path = Path::new("output.txt");
    let content = file_io::read_to_string(input)?;
    let value = formats::json::parse_json(&content)?;
    let formated_content = formats::json::format_json(&value)?;

    file_io::write_string(output, &formated_content)?;

    let lines = file_io::read_lines(input)?;
    file_io::write_lines(output, &lines)?;

    println!("File processed sucessfully!");

    Ok(())
}

fn main() {
    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error: {error}");
            eprintln!("Usage: cargo run -- <input> <output>");
            return;
        }
    };
    if let Err(error) = run(&args.input, &args.output) {
        eprintln!("Application failed: {error}")
    }
}
