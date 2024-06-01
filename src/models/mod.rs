pub struct GitObject {
    pub size: i32,
    pub object_type: String,
    pub content: String,
}

impl GitObject {
    pub fn from_object_file_string(file_string: &str) -> Result<Self, String> {
        let split_file_string: Vec<&str> = file_string.split('\0').collect();

        if split_file_string.len() < 2 {
            return Err(format!("not a valid object file: {}", file_string));
        }

        let non_content_parts: Vec<&str> = split_file_string[0].split(' ').collect();

        if split_file_string.len() != 2 {
            return Err(format!("not a valid object file: {}", file_string));
        }

        Ok(Self {
            size: match non_content_parts[1].parse() {
                Ok(s) => s,
                Err(err) => return Err(format!("error parsing object size: {}", err)),
            },
            object_type: non_content_parts[0].to_string(),
            content: split_file_string[1].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_object_file_string_parses_valid_object_file_string() {
        let file_string = "blob 6\0content";
        let git_object = GitObject::from_object_file_string(file_string).unwrap();

        assert_eq!(git_object.size, 6);
        assert_eq!(git_object.object_type, "blob");
        assert_eq!(git_object.content, "content");
    }

    #[test]
    fn from_object_file_string_returns_error_for_invalid_object_file_string() {
        let file_string = "invalid";
        let result = GitObject::from_object_file_string(file_string);

        assert!(result.is_err());
    }

    #[test]
    fn from_object_file_string_returns_error_for_object_file_string_with_invalid_size() {
        let file_string = "blob invalid\0content";
        let result = GitObject::from_object_file_string(file_string);

        assert!(result.is_err());
    }
}
