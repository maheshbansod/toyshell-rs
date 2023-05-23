use std::{io::{stdin, self, Write}, println};

use color_eyre::Result;
use toyshell::{ShellConfig, parse_input, process_command, RetStatus};


fn main() -> Result<()> {
    let shell_config = ShellConfig {
        prompt: "$ ".to_string()
    };
    let mut alive = true;
    while alive {
        print!("{}", shell_config.prompt);
        io::stdout().flush()?;
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Error reading user input");
        let cmd = parse_input(&buf);
        match process_command(cmd) {
            Ok(RetStatus { exit, message }) if exit && message.is_none() => alive = false,
            Ok(RetStatus { exit, message }) if exit => {
                // case where alive = false and message exists (happens when child
                // process didn't run)
                let message = message.unwrap();
                println!("{message}");
                alive = false;
            },
            Err(_) => println!("Error occurred"),
            _ => (),
        }
    }
    Ok(())
}
