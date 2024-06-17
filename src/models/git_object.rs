use core::str;

use crate::models::tree_object::TreeObject;

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
        let (header, content) = file_buffer.split_at(null_position);
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
impl PrintContent for GitObject {
    fn print_content(&self) -> &String {
        self.object.print_content()
    }
}

#[derive(Debug, PartialEq)]
struct Blob {
    content: String,
}

impl Blob {
    fn from_buffer(content: &Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            content: str::from_utf8(content)
                .map_err(|err| format!("error parsing blob content: {}", err))?
                .to_string(),
        })
    }
    fn print_content(&self) -> &String {
        &self.content
    }
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Blob(Blob),
    Tree(TreeObject),
}

pub trait PrintContent {
    fn print_content(&self) -> &String;
}

impl Object {
    pub fn new(type_str: &str, content: Vec<u8>) -> Result<Self, String> {
        let object_type = match type_str {
            "blob" => Object::Blob(Blob::from_buffer(&content)?),
            "tree" => Object::Tree(TreeObject::new()),
            _ => return Err(format!("Object type not recognized: {}", type_str)),
        };

        Ok(object_type)
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Object::Blob(_) => "blob",
            Object::Tree(_) => "tree",
        }
    }
}

impl PrintContent for Object {
    fn print_content(&self) -> &String {
        match self {
            Object::Blob(blob) => blob.print_content(),
            _ => panic!("Not implemented"),
        }
    }
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
        assert_eq!(git_object.print_content(), "content");
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
