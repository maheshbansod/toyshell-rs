use std::{fs::read_to_string, str::FromStr};

use color_eyre::Result;

use crate::RetStatus;

enum Mode {
    /// list all lines
    All,
    /// list n lines from the start
    FromStart { n: usize },
    /// list n lines from the end
    FromEnd { n: usize },
}

fn arg_to_num(arg: &str) -> std::result::Result<usize, String> {
    if let Ok(n) = arg.parse() {
        Ok(n)
    } else {
        Err("Not a valid number".to_owned())
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "a" {
            return Ok(Mode::All);
        }
        if let Some(n) = s.strip_prefix('+') {
            return Ok(Mode::FromStart { n: arg_to_num(n)? });
        }
        if let Some(n) = s.strip_prefix('-') {
            return Ok(Mode::FromEnd { n: arg_to_num(n)? });
        }
        Err("Not a valid mode".to_owned())
    }
}

fn get_usage(cmd: &str) -> String {
    format!("{} <a|+n|-n> <file>", cmd)
}

pub fn run_typeline_command(args: Vec<&str>) -> Result<RetStatus> {
    if args.len() != 3 {
        return Ok(RetStatus {
            exit: false,
            message: Some(get_usage(args[0])),
        });
    }
    let mode = args[1].parse();
    if mode.is_err() {
        return Ok(RetStatus {
            exit: false,
            message: Some(get_usage(args[0])),
        });
    }
    let mode = mode.unwrap();
    let path = args[2];

    let file = read_to_string(path)?;
    let lines = file.lines();

    let lines: Vec<&str> = match mode {
        Mode::All => lines.collect(),
        Mode::FromStart { n } => lines.take(n).collect(),
        Mode::FromEnd { n } => {
            let lines: Vec<&str> = lines.rev().take(n).collect();
            lines.into_iter().rev().collect()
        }
    };
    let message = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let lno = i + 1;
            format!("{lno}: {line}")
        })
        .collect::<Vec<String>>()
        .join("\n");
    let message = format!("{path}:\n{message}");
    Ok(RetStatus {
        exit: false,
        message: Some(message),
    })
}
