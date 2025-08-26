use std::fs;
use std::io::Result;

fn binload(path: &str) -> Result<()> {
    fs::read(path);
    Ok(())
}

pub fn prog_run(path: &str) {
    binload(&path);
}