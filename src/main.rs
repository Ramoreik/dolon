mod shell;

use read_input::prelude::*;
use std::error::Error;
use shell::Shell;


fn main() -> Result<(), Box<dyn Error>> {
    let shell: Shell = Shell::new();
    shell.init();
    // shell.interactive_session()?;
    Ok(())
}
