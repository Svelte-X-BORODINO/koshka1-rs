use std::io::Result;
use std::process::{Command, ExitStatus};
use std::fs;
pub struct BinaryLoad<'a> {
    name: &'a str,
}

impl<'a> BinaryLoad<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        fs::read(self.name)
    }

    pub fn run(&self) -> Result<ExitStatus> {
        Command::new(self.name).status()
    }
}