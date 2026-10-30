#![allow(clippy::empty_line_after_doc_comments)]
pub mod config;
pub mod cursor;
pub mod document;
pub mod filetree;

pub use config::Config;
pub use cursor::Cursor;
pub use document::Document;
pub use filetree::{Entry, FileTree};

pub fn find_matches(code: &str, query: &str) -> Vec<usize> {
    use std::collections::VecDeque;

    if query.is_empty() {
        return Vec::new();
    }

    let lower = |c: char| c.to_lowercase().next().unwrap_or(c);
    let query_chars: Vec<char> = query.chars().map(lower).collect();
    let query_len = query_chars.len();

    if query_len == 0 {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut char_pos = 0;
    let mut window: VecDeque<char> = VecDeque::with_capacity(query_len);
    let mut iter = code.chars().map(lower);

    // Isi window awal
    for _ in 0..query_len {
        match iter.next() {
            Some(c) => window.push_back(c),
            None => return Vec::new(),
        }
    }

    // Cek window pertama
    if window.iter().eq(query_chars.iter()) {
        matches.push(0);
    }

    // Geser window
    for c in iter {
        window.pop_front();
        window.push_back(c);
        char_pos += 1;

        if window.iter().eq(query_chars.iter()) {
            matches.push(char_pos);
        }
    }

    matches
}

#[cfg(test)]
mod find_matches_tests {
    use super::*;

    #[test]
    fn test_unicode_safe() {
        // "aé" adalah 2 karakter: 'a' dan 'é'
        // Query "ab" tidak boleh panic dan harus kosong
        assert!(find_matches("aé", "ab").is_empty());

        // Query "a" harus menemukan match di indeks 0
        assert_eq!(find_matches("aé", "a"), vec![0]);

        // Query "é" harus menemukan match di indeks 1
        assert_eq!(find_matches("aé", "é"), vec![1]);
    }

    #[test]
    fn test_empty_query() {
        assert!(find_matches("hello", "").is_empty());
    }

    #[test]
    fn test_normal() {
        assert_eq!(find_matches("hello hello", "hello"), vec![0, 6]);
    }
}
