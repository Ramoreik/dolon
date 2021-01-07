use read_input::prelude::*;
use std::error::Error;
use std::process::{Command, Output};

static USAGE: &str = "im the help message yo.";

fn main() -> Result<(), Box<dyn Error>> {
    let username: String = input::<String>().msg("Enter your username: ").get();
    println!("Hello mr.{} . :)", username);
    shell();
    Ok(())
}

fn shell() {
    loop {
        let cmd: String = input::<String>().msg("[?] Enter your command: ").get();
        match cmd.to_lowercase().as_str() {
            "help" => println!("{}", USAGE),
            "quit" | "exit" => {
                println!("[!] Exit requested, terminating.");
                std::process::exit(0);
            }
            _ => {
                let output: String = String::from_utf8(
                                    command(cmd.to_lowercase()).stdout).unwrap();
                println!("=========================");
                println!("{}", output);
            },
        }
    }
}

fn command(user_cmd: String) -> Output {
    let output = if cfg!(target_os = "windows") {
        Command::new("C:\\Windows\\System32\\cmd.exe")
                .arg("/C")
                .arg(user_cmd)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(user_cmd)
                .output()
                .expect("failed to execute process")
    };
    output
}
