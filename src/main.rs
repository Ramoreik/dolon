use std::process::Command;
use std::error::Error;
use read_input::prelude::*;

static USAGE: &str = "im the help message yo.";


fn main() -> Result<(), Box<dyn Error>>{
    let username: String = input::<String>().msg("Enter your username: ").get();
    println!("Hello mr.{} . :)", username);
    shell();
    Ok(())
}

fn shell(){
    loop {
        let command: String = input::<String>().msg("[?] Enter your command: ").get();
        match command.to_lowercase().as_str(){
            "help" => println!("{}", USAGE),
            "quit"|"exit" => {
                println!("[!] Exit requested, terminating.");
                std::process::exit(0);
            },
            _ => eprintln!("[!] Command not found."),
        }
    }
}