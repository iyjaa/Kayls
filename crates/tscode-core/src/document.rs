use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub lines: Vec<String>,
    pub dirty: bool,
}

impl Document {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)?;
        let lines: Vec<String> = content.lines().map(String::from).collect();

        Ok(Self {
            path,
            lines,
            dirty: false,
        })
    }

    pub fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            lines: vec![String::new()],
            dirty: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn content_as_string(&self) -> String {
        self.lines.join("\n")
    }
}