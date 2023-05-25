use color_eyre::Result;
use nix::unistd::chdir;

use crate::RetStatus;

pub fn run_cd_command(args: Vec<&str>) -> Result<RetStatus> {
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
