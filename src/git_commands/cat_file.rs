use std::io::Write;

use crate::git_commands::utils::{get_object_path, read_and_decompress_file};
use crate::models::GitObject;

pub trait ObjectPathGetter {
    fn get_object_path(&self, sha: &str) -> Result<String, &'static str>;
}

pub struct ActualObjectPathGetter {}

impl ObjectPathGetter for ActualObjectPathGetter {
    fn get_object_path(&self, sha: &str) -> Result<String, &'static str> {
        get_object_path(sha)
    }
}

pub fn cat_file<O: ObjectPathGetter, W: Write>(
    sha: &str,
    flag: &str,
    object_path_getter: O,
    writer: &mut W,
) -> Result<(), String> {
    if flag != "-p" {
        return Err("flag not recognized. Available flags: -p".to_string());
    }

    let object_path = object_path_getter.get_object_path(sha)?;
    let decompressed_content = match read_and_decompress_file(&object_path.as_str()) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    let git_object = GitObject::from_object_file_string(decompressed_content.as_str())?;

    writer
        .write_all(git_object.content.as_bytes())
        .expect("error writing content");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};

    use flate2::bufread::ZlibEncoder;
    use flate2::Compression;

    use super::*;

    #[test]
    pub fn cat_file_prints_content() {
        let content = "blob 123\0Hello, world!";
        let mut encoder = ZlibEncoder::new(content.as_bytes(), Compression::default());
        let mut compressed_content = Vec::new();

        encoder.read_to_end(&mut compressed_content).unwrap();

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(&compressed_content).unwrap();

        let temp_file_path = temp_file.path().to_str().unwrap();

        let mut writer = Vec::new();

        struct MockObjectPathGetter {
            file_path: String,
        }
        impl ObjectPathGetter for MockObjectPathGetter {
            fn get_object_path(&self, _sha: &str) -> Result<String, &'static str> {
                Ok(self.file_path.clone())
            }
        }

        cat_file(
            temp_file_path,
            "-p",
            MockObjectPathGetter {
                file_path: temp_file_path.to_string(),
            },
            &mut writer,
        )
        .unwrap();

        let expected = "Hello, world!";

        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    #[test]
    pub fn cat_file_fails_with_invalid_flag() {
        let mut writer = Cursor::new(Vec::new());

        struct MockObjectPathGetter {
            file_path: String,
        }
        impl ObjectPathGetter for MockObjectPathGetter {
            fn get_object_path(&self, _sha: &str) -> Result<String, &'static str> {
                Ok(self.file_path.clone())
            }
        }

        let result = cat_file(
            "some_sha",
            "invalid_flag",
            MockObjectPathGetter {
                file_path: "some_path".to_string(),
            },
            &mut writer,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "flag not recognized. Available flags: -p"
        );
    }

    #[test]
    pub fn cat_file_fails_with_invalid_sha() {
        let mut writer = Cursor::new(Vec::new());

        struct MockObjectPathGetter {
            file_path: String,
        }
        impl ObjectPathGetter for MockObjectPathGetter {
            fn get_object_path(&self, _sha: &str) -> Result<String, &'static str> {
                Err("invalid sha")
            }
        }

        let result = cat_file(
            "invalid_sha",
            "-p",
            MockObjectPathGetter {
                file_path: "some_path".to_string(),
            },
            &mut writer,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid sha");
    }

    #[test]
    pub fn cat_file_fails_with_invalid_file_path() {
        let mut writer = Cursor::new(Vec::new());

        struct MockObjectPathGetter {
            file_path: String,
        }
        impl ObjectPathGetter for MockObjectPathGetter {
            fn get_object_path(&self, _sha: &str) -> Result<String, &'static str> {
                Ok(self.file_path.clone())
            }
        }

        let result = cat_file(
            "some_sha",
            "-p",
            MockObjectPathGetter {
                file_path: "invalid_path".to_string(),
            },
            &mut writer,
        );

        assert!(result.is_err());
    }
}
