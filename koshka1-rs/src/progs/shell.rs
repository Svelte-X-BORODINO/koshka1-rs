use std::env;
use std::fs;
use std::io::{self, Write};


#[path = "../binload.rs"]
mod binload;

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

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect(); // Изменил на Vec<&str>

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
            "load" => {
                if let Some(prog) = args.get(0) {
                    // Предполагаем, что есть функция binload::run
                    if let Err(e) = binload::prog_run(prog) {
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