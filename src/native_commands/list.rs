use std::{fs, path::PathBuf, str::FromStr};

use color_eyre::Result;

use crate::RetStatus;

enum ListMode {
    All,
    Files,
    Count,
}

impl FromStr for ListMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "a" => Ok(ListMode::All),
            "f" => Ok(ListMode::Files),
            "c" => Ok(ListMode::Count),
            _ => Err("Not a ListMode".to_string()),
        }
    }
}

struct ListCmdOutput {
    path: String,
    result: Vec<PathBuf>,
}

fn list_files(paths: Vec<&str>, list_mode: &ListMode) -> Result<Vec<ListCmdOutput>> {
    let mut result = vec![];
    for path in paths {
        let files = fs::read_dir(path)?;
        let mut ls_files = vec![];
        for file in files {
            if file.is_err() {
                // ignoring inaccessible files
                continue;
            }
            let file = file.unwrap();
            match list_mode {
                ListMode::Files => {
                    if let Ok(ftype) = file.file_type() {
                        if ftype.is_file() {
                            ls_files.push(file.path());
                        }
                    }
                }
                _ => {
                    ls_files.push(file.path());
                }
            }
        }
        result.push(ListCmdOutput {
            path: path.to_string(),
            result: ls_files,
        });
    }
    Ok(result)
}

pub fn run_list_command(args: Vec<&str>) -> Result<RetStatus> {
    let mut consumed_args = 1;
    let list_mode = if args.len() > 1 {
        if let Ok(list_mode) = args[1].parse() {
            consumed_args += 1;
            list_mode
        } else {
            ListMode::All
        }
    } else {
        ListMode::All
    };
    let dirs = if args.len() - consumed_args > 0 {
        args.into_iter().skip(consumed_args).collect()
    } else {
        vec!["."]
    };
    match list_files(dirs, &list_mode) {
        Ok(result) => {
            let nfiles = result.len();
            let message = match list_mode {
                ListMode::Count => result
                    .into_iter()
                    .map(|lsop| {
                        let prefix = if nfiles > 1 {
                            format!("{}:\n", lsop.path)
                        } else {
                            String::new()
                        };
                        format!("{}{}", prefix, lsop.result.len())
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
                _ => result
                    .into_iter()
                    .map(|lsop| {
                        let prefix = if nfiles > 1 {
                            format!("{}:\n", lsop.path)
                        } else {
                            String::new()
                        };
                        let lsop = lsop
                            .result
                            .iter()
                            .map(|p| format!("{}", p.display()))
                            .collect::<Vec<String>>()
                            .join(" ");
                        format!("{}{}", prefix, lsop)
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            };
            Ok(RetStatus {
                exit: false,
                message: Some(message),
            })
        }
        Err(err) => Ok(RetStatus {
            exit: false,
            message: Some(format!("Error in list: {}", err)),
        }),
    }
}
