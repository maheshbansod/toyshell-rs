use std::{fs, str::FromStr};

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
            "list" => Ok(NativeCommand::List),
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
            let mut total_ls_output = String::new();
            // TODO: modes + refactor maybe
            enum _ListMode {
                All,
                Files,
                Count
            }
            let consumed_args = 1;
            let dirs = if args.len() > 1 {
                args.into_iter().skip(consumed_args).collect()
            } else {
                vec!["."]
            };
            for dir in dirs {
                let paths = fs::read_dir(dir)?;
                let ls_output = paths
                    .into_iter()
                    .filter(|p| {
                        if let Ok(p) = p {
                            if let Ok(f) = p.file_type() {
                                return f.is_file();
                            }
                        }
                        return false;
                    })
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                let ls_output = ls_output
                    .iter()
                    .map(|dir_entry| dir_entry.path().display().to_string())
                    .collect::<Vec<_>>().join(" ");
                total_ls_output += &ls_output;
            }
            Ok(RetStatus {
                exit: false,
                message: Some(total_ls_output),
            })
        }
        (NativeCommand::Exit, _args) => Ok(RetStatus {
            exit: true,
            message: Some("exit".to_string()),
        }),
        (NativeCommand::ChangeDirectory, args) => {
            if args.len() != 2 {
                Ok(RetStatus {
                    exit: false,
                    message: Some("Usage: cd <DIR>".to_string()),
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
