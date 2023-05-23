use std::{
    io::{self, stdin, Write},
    println,
};

use color_eyre::Result;
use toyshell::{parse_input, process_command, ShellConfig};

fn main() -> Result<()> {
    let shell_config = ShellConfig {
        prompt: "$ ".to_string(),
    };
    let mut alive = true;
    while alive {
        print!("{}", shell_config.prompt);
        io::stdout().flush()?;
        let mut buf = String::new();
        stdin()
            .read_line(&mut buf)
            .expect("Error reading user input");
        let cmd = parse_input(&buf);
        match process_command(cmd) {
            Ok(ret_status) => {
                if let Some(message) = ret_status.message {
                    println!("{message}");
                }
                alive = !ret_status.exit;
            }
            Err(_) => println!("Error occurred"),
        }
    }
    Ok(())
}
