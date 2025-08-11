use std::io::{self, Write};
use std::process::Command;
use std::fs;
use std::env;
use crate::binload;

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
                    } else {
                        fs::create_dir(args.get(0));
                    }
                } else {
                    println!("Usage: mkdir [DIR]. Try 'help' for more information");
                }
            },
            "cd" => {
                if let Some(dir_name) = args.get(0) {
                    if let Err(e) = fs::set_current_dir(dir_name) {
                        eprintln!("cd: error: {}", e);
                    }
                } else {
                    println!("Usage: cd [DIR]. Try 'help' for more information");
                    
                }
            },
            "load" => {
                if let Some(prog) = args.get(0) {
                    if let Err(e) = prog_run(args.get(0)) { // prog_run() - из ../binload.rs
                        eprintln!("load: error: {}", e);
                    }
                } else {
                    println!("Usage: load [FILE].exe. Try 'help' for more information");
                }
            },
            "echo" => {
                if let Some(text) = args.get(0) {
                    
                }
            }
            "exit" => break,
            "" => (), 
            _ => {
                println!("Unknown command: '{}'", command);
            }
        }
    }
}



