use std::io::{self, Write};
use std::process::Command;
use std::fs;
use std::env;

macro_rules! help {
    () => {
        println!("Available commands:");
        println!("help, mkdir [DIR], cd [DIR], load [FILE].exe, echo [TEXT], exit");
    }
}

pub fn main() {
    loop {
        print!("k-os -> $ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "help" => help!(),
            "mkdir" => {
                if let Some(dir_name) = args.get(0) {
                    if let Err(e) = fs::create_dir(dir_name) {
                        eprintln!("mkdir: error: {}", e);
                    }
                } else {
                    println!("Usage: mkdir [DIR]");
                }
            },
            "cd" => {
                if let Some(dir_name) = args.get(0) {
                    if let Err(e) = fs::change_dir(dir_name) {
                        eprintln!("cd: error: {}", e);
                    }
                } else {
                    println!("")
                }
            }
            "exit" => break,
            "" => (), // Do nothing on empty input
            _ => {
                println!("Unknown command: '{}'", command);
            }
        }
    }
}



