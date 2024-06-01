use std::io::stdout;

use crate::git_commands::cat_file::{ActualObjectPathGetter, cat_file};
use crate::git_commands::GitCommand::{CatFile, Init};
use crate::git_commands::init::init;

mod cat_file;
mod init;
mod utils;

pub enum GitCommand<'a> {
    CatFile { sha: &'a str, flag: &'a str },
    Init,
}

impl<'a> GitCommand<'a> {
    pub fn from_args(args: &'a Vec<String>) -> Result<Self, String> {
        match args[1].as_str() {
            "init" => Ok(Init {}),
            "cat-file" => {
                if args.len() < 4 {
                    return Err("usage: git cat-file -p <object>".to_string());
                }

                Ok(CatFile {
                    flag: args[2].as_str(),
                    sha: args[3].as_str(),
                })
            }
            _ => Err("not a recognized git command".to_string()),
        }
    }

    pub fn execute(&self) {
        match self {
            Init => init(),
            CatFile { sha, flag } => {
                cat_file(sha, flag, ActualObjectPathGetter {}, &mut stdout()).unwrap()
            }
        }
    }
}
