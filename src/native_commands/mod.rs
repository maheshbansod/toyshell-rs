use std::str::FromStr;
mod cd;
mod count;
mod list;
mod search;
mod typeline;

use color_eyre::Result;

use self::{
    cd::run_cd_command, count::run_count_command, list::run_list_command,
    search::run_search_command, typeline::run_typeline_command,
};

pub enum NativeCommand {
    ChangeDirectory,
    Count,
    Exit,
    List,
    Search,
    Typeline,
}

pub type NativeFullCommand<'a> = (NativeCommand, Vec<&'a str>);

impl FromStr for NativeCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "cd" => Ok(NativeCommand::ChangeDirectory),
            "count" => Ok(NativeCommand::Count),
            "exit" => Ok(NativeCommand::Exit),
            "list" => Ok(NativeCommand::List),
            "search" => Ok(NativeCommand::Search),
            "typeline" => Ok(NativeCommand::Typeline),
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
        (NativeCommand::List, args) => run_list_command(args),
        (NativeCommand::Exit, _args) => Ok(RetStatus {
            exit: true,
            message: Some("exit".to_string()),
        }),
        (NativeCommand::ChangeDirectory, args) => run_cd_command(args),
        (NativeCommand::Count, args) => run_count_command(args),
        (NativeCommand::Search, args) => run_search_command(args),
        (NativeCommand::Typeline, args) => run_typeline_command(args),
    }
}
