use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn read_to_string(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_string(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn write_lines(path: &Path, lines: &[String]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{line}")?;
    }
    writer.flush()?;
    Ok(())
}
