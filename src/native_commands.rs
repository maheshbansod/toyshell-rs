use std::str::FromStr;

use color_eyre::Result;

pub struct NativeResult {
    pub exit: bool,
}

pub enum NativeCommand {
    Ls,
    Exit
}

pub type NativeFullCommand<'a> = (NativeCommand, Vec<&'a str>);

impl FromStr for NativeCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "exit" => Ok(NativeCommand::Exit),
            "ls" => Ok(NativeCommand::Ls),
            _ => Err("Not a native command".to_owned())
        }
    }
}

pub fn run_native(cmd: NativeFullCommand) -> Result<NativeResult> {
    match cmd {
        (NativeCommand::Ls, args) => {
            println!("LS: {:?}", args);
            Ok(NativeResult { exit: false })
        }
        (NativeCommand::Exit, _args) => {
            println!("exit");
            Ok(NativeResult { exit: true })
            // inter process communication needed?
        }
    }
}
