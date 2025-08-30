use std::io::Result;
use std::fs;

pub struct BinaryLoad<'a> {
    name: &'a str,
}

impl<'a> BinaryLoad<'a> {
    pub fn init_binload(name: &'a str) -> Self {
        Self { name }
    }

    pub fn binload(&self) -> Result<Vec<u8>> {
        fs::read(self.name)
    }
}