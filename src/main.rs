use read_input::prelude::*;
use std::error::Error;
use std::process::{Command, Output};

static USAGE: &str = "im the help message yo.";
static SHELL: [&str; 2] = shell_program();

const fn shell_program() -> [&'static str; 2]{
    if cfg!(target_os = "windows"){
        ["C:\\Windows\\System32\\cmd.exe","/C"]
    }else{
        ["sh", "-c"]
    }
}

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
                display(command(cmd))
            },
        }
    }
}

fn command(user_cmd: String) -> Output {
    Command::new(SHELL[0])
            .arg(SHELL[1])
            .arg(user_cmd)
            .output()
            .expect("failed to execute process")
}

fn display(output: Output) {
    let stdout: String = match String::from_utf8(output.stdout){
        Ok(stdout) => stdout,
        Err(_) => {
            eprintln!("[!] Trouble parsing stdout, exiting.");
            std::process::exit(1);
        }
    };
    let stderr: String = match String::from_utf8(output.stderr){
        Ok(stderr) => stderr,
        Err(_)=> {
            eprintln!("[!] Trouble parsing stderr, exiting");
            std::process::exit(1);
        }
    };
    println!("============================");
    println!("{}", stdout);
    println!("{}", stderr);
}
