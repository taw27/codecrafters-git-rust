use core::str;

use crate::models::blob::Blob;
use crate::models::git_object::GetContent;
use crate::models::tree::Tree;

#[derive(Debug, PartialEq)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
}

impl Object {
    pub fn new(type_str: &str, content: Vec<u8>) -> Result<Self, String> {
        let object_type = match type_str {
            "blob" => Object::Blob(Blob::new(content)),
            "tree" => Object::Tree(Tree::new(content)?),
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

impl GetContent for Object {
    fn get_content(&self) -> Result<String, String> {
        match self {
            Object::Blob(blob) => blob.get_content(),
            Object::Tree(tree) => tree.get_content(),
        }
    }
}
