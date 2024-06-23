use std::str;

use crate::models::git_object::PrintContent;

#[derive(Debug, PartialEq)]
pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

impl PrintContent for Blob {
    fn print_content(&self) -> Result<String, String> {
        Ok(str::from_utf8(&self.content)
            .map_err(|err| format!("error parsing blob content: {}", err))?
            .to_string())
    }
}
