use std::str;

use crate::models::git_object::GetContentString;

#[derive(Debug, PartialEq)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

impl GetContentString for Blob {
    fn get_content_string(&self) -> Result<String, String> {
        Ok(str::from_utf8(&self.content)
            .map_err(|err| format!("error parsing blob content: {}", err))?
            .to_string())
    }
}
