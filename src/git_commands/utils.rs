use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use flate2::Compression;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use sha1::{Digest, Sha1};

pub trait ShaGetter {
    fn get_sha(&self) -> Result<String, Box<dyn Error>>;
}

impl<T: AsRef<[u8]> + ?Sized> ShaGetter for &T {
    fn get_sha(&self) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha1::new();
        hasher.update(self);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}

impl ShaGetter for Vec<u8> {
    fn get_sha(&self) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha1::new();
        hasher.update(self);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}

impl ShaGetter for String {
    fn get_sha(&self) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha1::new();
        hasher.update(self.as_bytes());
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}

pub fn get_object_path(sha: &str) -> Result<String, &'static str> {
    if sha.len() != 40 {
        return Err("file sha is invalid. Needs to be 40 char");
    }

    let bas_dir = ".git/objects/";
    let first_two_chars = &sha[0..2];
    let file_name = &sha[2..];

    let object_path = format!("{}{}/{}", bas_dir, first_two_chars, file_name);

    Ok(object_path)
}

pub fn read_and_decompress_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut decompressed_content_buffer = Vec::new();

    let mut decoder = ZlibDecoder::new(file);

    decoder.read_to_end(&mut decompressed_content_buffer)?;

    Ok(decompressed_content_buffer)
}

pub fn compress_and_write_file(path: &str, content: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let parent_dir = Path::new(path).parent().ok_or("Invalid path")?;
    fs::create_dir_all(parent_dir)?;

    let mut file = File::create(path)?;
    let mut compressed_content = Vec::new();
    let mut encoder = ZlibEncoder::new(content.as_slice(), Compression::default());

    encoder.read_to_end(&mut compressed_content)?;

    file.write_all(&compressed_content)?;

    Ok(())
}

pub trait ObjectPathGetter {
    fn get_object_path(&self, sha: &str) -> Result<String, &'static str>;
}

impl ObjectPathGetter for ActualObjectPathGetter {
    fn get_object_path(&self, sha: &str) -> Result<String, &'static str> {
        get_object_path(sha)
    }
}

pub struct ActualObjectPathGetter {}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::bufread::ZlibEncoder;
    use flate2::Compression;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn get_sha_returns_correct_sha_for_str_input() {
        let data = "Hello, world!";
        let sha = data.get_sha().unwrap();
        assert_eq!(sha, "943a702d06f34599aee1f8da8ef9f7296031d699");
    }

    #[test]
    fn get_sha_returns_correct_sha_for_vector_input() {
        let data = b"Hello, world!".to_vec();
        let sha = data.get_sha().unwrap();
        assert_eq!(sha, "943a702d06f34599aee1f8da8ef9f7296031d699");
    }

    #[test]
    fn get_object_path_returns_correct_path_for_valid_sha() {
        let sha = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
        let path = get_object_path(sha).unwrap();
        assert_eq!(
            path,
            ".git/objects/2a/ae6c35c94fcfb415dbe95f408b9ce91ee846ed"
        );
    }

    #[test]
    fn get_object_path_returns_error_for_invalid_sha() {
        let sha = "invalid_sha";
        let result = get_object_path(sha);
        assert!(result.is_err());
    }

    #[test]
    fn read_and_decompress_file_returns_correct_content_for_valid_path() {
        let content = "Hello, world!";
        let mut encoder = ZlibEncoder::new(content.as_bytes(), Compression::default());
        let mut compressed_content = Vec::new();

        encoder.read_to_end(&mut compressed_content).unwrap();

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(&compressed_content).unwrap();

        let temp_file_path = temp_file.path().to_str().unwrap();

        let decompressed_content = read_and_decompress_file(temp_file_path).unwrap();

        assert_eq!(
            String::from_utf8(decompressed_content).unwrap().to_string(),
            content
        );
    }

    #[test]
    fn read_and_decompress_file_returns_error_for_invalid_path() {
        let path = "non_existent_file.txt";
        let result = read_and_decompress_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn read_and_decompress_file_returns_error_for_corrupted_file() {
        let content = "This is not a valid zlib compressed file";

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();

        let temp_file_path = temp_file.path().to_str().unwrap();

        let result = read_and_decompress_file(temp_file_path);
        assert!(result.is_err());
    }
    #[test]
    fn compress_and_write_file_creates_file_with_correct_content() {
        let content = "Hello, world!";
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_str().unwrap();

        compress_and_write_file(temp_file_path, &content.as_bytes().to_vec()).unwrap();

        let file = File::open(temp_file_path).unwrap();
        let mut decompressed_content = String::new();
        let mut decoder = ZlibDecoder::new(file);
        decoder.read_to_string(&mut decompressed_content).unwrap();

        assert_eq!(decompressed_content, content);
    }

    #[test]
    fn compress_and_write_file_overwrites_existing_file_content() {
        let initial_content = b"Initial content".to_vec();
        let new_content = b"New content".to_vec();
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_str().unwrap();

        compress_and_write_file(temp_file_path, &initial_content).unwrap();
        compress_and_write_file(temp_file_path, &new_content).unwrap();

        let file = File::open(temp_file_path).unwrap();
        let mut decompressed_content: Vec<u8> = Vec::new();
        let mut decoder = ZlibDecoder::new(file);
        decoder.read_to_end(&mut decompressed_content).unwrap();

        assert_eq!(decompressed_content, new_content);
    }

    #[test]
    fn compress_and_write_file_returns_error_for_invalid_path() {
        let content = b"Hello, world!".to_vec();
        let path = "\0";

        let result = compress_and_write_file(path, &content);

        assert!(result.is_err());
    }
}
