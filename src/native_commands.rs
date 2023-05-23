use std::{str::FromStr, println};

use color_eyre::Result;
use nix::unistd::chdir;

pub struct NativeResult {
    pub exit: bool,
}

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

pub fn run_native(cmd: NativeFullCommand) -> Result<NativeResult> {
    match cmd {
        (NativeCommand::List, args) => {
            println!("LS: {:?}", args);
            Ok(NativeResult { exit: false })
        }
        (NativeCommand::Exit, _args) => {
            println!("exit");
            Ok(NativeResult { exit: true })
        }
        (NativeCommand::ChangeDirectory, args) => {
            if args.len() > 2 {
                println!("Too many arguments");
            } else if chdir(args[1]).is_err() {
                println!("Couldn't change directory");
            }
            Ok(NativeResult { exit: false })
        }
    }
}
