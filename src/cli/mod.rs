use std::env;
use std::path::PathBuf;

pub struct CliArgs {
    pub input: PathBuf,
    pub output:PathBuf,
}

pub fn parse_args() -> Result<CliArgs,String> {
    let mut args = env::args().skip(1);

    let input = args
        .next()
        .ok_or("Missing input_path")?;

    let output = args
        .next()
        .ok_or("Missing output_path")?;

    if args.next().is_some() {
        return Err("To many args".to_string());
    }

    Ok(CliArgs{
        input: PathBuf::from(input),
        output: PathBuf::from(output),
    })


}

