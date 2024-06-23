use core::str;

use crate::models::object::Object;

pub struct GitObject {
    pub size: i32,
    pub object: Object,
}

impl GitObject {
    pub fn new(size: i32, object_type: Object) -> Self {
        Self {
            size,
            object: object_type,
        }
    }

    pub fn from_object_file_buffer(file_buffer: &Vec<u8>) -> Result<Self, String> {
        let null_position = file_buffer
            .iter()
            .position(|&x| x == 0)
            .ok_or_else(|| "not a valid git object file".to_string())?;
        let header = &file_buffer[0..null_position];
        let content = &file_buffer[null_position + 1..];
        let header_str = str::from_utf8(header)
            .map_err(|err| format!("error parsing git object header: {}", err))?;
        let header_parts: Vec<&str> = header_str.split(" ").collect();

        Ok(Self {
            size: match header_parts[1].parse() {
                Ok(s) => s,
                Err(err) => return Err(format!("error parsing object size: {}", err)),
            },
            object: Object::new(header_parts[0], content.to_vec())?,
        })
    }
}
impl GetContent for GitObject {
    fn get_content(&self) -> Result<String, String> {
        self.object.get_content()
    }
}

pub trait GetContent {
    fn get_content(&self) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_object_file_buffer_parses_valid_blob_object() {
        let file_string = "blob 6\0content".to_string();
        let git_object = GitObject::from_object_file_buffer(&file_string.into_bytes()).unwrap();

        assert_eq!(git_object.size, 6);
        assert_eq!(git_object.object.get_type(), "blob");
        assert_eq!(&git_object.get_content().unwrap(), "content");
    }

    #[test]
    fn from_object_file_string_returns_error_for_invalid_object_file_string() {
        let file_string = "invalid".to_string();
        let result = GitObject::from_object_file_buffer(&file_string.into_bytes());

        assert!(result.is_err());
    }

    #[test]
    fn from_object_file_string_returns_error_for_object_file_string_with_invalid_size() {
        let file_string = "blob invalid\0content".to_string();
        let result = GitObject::from_object_file_buffer(&file_string.into_bytes());

        assert!(result.is_err());
    }
}
