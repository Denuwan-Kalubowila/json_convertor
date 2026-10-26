mod cli;
mod file_io;
mod formats;

use std::error::Error;
use std::io::{Error as IoError, ErrorKind};
use std::path::Path;

fn run(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    //let input_path = Path::new("input.txt");
    //let output_path = Path::new("output.txt");
    //let content = file_io::read_to_string(input)?;
    //let lines = formats::json::json_to_jsonl(&content)?;

    let input_ext = input.extension().and_then(|ext| ext.to_str());
    let output_ext = output.extension().and_then(|ext| ext.to_str());

    match (input_ext, output_ext) {
        (Some("json"), Some("jsonl")) => {
            let content = file_io::read_to_string(input)?;
            let lines = formats::json::json_to_jsonl(&content)?;
            let count = lines.len();

            file_io::write_lines(output, &lines)?;
            println!("JSON to JSONL conversion completed: {count} records written");
        }

        (Some("jsonl"), Some("json")) => {
            let lines = file_io::read_lines(input)?;
            let output_cont = formats::json::jsonl_to_json(&lines)?;

            file_io::write_string(output, &output_cont)?;
            println!("JSONL to JSON conversion completed");
        }

        _ => {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                "Unsupported file extensions. Use .json -> .jsonl or .jsonl -> .json",
            )
            .into());
        }
    }
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
