use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn  read_lines(path: &Path) -> io::Result<Vec<String>> {
    let file =  File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn write_lines(path: &Path, lines: &[String]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer,"{line}")?;
    }
    writer.flush()?;
    Ok(())

}