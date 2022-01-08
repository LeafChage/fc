use std::fs::File;
use std::io::{BufReader, Read, Result};

pub fn read_to_binary(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}
