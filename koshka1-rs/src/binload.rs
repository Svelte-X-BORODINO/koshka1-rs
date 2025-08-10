pub mod binload;

use std::fs;

fn binload(path: &str) -> Vec<u8> {
    fs::read(path).except("Failed to read binary")
}

fn prog_run(path: &str) {
    binload(&path);
}