use std::io::stdout;

use utils::ActualObjectPathGetter;

use crate::git_commands::cat_file::cat_file;
use crate::git_commands::GitCommand::{CatFile, HashObject, Init, LsTree};
use crate::git_commands::hash_object::hash_object;
use crate::git_commands::init::init;
use crate::git_commands::ls_tree::ls_tree;

mod cat_file;
mod hash_object;
mod init;
mod ls_tree;
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
    LsTree {
        sha: &'a str,
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
                    return Err("usage: git cat-file -p <object_sha>".to_string());
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
            "ls-tree" => {
                let arg_len = args.len();

                if arg_len < 3 && arg_len > 4 {
                    return Err("usage: git ls-tree [--name-only] <tree_sha>".to_string());
                }

                if arg_len == 3 {
                    Ok(LsTree {
                        sha: args[2].as_str(),
                        flag: None,
                    })
                } else {
                    Ok(LsTree {
                        sha: args[3].as_str(),
                        flag: Some(args[2].as_str()),
                    })
                }
            }
            _ => Err("not a recognized git command".to_string()),
        }
    }

    pub fn execute(&self) {
        let error: Result<(), String> = match self {
            Init => {
                init();
                Ok(())
            }
            CatFile { sha, flag } => cat_file(sha, flag, ActualObjectPathGetter {}, &mut stdout()),
            HashObject { file_path, flag } => {
                hash_object(file_path, flag, ActualObjectPathGetter {}, &mut stdout())
            }
            LsTree { sha, flag } => ls_tree(sha, flag, ActualObjectPathGetter {}, &mut stdout()),
        };

        match error {
            Err(e) => print!("{}", e.to_string()),
            _ => {}
        }
    }
}
