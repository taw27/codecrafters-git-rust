use std::io::Write;

use crate::git_commands::utils::{ObjectPathGetter, read_and_decompress_file};
use crate::models::git_object::{GetContentString, GitObject};
use crate::models::object::Object;

pub fn ls_tree<O: ObjectPathGetter, W: Write>(
    sha: &str,
    flag: &Option<&str>,
    object_path_getter: O,
    writer: &mut W,
) -> Result<(), String> {
    let name_only = match flag {
        Some("--name-only") => true,
        None => false,
        _ => return Err("flag no recognized. Available flags: --name-only".to_string()),
    };
    let object_path = object_path_getter.get_object_path(sha)?;
    let decompressed_content =
        read_and_decompress_file(&object_path.as_str()).map_err(|e| e.to_string())?;

    let git_object = GitObject::from_object_file_buffer(&decompressed_content)?;

    let to_print: String = match git_object.object {
        Object::Blob(_) => return Err("not a tree object".to_string()),
        Object::Tree(tree) => {
            if name_only {
                tree.get_names()
            } else {
                tree.get_content_string()?
            }
        }
    };

    writer
        .write_all(to_print.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
