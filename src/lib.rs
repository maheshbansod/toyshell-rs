use std::{ffi::CString, println};

use color_eyre::Result;
use native_commands::run_native;
use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::{execv, fork, ForkResult},
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
    (tokens[0], tokens[1..].to_vec())
}

pub struct RetStatus {
    pub exit: bool,
    pub message: Option<String>,
}

pub fn process_command(cmd: ShellCommand) -> Result<RetStatus> {
    if let Ok(main_cmd) = cmd.0.parse() {
        let res = run_native((main_cmd, cmd.1))?;
        return Ok(RetStatus {
            exit: res.exit,
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
            if execv(&cmd, &args).is_err() {
                return Ok(RetStatus {
                    exit: true,
                    message: Some(format!("Error running command.")),
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
