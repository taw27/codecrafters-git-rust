#[cfg(feature = "wip")]
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
    let git_object = GitObject::from_object_file_buffer(decompressed_content.as_str())?;

    writer
        .write_all(git_object.content.as_bytes())
        .expect("error writing content");

    Ok(())
}
