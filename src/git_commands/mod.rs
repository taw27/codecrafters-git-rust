use std::io::stdout;

use utils::ActualObjectPathGetter;

use crate::git_commands::cat_file::cat_file;
use crate::git_commands::GitCommand::{CatFile, HashObject, Init};
use crate::git_commands::hash_object::hash_object;
use crate::git_commands::init::init;

mod cat_file;
mod hash_object;
mod init;
mod utils;

pub enum GitCommand<'a> {
    CatFile {
        sha: &'a str,
        flag: &'a str,
    },
    HashObject {
        file_path: &'a str,
        flag: Option<&'a str>,
    },
    Init,
}

impl<'a> GitCommand<'a> {
    pub fn from_args(args: &'a Vec<String>) -> Result<Self, String> {
        match args[1].as_str() {
            "init" => Ok(Init {}),
            "cat-file" => {
                if args.len() != 4 {
                    return Err("usage: git cat-file -p <object>".to_string());
                }

                Ok(CatFile {
                    flag: args[2].as_str(),
                    sha: args[3].as_str(),
                })
            }
            "hash-object" => {
                let arg_len = args.len();

                if arg_len < 3 && arg_len > 4 {
                    return Err("usage: git hash-object [-w] <file_path>".to_string());
                }

                if arg_len == 3 {
                    Ok(HashObject {
                        file_path: args[2].as_str(),
                        flag: None,
                    })
                } else {
                    Ok(HashObject {
                        file_path: args[3].as_str(),
                        flag: Some(args[2].as_str()),
                    })
                }
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
            HashObject { file_path, flag } => {
                hash_object(file_path, flag, ActualObjectPathGetter {}, &mut stdout()).unwrap();
            }
        }
    }
}
