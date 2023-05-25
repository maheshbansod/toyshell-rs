use std::{format, fs::read_to_string, str::FromStr};

use color_eyre::Result;

use crate::RetStatus;

fn get_usage(cmd: &str) -> RetStatus {
    RetStatus {
        exit: false,
        message: Some(format!("Usage: {cmd} <f|a|c> <filepath> <pattern>")),
    }
}

#[derive(PartialEq)]
enum Mode {
    First,
    All,
    Count,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "f" => Ok(Mode::First),
            "a" => Ok(Mode::All),
            "c" => Ok(Mode::Count),
            _ => Err("Not a mode for this commmand".to_owned()),
        }
    }
}

pub fn run_search_command(args: Vec<&str>) -> Result<RetStatus> {
    let cmd = args[0];
    if args.len() != 4 {
        return Ok(get_usage(cmd));
    }
    let mode = args[1].parse();
    if mode.is_err() {
        return Ok(get_usage(cmd));
    }
    let mode: Mode = mode.unwrap();

    let filepath = args[2];
    let pattern = args[3];

    let file = read_to_string(filepath)?;
    let lines = file.lines();
    let mut matches = vec![];
    for (i, line) in lines.enumerate() {
        if line.contains(pattern) {
            matches.push((i, line));
            if mode == Mode::First {
                break;
            }
        }
    }
    let message = match mode {
        Mode::Count => format!("{}", matches.len()),
        _ => matches
            .iter()
            .map(|(i, line)| format!("{}: {}", i + 1, line))
            .collect::<Vec<_>>()
            .join("\n"),
    };
    Ok(RetStatus {
        exit: false,
        message: Some(message),
    })
}
