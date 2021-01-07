use std::error::Error;
use read_input::prelude::*;
use std::process::{Command, Output, Child, Stdio};
use std::io::{Write, Read, BufReader, BufWriter, BufRead};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

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
    interactive: Option<Child>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            binary: SHELL[0].to_owned(),
            option: SHELL[1].to_owned(),
            last: None,
            cmd: None,
            interactive: None
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

    // DEVNOTE: Does not quite work, it executes ls one time but then stops working.
    // I'm pretty sure that it is caused by take, which makes the process' stdin none after its use.
    // Since im looping the next iterating is then always empty.
    // I have to read on BufReaders but i'm too tired.
    // https://stackoverflow.com/questions/21615188/how-to-send-input-to-a-program-through-stdin-in-rust
    // https://doc.rust-lang.org/std/process/struct.Stdio.html

    pub fn interactive_session(&self) -> Result<(), Box<dyn Error>> {
        // Channels to send commands to thread
        let (sti_sender, sti_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

        // Channels to receive output from thread
        let (sto_sender, sto_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

        // launch shell thread
        std::thread::spawn(move || {
            // launch bash in interactive (default)
            let mut handle: Child = Command::new("bash")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process");

            // Loop to listen for commands.
            // It will send the commands to shell and catch the output.
            loop {
                if let Some(mut stdin) = handle.stdin.take(){
                    if let Ok(cmd) = sti_receiver.recv(){
                        println!("[!] command received.");
                        stdin.write_all(&cmd).unwrap();
                    }
                }

                if let Some(mut stdout) = handle.stdout.take(){
                    let mut buf: Vec<u8> = Vec::new();
                    stdout.read_to_end(&mut buf).unwrap();
                    println!("[!] Sending output.");
                    sto_sender.send(buf).unwrap();
                }
                thread::sleep(std::time::Duration::from_millis(200));
            }
        });

        // Thread that continuously asks user for his input,
        // This input is given as commands to the interactive shell using above sti channels.
        loop {
            println!("[!] Handler thread started.");
            let cmd: String = input::<String>().msg("$ ").get();
            sti_sender.send(cmd.into_bytes())?;
            println!("[!] command sent.");
            if let Ok(out) = sto_receiver.recv() {
                println!("{}", String::from_utf8(out).unwrap());
            }
        }
        // Ok(())
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
