#[derive(Debug, PartialEq)]
pub struct TreeObject {
    tree_entries: Vec<TreeEntry>,
}

impl TreeObject {
    pub fn new() -> Self {
        Self {
            tree_entries: Vec::new(),
        }
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
