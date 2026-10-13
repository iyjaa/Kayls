use std::fs;
use std::path::{Path, PathBuf};

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub dirty: bool,
}

impl Document {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        if lines.is_empty() {
            lines.push(String::new());
        }

        Ok(Self {
            path,
            lines,
            cursor: Cursor::new(),
            dirty: false,
        })
    }

    pub fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            lines: vec![String::new()],
            cursor: Cursor::new(),
            dirty: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn current_line_len(&self) -> usize {
        self.lines
            .get(self.cursor.line)
            .map(|l| l.chars().count())
            .unwrap_or(0)
    }

    pub fn content_as_string(&self) -> String {
        self.lines.join("\n")
    }
}