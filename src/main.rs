mod commands;

use crate::commands::*;
use std::env;
use std::io::{self, Write};

fn get_current_dir() -> Option<String> {
    if let Ok(path) = env::current_dir() {
        return Some(path.display().to_string());
    }
    None
}

fn main() {
    loop {
        if let Some(current_dir) = get_current_dir() {
            print!("({current_dir}) $ ");
        } else {
            print!("$ ");
        }
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.is_empty() {
            continue;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        let command = args[0];

        match get_available_command(command) {
            Some(shell_cmd) => shell_cmd.handle_command(args[1..].to_vec()),
            None => {
                let cmd = ProgramCommand {};
                cmd.handle_command(args)
            }
        }
    }
}
