use std::{fs, io};

use crate::{binload, cpu, shell, video};

use core::process::Command;

fn init() {
    init_cpu();
    dispd("CPU Initialized");
    init_video();
    
}

fn main() {}
