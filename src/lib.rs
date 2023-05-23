use std::{ffi::CString, println};

use color_eyre::Result;
use native_commands::run_native;
pub use native_commands::RetStatus;
use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::{execvp, fork, ForkResult},
};
mod native_commands;

pub struct ShellConfig {
    pub prompt: String,
}

type ShellCommand<'a> = (&'a str, Vec<&'a str>);
pub fn parse_input(input: &str) -> ShellCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return ("", vec![]);
    }
    (tokens[0], tokens.to_vec())
}

pub fn process_command(cmd: ShellCommand) -> Result<RetStatus> {
    if let Ok(main_cmd) = cmd.0.parse() {
        return run_native((main_cmd, cmd.1));
    }
    if cmd.0.trim() == "" {
        return Ok(RetStatus {
            exit: false,
            message: None,
        });
    }
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let (cmd, args) = cmd;
            let cmd = CString::new(cmd)?;
            let args: Vec<_> = args
                .into_iter()
                .map(CString::new)
                .collect::<std::result::Result<_, _>>()?;
            if execvp(&cmd, &args).is_err() {
                return Ok(RetStatus {
                    exit: true,
                    message: Some("Error running command.".to_string()),
                });
            }
        }
        Ok(ForkResult::Parent { child }) => {
            while waitpid(child, None)? == WaitStatus::StillAlive {}
        }
        Err(_) => println!("Failed to fork"),
    }
    Ok(RetStatus {
        exit: false,
        message: None,
    })
}
