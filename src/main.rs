use std::{io::{stdin, self, Write}, println};

use color_eyre::Result;
use nix::{unistd::{fork, ForkResult}, sys::wait::{waitpid, WaitStatus}};



struct ShellConfig {
    prompt: String,
}

type ShellCommand<'a> = (&'a str, Vec<&'a str>);

fn parse_input(input: &str) -> ShellCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return ("", vec![])
    }
    (tokens[0], tokens[1..].to_vec())
}

struct NativeResult {
    exit: bool,
}

fn run_native(cmd: ShellCommand) -> Result<NativeResult> {
    match cmd {
        ("ls", args) => {
            println!("LS: {:?}", args);
            Ok(NativeResult { exit: false })
        }
        ("exit", _args) => {
            println!("exit");
            Ok(NativeResult { exit: true })
            // inter process communication needed?
        }
        (cmd, args) => {
            println!("cmd: {}, args: {:?}", cmd, args);
            Ok(NativeResult { exit: false })
        }
    }
}

enum RetStatus {
    NativeStatus(NativeResult),
    ExtStatus { stat: i32 },
}

fn process_command(cmd: ShellCommand) -> Result<RetStatus> {
    if true {
        let res = run_native(cmd)?;
        return Ok(RetStatus::NativeStatus(res))
    }
    match unsafe {fork()} {
        Ok(ForkResult::Child) => {
            // exec the command
        }
        Ok(ForkResult::Parent { child }) => {
            while waitpid(child, None)? == WaitStatus::StillAlive {

            }
        }
        Err(_) => println!("Failed to fork")
    }
    Ok(RetStatus::ExtStatus { stat: 0 })
}

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
            Ok(RetStatus::NativeStatus(res)) if res.exit => alive = false,
            Err(_) => println!("Error occurred"),
            _ => (),
        }
    }
    Ok(())
}
