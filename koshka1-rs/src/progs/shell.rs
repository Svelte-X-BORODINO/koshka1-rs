use std::env;
use std::fs;
use std::io::{self, Write};


#[path = "../binload.rs"]
mod binload;
use binload::BinaryLoad;

fn show_help() {
    println!("Available commands:");
    println!("help, mkdir [DIR], cd [DIR], load [FILE].exe, echo [TEXT], exit");
}

pub fn shell() {
    loop {
        print!("k-os -> $ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts: std::str::SplitWhitespace<'_> = input.trim().split_whitespace();
        let command: &str = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect(); 

        match command {
            "help" => show_help(),
            "mkdir" => {
                if let Some(dir_name) = args.get(0) {
                    if let Err(e) = fs::create_dir(dir_name) {
                        eprintln!("mkdir: error: {}", e);
                    }
                } else {
                    println!("Usage: mkdir [DIR]. Try 'help' for more information");
                }
            }
            "cd" => {
                if let Some(dir_name) = args.get(0) {
                    if let Err(e) = env::set_current_dir(dir_name) {
                        eprintln!("cd: error: {}", e);
                    }
                } else {
                    println!("Usage: cd [DIR]. Try 'help' for more information");
                }
            }
            "read" => {
                if let Some(prog) = args.get(0) {
                    
                }
            }
            "load" => {
                if let Some(prog) = args.get(0) {
                    let loader = BinaryLoad::init_binload(&prog);
                    if let Err(e) = BinaryLoad::binload(&args.get(0)) {
                        eprintln!("load: error: {}", e);
                    }
                } else {
                    println!("Usage: load [FILE].exe. Try 'help' for more information");
                }
            }
            "echo" => {
                if !args.is_empty() {
                    println!("{}", args.join(" "));
                } else {
                    println!("Usage: echo [TEXT]. Try 'help' for more information");
                }
            }
            "exit" => break,
            "" => (),
            _ => {
                println!(
                    "Unknown command: '{}'. Try 'help' for more information",
                    command
                );
            }
        }
    }
}