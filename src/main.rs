use std::env;

use crate::git_commands::GitCommand;

mod git_commands;
mod models;

fn main() {
    let args: Vec<String> = env::args().collect();

    GitCommand::from_args(&args).unwrap().execute()
}
