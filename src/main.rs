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
                let output = command(cmd.to_lowercase());
                // println!("{}", String::from_utf8(output.stdout).unwrap());
                println!("{:?}", output.stdout);
            },
        }
    }
}

fn command(user_cmd: String) -> Output {
    let mut cmd: Vec<&str> = user_cmd.split("").collect();
    let output = if cfg!(target_os = "windows") {
        cmd.insert(0, "/C");
        Command::new("C:\\Windows\\System32\\cmd.exe")
                .args(&cmd)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("")
                .output()
                .expect("failed to execute process")
    };
    output
}
