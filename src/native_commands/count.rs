use std::{fs::read_to_string, str::FromStr};

use color_eyre::Result;

use crate::RetStatus;

fn get_usage(cmd: &str) -> RetStatus {
    RetStatus {
        exit: false,
        message: Some(format!("Usage: {cmd} <w|c|l> <filename>")),
    }
}

enum Mode {
    Characters,
    Words,
    Lines,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "c" => Ok(Self::Characters),
            "w" => Ok(Self::Words),
            "l" => Ok(Self::Lines),
            _ => Err("Not a mode for this command".to_owned()),
        }
    }
}

pub fn run_count_command(args: Vec<&str>) -> Result<RetStatus> {
    let cmd = args[0];
    if args.len() != 3 {
        return Ok(get_usage(cmd));
    }

    let mode = args[1].parse();
    if mode.is_err() {
        return Ok(get_usage(cmd));
    }
    let mode: Mode = mode.unwrap();
    let path = args[2];
    let file = read_to_string(path)?;
    let count = match mode {
        Mode::Lines => file.lines().collect::<Vec<_>>().len(),
        Mode::Words => file.split_whitespace().collect::<Vec<_>>().len(),
        Mode::Characters => file.len(),
    };
    Ok(RetStatus {
        exit: false,
        message: Some(count.to_string()),
    })
}
