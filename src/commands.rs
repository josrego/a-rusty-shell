use std::env::{self, VarError};
use std::ffi::OsString;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process;
use std::process::Command;

pub trait ShellCommand {
    fn handle_command(&self, args: Vec<&str>);
}

pub struct EchoCommand;
pub struct TypeCommand;
pub struct ProgramCommand;
pub struct ExitCommand;
pub struct PwdCommand;
pub struct CdCommand;

impl ShellCommand for ExitCommand {
    fn handle_command(&self, _args: Vec<&str>) {
        process::exit(0);
    }
}

impl ShellCommand for EchoCommand {
    fn handle_command(&self, args: Vec<&str>) {
        let echo_msg = args.join(" ");
        println!("{}", echo_msg);
    }
}

impl ShellCommand for TypeCommand {
    fn handle_command(&self, args: Vec<&str>) {
        // get just first argument
        let command_arg = args[0];
        match get_available_command(command_arg) {
            Some(_cmd) => println!("{} is a shell builtin", command_arg),
            None => {
                if let Ok(path_env) = std::env::var("PATH") {
                    for path in path_env.split(':') {
                        if let Some(file) = find_file(command_arg, path) {
                            println!("{} is {}/{}", command_arg, path, file.to_str().unwrap());
                            return;
                        }
                    }
                }

                println!("{} not found", command_arg);
            }
        }
    }
}

impl ShellCommand for ProgramCommand {
    fn handle_command(&self, args: Vec<&str>) {
        if args.is_empty() {
            return;
        }

        let program_name = &args[0];
        let program_args = &args[1..];

        if let Ok(path_env) = std::env::var("PATH") {
            for path in path_env.split(':') {
                if let Some(_file) = find_file(program_name, path) {
                    let output = try_run_program(program_name, program_args.to_vec())
                        .unwrap_or(String::from("Program could not be executed"));
                    print!("{}", output);
                    return;
                }
            }
        }

        if let Ok(output) = try_run_program(program_name, program_args.to_vec()) {
            print!("{}", output);
        } else {
            println!("{}: command not found", program_name);
        }
    }
}

impl ShellCommand for PwdCommand {
    fn handle_command(&self, _args: Vec<&str>) {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("Error retrieving current directory: {}", e),
        }
    }
}

impl ShellCommand for CdCommand {
    fn handle_command(&self, args: Vec<&str>) {
        if args.is_empty() {
            go_to_home_dir(String::from("~"));
            return;
        }

        let path = args[0].to_string();
        if path.starts_with('~') {
            go_to_home_dir(path);
        } else {
            move_to_dir(&path);
        }
    }
}

fn go_to_home_dir(mut full_path: String) {
    if let Ok(home_dir) = get_home_dir() {
        full_path = full_path.replacen('~', home_dir.as_str(), 1);
        move_to_dir(&full_path);
    }
}

fn get_home_dir() -> Result<String, VarError> {
    if cfg!(target_os = "windows") {
        return env::var("USERPROFILE");
    }
    env::var("HOME")
}

fn move_to_dir(full_path: &str) {
    let new_dir_path = Path::new(full_path);
    if env::set_current_dir(new_dir_path).is_err() {
        println!("{}: No such file or directory", full_path)
    }
}

fn try_run_program(program_name: &str, args: Vec<&str>) -> Result<String, Error> {
    let output = Command::new(program_name)
        .args(args)
        .output()
        .expect("Failed to execute error");
    let output_stdout_utf8 = String::from_utf8(output.stdout).unwrap();
    Ok(output_stdout_utf8)
}

fn find_file(file_name: &str, dir: &str) -> Option<OsString> {
    let entries = fs::read_dir(dir).unwrap();
    entries
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().is_ok_and(|t| t.is_file()))
        .map(|entry| entry.file_name())
        .find(|fname| fname.eq_ignore_ascii_case(file_name))
}

pub fn get_available_command(cmd_token: &str) -> Option<Box<dyn ShellCommand>> {
    match cmd_token {
        "exit" => Some(Box::new(ExitCommand {})),
        "echo" => Some(Box::new(EchoCommand {})),
        "type" => Some(Box::new(TypeCommand {})),
        "pwd" => Some(Box::new(PwdCommand {})),
        "cd" => Some(Box::new(CdCommand {})),
        _ => None,
    }
}
