#[derive(Debug, PartialEq)]
pub struct Tree {
    tree_entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new(content: Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            tree_entries: Self::content_to_tree_entries(content)?,
        })
    }

    fn content_to_tree_entries(content: Vec<u8>) -> Result<Vec<TreeEntry>, String> {
        let mut tree_entries: Vec<TreeEntry> = Vec::new();
        let mut content_slice = &content[0..];

        while content_slice.len() > 0 {
            let mut slice_start_position;
            let mut end_position;

            end_position = content_slice
                .iter()
                .position(|&x| x == b' ')
                .ok_or("Invalid mode")?;

            let mode = Self::parse_string_from_content(content_slice, 0, end_position);

            // skip space ' '
            slice_start_position = end_position + 1;

            content_slice = &content_slice[slice_start_position..];

            end_position = content_slice
                .iter()
                .position(|&x| x == 0)
                .ok_or("Invalid name")?;

            let name = Self::parse_string_from_content(content_slice, 0, end_position);

            if name.len() == 0 {
                return Err("Name missing".to_string());
            }

            // skip null byte
            slice_start_position = end_position + 1;

            content_slice = &content_slice[slice_start_position..];

            if content_slice.len() < 20 {
                return Err("Invalid sha: start position".to_string());
            }

            let sha = hex::encode(&content_slice[0..20]);

            content_slice = &content_slice[20..];

            tree_entries.push(TreeEntry {
                mode: TreeEntryMode::from_string(mode.as_str())?,
                name,
                sha,
            });
        }

        Ok(tree_entries)
    }

    fn parse_string_from_content(content: &[u8], start_idx: usize, end_idx: usize) -> String {
        let str = String::from_utf8_lossy(&content[start_idx..end_idx]).to_string();

        str
    }
}

#[derive(Debug, PartialEq)]
struct TreeEntry {
    mode: TreeEntryMode,
    sha: String,
    name: String,
}

impl TreeEntry {
    fn object_type(&self) -> String {
        match self.mode {
            TreeEntryMode::Directory => "tree".to_string(),
            _ => "blob".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum TreeEntryMode {
    RegularFile,
    ExecutableFile,
    SymbolicLink,
    Directory,
}

impl TreeEntryMode {
    fn from_string(mode_str: &str) -> Result<Self, String> {
        let mode = match mode_str {
            "100644" => TreeEntryMode::RegularFile,
            "100755" => TreeEntryMode::ExecutableFile,
            "120000" => TreeEntryMode::SymbolicLink,
            "040000" => TreeEntryMode::Directory,
            _ => {
                return Err(format!(
                    "tree entry mode string not recognized (Given {})",
                    mode_str
                ))
            }
        };

        Ok(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_object_new_with_valid_content() {
        let content = b"100644 blob.txt\0".to_vec();
        let content = [content, vec![1; 20]].concat();
        let tree_object = Tree::new(content).unwrap();

        assert_eq!(tree_object.tree_entries.len(), 1);
        assert_eq!(tree_object.tree_entries[0].mode, TreeEntryMode::RegularFile);
        assert_eq!(tree_object.tree_entries[0].name, "blob.txt");
    }

    #[test]
    fn tree_object_new_with_invalid_mode() {
        let content = b"999999 blob.txt\0".to_vec();
        let content = [content, vec![0; 20]].concat();
        let tree_object = Tree::new(content);

        assert!(tree_object.is_err());
    }

    #[test]
    fn tree_object_new_with_invalid_name() {
        let content = b"100644 \0".to_vec();
        let content = [content, vec![1; 20]].concat();
        let tree_object = Tree::new(content);

        assert!(tree_object.is_err());
    }

    #[test]
    fn tree_object_new_with_invalid_sha() {
        let content = b"100644 blob.txt\0short_sha".to_vec();
        let tree_object = Tree::new(content);

        assert!(tree_object.is_err());
    }

    #[test]
    fn tree_object_new_with_multiple_entries() {
        let content1 = b"100644 blob.txt\0".to_vec();
        let content1 = [content1, vec![2; 20]].concat();
        let content2 = b"100755 exec_file\0".to_vec();
        let content2 = [content2, vec![3; 20]].concat();
        let content = [content1, content2].concat();
        let tree_object = Tree::new(content).unwrap();

        assert_eq!(tree_object.tree_entries.len(), 2);
        assert_eq!(tree_object.tree_entries[0].mode, TreeEntryMode::RegularFile);
        assert_eq!(tree_object.tree_entries[0].name, "blob.txt");
        assert_eq!(
            tree_object.tree_entries[1].mode,
            TreeEntryMode::ExecutableFile
        );
        assert_eq!(tree_object.tree_entries[1].name, "exec_file");
    }
}
