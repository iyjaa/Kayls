use std::fs;
use std::path::{Path, PathBuf};

/// Satu entry di file tree (bisa file atau folder)
#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

impl Entry {
    pub fn new(path: PathBuf, is_dir: bool, depth: usize) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        Self {
            path,
            name,
            is_dir,
            depth,
            expanded: false,
        }
    }
}

/// File tree: root + daftar entry (rekursif, hanya yang expanded)
#[derive(Debug)]
pub struct FileTree {
    pub root: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
}

impl FileTree {
    /// Baca folder root, load entries level 1
    pub fn from_path<P: AsRef<Path>>(root: P) -> std::io::Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a directory: {}", root.display()),
            ));
        }

        let mut tree = Self {
            root: root.clone(),
            entries: Vec::new(),
            selected: 0,
        };

        tree.load_entries(&root, 0)?;
        Ok(tree)
    }

    /// Baca isi folder, tambahkan ke entries
    fn load_entries(&mut self, dir: &Path, depth: usize) -> std::io::Result<()> {
        let mut items: Vec<(PathBuf, bool)> = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let is_dir = path.is_dir();
            items.push((path, is_dir));
        }

        // Urutkan: folder dulu, lalu alfabetis
        items.sort_by(|a, b| {
            match (a.1, b.1) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.file_name().cmp(&b.0.file_name()),
            }
        });

        for (path, is_dir) in items {
            self.entries.push(Entry::new(path, is_dir, depth));
        }

        Ok(())
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        self.entries.get(self.selected)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Pindah selection ke atas
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Pindah selection ke bawah
    pub fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }
}