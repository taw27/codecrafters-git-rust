use std::error::Error;
use std::fs::File;
use std::io::Read;

use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};

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

pub fn get_sha<R: Read>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha1::new();

    std::io::copy(reader, &mut hasher)?;

    let result = hasher.finalize();
    let sha = format!("{:x}", result);

    Ok(sha)
}

pub fn read_and_decompress_file(path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut decompressed_content = String::new();

    let mut decoder = ZlibDecoder::new(file);

    decoder.read_to_string(&mut decompressed_content)?;

    Ok(decompressed_content)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::Write;

    use flate2::bufread::ZlibEncoder;
    use flate2::Compression;

    use super::*;

    #[test]
    fn get_sha_returns_correct_sha_for_given_input() {
        let data = "Hello, world!";
        let mut cursor = Cursor::new(data);
        let sha = get_sha(&mut cursor).unwrap();
        assert_eq!(sha, "943a702d06f34599aee1f8da8ef9f7296031d699");
    }

    #[test]
    fn get_sha_returns_error_for_io_failure() {
        struct FailingReader;
        impl Read for FailingReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "forced failure",
                ))
            }
        }
        let mut reader = FailingReader;
        let result = get_sha(&mut reader);
        assert!(result.is_err());
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
        assert_eq!(decompressed_content, content);
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
}
