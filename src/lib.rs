use color_eyre::Result;
use nix::{sys::wait::{waitpid, WaitStatus}, unistd::{ForkResult, fork}};


pub struct ShellConfig {
    pub prompt: String,
}

type ShellCommand<'a> = (&'a str, Vec<&'a str>);
pub fn parse_input(input: &str) -> ShellCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return ("", vec![])
    }
    (tokens[0], tokens[1..].to_vec())
}

pub struct NativeResult {
    pub exit: bool,
}

pub fn run_native(cmd: ShellCommand) -> Result<NativeResult> {
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

pub enum RetStatus {
    NativeStatus(NativeResult),
    ExtStatus { stat: i32 },
}

pub fn process_command(cmd: ShellCommand) -> Result<RetStatus> {
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


