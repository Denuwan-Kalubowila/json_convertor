mod file_io;
mod cli;

use std::io;
use std::path::Path;

fn run(input: &Path,output:& Path) ->io::Result<()> {
    //let input_path = Path::new("input.txt");
    //let output_path = Path::new("output.txt");



    let lines = file_io::read_lines(input)?;
    file_io::write_lines(output, &lines)?;

    println!("File processed sucessfully!");

    Ok(())
}

fn main() {
    let args = match cli::parse_args() {
        Ok(args)=>args,
        Err(error) =>{
            eprintln!("Error: {error}");
            eprintln!("Usage: cargo run -- <input> <output>");
            return;
        }
    };
    if let Err(error) = run(&args.input,&args.output) {
        eprintln!("Application failed: {error}")
    } 
}
