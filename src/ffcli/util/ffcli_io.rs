use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::util::errors::Result;

/// Write the contents to the given `path`.
pub fn write<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<()> {
    let mut f = File::create(path.as_ref())?;
    f.write_all(contents)?;
    Ok(())
}

/// Write the contents to the given `path` in "append mode".
pub fn append<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    f.write_all(contents)?;
    Ok(())
}

/// Remove a single file specified by `path`.
pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path.as_ref())
}

/// Read a file from the given `path`.
pub fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}
