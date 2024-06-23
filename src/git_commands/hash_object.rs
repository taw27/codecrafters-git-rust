use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::git_commands::utils::{compress_and_write_file, get_sha, ObjectPathGetter};
use crate::models::git_object::{GitObject, GetContent};
use crate::models::object::Object;

pub fn hash_object<O: ObjectPathGetter, W: Write>(
    file_path: &str,
    flag: &Option<&str>,
    object_path_getter: O,
    writer: &mut W,
) -> Result<(), String> {
    let should_write = match flag {
        Some("-w") => true,
        None => false,
        _ => return Err("flag not recognized, Available flags: -w".to_string()),
    };
    let mut file = File::open(file_path).map_err(|err| format!("error opening file: {}", err))?;
    let mut contents: Vec<u8> = Vec::new();

    let content_size = file
        .read_to_end(&mut contents)
        .map_err(|err| format!("error opening file: {}", err))?;

    let git_object = GitObject::new(content_size as i32, Object::new("blob", contents)?);
    let object_file_string = git_object.get_content()?;
    let sha = get_sha(&object_file_string)
        .map_err(|err| format!("error constructing sha from contents: {}", err))?;

    writer.write_all(sha.as_bytes()).expect("error writing sha");

    if !should_write {
        return Ok(());
    }

    let object_file_path = object_path_getter
        .get_object_path(sha.as_str())
        .map_err(|err| err.to_string())?;

    compress_and_write_file(object_file_path.as_str(), object_file_string.as_str())
        .map_err(|err| format!("error writing object: {} {}", err, object_file_path))?;

    Ok(())
}
