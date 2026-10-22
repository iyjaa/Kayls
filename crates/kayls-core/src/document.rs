use std::fs;
use std::path::{Path, PathBuf};

use ropey::Rope;
use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub rope: Rope,
    pub cursor: Cursor,
    pub dirty: bool,
}

impl Document {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        // Jangan canonicalize: operasi ini lambat (1-2ms) dan tidak perlu
        // untuk sekadar membaca file. Path relatif sudah cukup.
        let content = fs::read_to_string(&path)?;
        let rope = Rope::from_str(&content);

        Ok(Self {
            path,
            rope,
            cursor: Cursor::new(),
            dirty: false,
        })
    }

    pub fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            rope: Rope::new(),
            cursor: Cursor::new(),
            dirty: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn current_line_len(&self) -> usize {
        if self.cursor.line < self.rope.len_lines() {
            self.rope.line(self.cursor.line).len_chars().saturating_sub(1)
        } else {
            0
        }
    }

    pub fn content_as_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn insert_char(&mut self, c: char) {
        let line = self.cursor.line;
        let col = self.cursor.col;

        if line < self.rope.len_lines() {
            let line_start = self.rope.line_to_char(line);
            let char_idx = line_start + col;
            self.rope.insert_char(char_idx, c);
            self.cursor.col += 1;
            self.dirty = true;
        }
    }

    pub fn delete_char(&mut self) {
        let line = self.cursor.line;
        let col = self.cursor.col;

        if col > 0 {
            // Hapus karakter sebelum kursor di baris yang sama
            if line < self.rope.len_lines() {
                let line_start = self.rope.line_to_char(line);
                let char_idx = line_start + col - 1;
                if char_idx < self.rope.len_chars() {
                    self.rope.remove(char_idx..char_idx + 1);
                    self.cursor.col -= 1;
                    self.dirty = true;
                }
            }
        } else if line > 0 {
            // Gabungkan dengan baris sebelumnya: hapus newline di akhir baris sebelumnya
            let line_start = self.rope.line_to_char(line);
            if line_start > 0 {
                let prev_line_len = self.rope.line(line - 1).len_chars().saturating_sub(1);
                self.rope.remove(line_start - 1..line_start);
                self.cursor.line -= 1;
                self.cursor.col = prev_line_len;
                self.dirty = true;
            }
        }
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let content = self.content_as_string();
        fs::write(&self.path, content)?;
        self.dirty = false;
        Ok(())
    }
}
