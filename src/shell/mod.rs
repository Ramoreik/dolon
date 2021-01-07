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

    pub fn interactive_session(&self) -> Result<(), Box<dyn Error>> {
        let (sti_sender, sti_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (sto_sender, sto_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let hndl = std::thread::spawn(move || {
            let mut handle: Child = Command::new("bash")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process");
                let mut stdin = handle.stdin.unwrap();
                let mut w_stdin = BufWriter::new(&mut stdin);
            loop {
                println!("[!] Shell thread started.");
                // if stdin is Some, write command?
                    if let Ok(cmd) = sti_receiver.recv(){
                        println!("[!] command received.");
                        w_stdin.write_all(&cmd).unwrap();
                    }
                // get stdout, create buf, read into buf
                if let Some(mut stdout) = handle.stdout.take(){
                    let mut buf: Vec<u8> = Vec::new();
                    stdout.read_to_end(&mut buf).unwrap();
                    println!("[!] Sending output.");
                    sto_sender.send(buf).unwrap();
                }
                thread::sleep(std::time::Duration::from_millis(200));
            }
        });
        loop {
            println!("[!] Handler thread started.");
            // let cmd: String = input::<String>().msg("$").get();
            // sti_sender.send(cmd.into_bytes())?;
            sti_sender.send("ls".to_owned().into_bytes())?;
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
