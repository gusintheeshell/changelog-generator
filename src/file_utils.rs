use std::fs::File;
use std::io::{self, Write};

pub fn save_to_file(content: &str, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
