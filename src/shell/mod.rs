use std::error::Error;
use read_input::prelude::*;
use std::process::{Command, Output};

static PROMPT: &str = "d0l0n💀: ";
static SHELL: [&str; 2] = shell_program();
static USAGE: &str = "im the help message yo.";

const fn shell_program() -> [&'static str; 2]{
    if cfg!(target_os = "windows"){
        ["C:\\Windows\\System32\\cmd.exe","/C"]
    }else{
        ["sh", "-c"]
    }
}

pub struct Shell{
    binary: String,
    option: String,
    last: Option<String>,
    cmd: Option<String>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            binary: SHELL[0].to_owned(),
            option: SHELL[1].to_owned(),
            last: None,
            cmd: None
        }
    }

    pub fn init(&self){
        loop {
            let cmd: String = input::<String>().msg(PROMPT).get();
            match cmd.to_lowercase().as_str() {
                "help" => println!("{}", USAGE),
                "quit" | "exit" => {
                    println!("[!] Exit requested, terminating.");
                    std::process::exit(0);
                }
                _ => {
                    match self.cmd_out(cmd){
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("[!] Could not execute shell command.");
                        }
                    };
                },
            }
        }
    }

    pub fn command(&self, cmd: String) -> Result<Output, Box<dyn Error>> {
        Ok(Command::new(&self.binary)
                .arg(&self.option)
                .arg(cmd)
                .output()
                .expect("failed to execute process")
        )
    }

    pub fn out(&self, output: Output) -> Result<(), Box<dyn Error>>{
        match output {
            Output {stdout, stderr, status: _} => {
                println!("==========");
                print!("{}", String::from_utf8(stdout)?);
                println!("{}", String::from_utf8(stderr)?);
            },
        };
        Ok(())
    }

    pub fn cmd_out(&self, cmd: String) -> Result<(), Box<dyn Error>>{
        let output = match self.command(cmd){
            Ok(output) => output,
            Err(_) => {
                eprintln!("[!] Problem while executing command.");
                panic!();
            }
        };
        self.out(output)?;
        Ok(())
    }
}
