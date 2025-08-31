use std::io::Result;
use std::fs;
use std::process::Command;

pub struct BinaryLoadData {
    data: Vec<u8>
}
pub struct BinaryLoad<'a> {
    name: &'a str,
}

impl<'a> BinaryLoad<'a> {
    pub fn init_binload(name: &'a str) -> Self {
        Self { name }
    }

    pub fn binread(&self) -> Result<BinaryLoadData> {
        fs::read(self.name)
    }

    pub fn binload(&self) {
        Command::new(self.name)
            .spawn()?
            .expect("Failed to execute a binary")?;
    }
}