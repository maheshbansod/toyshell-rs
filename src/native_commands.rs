use std::str::FromStr;

use color_eyre::Result;
use nix::unistd::chdir;

pub enum NativeCommand {
    ChangeDirectory,
    Exit,
    List,
}

pub type NativeFullCommand<'a> = (NativeCommand, Vec<&'a str>);

impl FromStr for NativeCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "cd" => Ok(NativeCommand::ChangeDirectory),
            "exit" => Ok(NativeCommand::Exit),
            "ls" => Ok(NativeCommand::List),
            _ => Err("Not a native command".to_owned()),
        }
    }
}

pub struct RetStatus {
    pub exit: bool,
    pub message: Option<String>,
}

pub fn run_native(cmd: NativeFullCommand) -> Result<RetStatus> {
    match cmd {
        (NativeCommand::List, args) => {
            let ls_output = format!("LS: {:?}", args);
            Ok(RetStatus {
                exit: false,
                message: Some(ls_output),
            })
        }
        (NativeCommand::Exit, _args) => Ok(RetStatus {
            exit: true,
            message: Some("exit".to_string()),
        }),
        (NativeCommand::ChangeDirectory, args) => {
            if args.len() > 2 {
                Ok(RetStatus {
                    exit: false,
                    message: Some("Too many arguments".to_string()),
                })
            } else if chdir(args[1]).is_err() {
                Ok(RetStatus {
                    exit: false,
                    message: Some("Couldn't change directory".to_string()),
                })
            } else {
                Ok(RetStatus {
                    exit: false,
                    message: None,
                })
            }
        }
    }
}
