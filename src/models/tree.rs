use std::fmt;

use crate::models::git_object::GetContentString;

#[derive(Debug, PartialEq)]
pub struct Tree {
    pub tree_entries: Vec<TreeEntry>,
    pub content: Vec<u8>,
}

impl Tree {
    pub fn new(content: Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            tree_entries: Self::content_to_tree_entries(&content)?,
            content,
        })
    }

    fn content_to_tree_entries(content: &Vec<u8>) -> Result<Vec<TreeEntry>, String> {
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

impl GetContentString for Tree {
    fn get_content_string(&self) -> Result<String, String> {
        let mut content: String = String::new();

        for tree_entry in &self.tree_entries {
            let entry_content = format!(
                "{} {} {} {}",
                tree_entry.mode,
                tree_entry.object_type(),
                tree_entry.sha,
                tree_entry.name
            );

            content.push_str(entry_content.as_str());

            content.push_str("\n");
        }

        print!("{}", content);
        Ok(content)
    }
}

impl Tree {
    pub fn get_names(&self) -> String {
        let mut names: String = String::new();

        for tree_entry in &self.tree_entries {
            names.push_str(tree_entry.name.as_str());

            names.push_str("\n");
        }

        names
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeEntry {
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
            "40000" => TreeEntryMode::Directory,
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

impl fmt::Display for TreeEntryMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode_str = match self {
            TreeEntryMode::RegularFile => "100644",
            TreeEntryMode::ExecutableFile => "100755",
            TreeEntryMode::SymbolicLink => "120000",
            TreeEntryMode::Directory => "040000",
        };

        write!(f, "{}", mode_str)
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

    #[test]
    fn get_content_returns_correct_format_for_single_entry() {
        let tree_entries = vec![TreeEntry {
            mode: TreeEntryMode::RegularFile,
            sha: "abc123".to_string(),
            name: "file1.txt".to_string(),
        }];
        let tree = Tree {
            tree_entries,
            content: Vec::new(),
        };

        let content = tree.get_content_string().unwrap();

        assert_eq!(content, "100644 blob abc123 file1.txt\n");
    }

    #[test]
    fn get_content_returns_correct_format_for_multiple_entries() {
        let tree_entries = vec![
            TreeEntry {
                mode: TreeEntryMode::RegularFile,
                sha: "abc123".to_string(),
                name: "file1.txt".to_string(),
            },
            TreeEntry {
                mode: TreeEntryMode::ExecutableFile,
                sha: "def456".to_string(),
                name: "file2.txt".to_string(),
            },
        ];
        let tree = Tree {
            tree_entries,
            content: Vec::new(),
        };

        let content = tree.get_content_string().unwrap();

        assert_eq!(
            content,
            "100644 blob abc123 file1.txt\n100755 blob def456 file2.txt\n"
        );
    }

    #[test]
    fn get_content_returns_correct_format_for_directory_entry() {
        let tree_entries = vec![TreeEntry {
            mode: TreeEntryMode::Directory,
            sha: "abc123".to_string(),
            name: "dir1".to_string(),
        }];
        let tree = Tree {
            tree_entries,
            content: Vec::new(),
        };

        let content = tree.get_content_string().unwrap();

        assert_eq!(content, "040000 tree abc123 dir1\n");
    }

    #[test]
    fn get_content_returns_correct_format_for_symbolic_link_entry() {
        let tree_entries = vec![TreeEntry {
            mode: TreeEntryMode::SymbolicLink,
            sha: "abc123".to_string(),
            name: "link1".to_string(),
        }];
        let tree = Tree {
            tree_entries,
            content: Vec::new(),
        };

        let content = tree.get_content_string().unwrap();

        assert_eq!(content, "120000 blob abc123 link1\n");
    }
}
