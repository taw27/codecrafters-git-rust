use std::str;

use crate::models::git_object::GetContent;

#[derive(Debug, PartialEq)]
pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

impl GetContent for Blob {
    fn get_content(&self) -> Result<String, String> {
        Ok(str::from_utf8(&self.content)
            .map_err(|err| format!("error parsing blob content: {}", err))?
            .to_string())
    }
}
