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
        let path = fs::canonicalize(&path)?;
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

    pub fn insert_char(&mut self, c: char) {
        let line = self.cursor.line;
        let col = self.cursor.col;

        if let Some(current) = self.lines.get_mut(line) {
            let char_count = current.chars().count();

            if col >= char_count {
                current.push(c);
            } else {
                let byte_idx = current
                    .char_indices()
                    .nth(col)
                    .map(|(i, _)| i)
                    .unwrap_or(current.len());
                current.insert(byte_idx, c);
            }

            self.cursor.col += 1;
            self.dirty = true;
        }
    }

    pub fn delete_char(&mut self) {
        let line = self.cursor.line;
        let col = self.cursor.col;

        if col == 0 {
            return;
        }

        if let Some(current) = self.lines.get_mut(line) {
            let byte_idx = current
                .char_indices()
                .nth(col - 1)
                .map(|(i, _)| i)
                .unwrap_or(current.len());
            current.remove(byte_idx);
            self.cursor.col -= 1;
            self.dirty = true;
        }
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let content = self.content_as_string();
        fs::write(&self.path, content)?;
        self.dirty = false;
        Ok(())
    }
}
