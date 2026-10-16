use std::fs;
use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules"];

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub loaded: bool,
    pub children: Vec<Entry>,
}

impl Entry {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        Self {
            path,
            name,
            is_dir,
            expanded: false,
            loaded: false,
            children: Vec::new(),
        }
    }

    /// Load children kalau folder dan belum loaded
    fn ensure_loaded(&mut self) {
        if !self.is_dir || self.loaded {
            return;
        }

        self.children = read_dir_entries(&self.path);
        self.loaded = true;
    }

    /// Toggle expand/collapse
    pub fn toggle_expand(&mut self) {
        if !self.is_dir {
            return;
        }

        self.ensure_loaded();
        self.expanded = !self.expanded;
    }
}

fn read_dir_entries(dir: &Path) -> Vec<Entry> {
    let Ok(read) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut items: Vec<(PathBuf, bool)> = Vec::new();

    for entry in read {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip dirs yang tidak perlu
        if path.is_dir() && SKIP_DIRS.contains(&name) {
            continue;
        }

        let is_dir = path.is_dir();
        items.push((path, is_dir));
    }

    // Urutkan: folder dulu, lalu alfabetis
    items.sort_by(|a, b| match (a.1, b.1) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.0.file_name().cmp(&b.0.file_name()),
    });

    items
        .into_iter()
        .map(|(path, is_dir)| Entry::new(path, is_dir))
        .collect()
}

pub struct FileTree {
    pub root: PathBuf,
    pub root_entry: Entry,
    pub selected_path: Option<PathBuf>,
    pub flat: Vec<PathBuf>,
}

impl FileTree {
    pub fn from_path<P: AsRef<Path>>(root: P) -> std::io::Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a directory: {}", root.display()),
            ));
        }

        let mut root_entry = Entry::new(root.clone(), true);
        root_entry.ensure_loaded();
        root_entry.expanded = true;

        let mut tree = Self {
            root,
            root_entry,
            selected_path: None,
            flat: Vec::new(),
        };

        tree.rebuild_flat();

        // Pilih item pertama
        if let Some(first) = tree.flat.first() {
            tree.selected_path = Some(first.clone());
        }

        Ok(tree)
    }

    /// Rebuild cache flat (urutan tampilan)
    pub fn rebuild_flat(&mut self) {
        self.flat.clear();
        flatten(&self.root_entry, &mut self.flat);
    }

    pub fn len(&self) -> usize {
        self.flat.len()
    }

    pub fn is_empty(&self) -> bool {
        self.flat.is_empty()
    }

    /// Cari entry berdasarkan path (di tree)
    pub fn find_entry(&self, path: &Path) -> Option<&Entry> {
        find_in_entry(&self.root_entry, path)
    }

    pub fn find_entry_mut(&mut self, path: &Path) -> Option<&mut Entry> {
        find_in_entry_mut(&mut self.root_entry, path)
    }

    pub fn selected_index(&self) -> Option<usize> {
        let sel = self.selected_path.as_ref()?;
        self.flat.iter().position(|p| p == sel)
    }

    pub fn move_up(&mut self) {
        if let Some(idx) = self.selected_index() {
            if idx > 0 {
                self.selected_path = Some(self.flat[idx - 1].clone());
            }
        }
    }

    pub fn move_down(&mut self) {
        if let Some(idx) = self.selected_index() {
            if idx + 1 < self.flat.len() {
                self.selected_path = Some(self.flat[idx + 1].clone());
            }
        }
    }

    /// Toggle expand pada folder yang dipilih
    pub fn toggle_expand_selected(&mut self) -> bool {
        let Some(sel) = self.selected_path.clone() else {
            return false;
        };

        let Some(entry) = self.find_entry_mut(&sel) else {
            return false;
        };

        if !entry.is_dir {
            return false;
        }

        entry.toggle_expand();

        // Kalau folder di-collapse, dan selected ada di dalamnya, pindah ke folder
        if !entry.expanded {
            self.rebuild_flat();
            // Pastikan selected masih di flat
            if !self.flat.contains(&sel) {
                self.selected_path = Some(sel);
            }
        } else {
            self.rebuild_flat();
        }

        true
    }

    /// Buat file baru di folder yang dipilih.
    /// Return path file yang dibuat.
    pub fn create_file(&mut self, name: &str) -> std::io::Result<PathBuf> {
        let parent = self.resolve_parent_dir();

        let new_path = parent.join(name);

        if new_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Already exists: {}", new_path.display()),
            ));
        }

        fs::write(&new_path, "")?;

        // Reload parent folder
        self.reload_folder(&parent)?;

        // Select file baru
        self.selected_path = Some(new_path.clone());

        Ok(new_path)
    }

    /// Buat folder baru di folder yang dipilih.
    /// Return path folder yang dibuat.
    pub fn create_folder(&mut self, name: &str) -> std::io::Result<PathBuf> {
        let parent = self.resolve_parent_dir();

        let new_path = parent.join(name);

        if new_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Already exists: {}", new_path.display()),
            ));
        }

        fs::create_dir(&new_path)?;

        self.reload_folder(&parent)?;

        self.selected_path = Some(new_path.clone());

        Ok(new_path)
    }
    /// Hapus item yang dipilih (file atau folder).
    /// Return path yang dihapus.
    pub fn delete_selected(&mut self) -> std::io::Result<PathBuf> {
        let Some(sel) = self.selected_path.clone() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No selection",
            ));
        };

        // Cari entry untuk tahu is_dir
        let is_dir = self.find_entry(&sel).map(|e| e.is_dir).unwrap_or(false);

        if is_dir {
            fs::remove_dir_all(&sel)?;
        } else {
            fs::remove_file(&sel)?;
        }

        // Cari item berikutnya sebelum rebuild
        let next_selected = self.find_next_after_delete(&sel);

        // Reload parent folder
        let parent = sel.parent().map(|p| p.to_path_buf());
        if let Some(parent) = parent {
            self.reload_folder(&parent)?;
        } else {
            self.rebuild_flat();
        }

        // Set selection berikutnya
        self.selected_path = next_selected;

        Ok(sel)
    }

    /// Cari item berikutnya setelah delete.
    /// Kalau tidak ada, cari item sebelumnya.
    fn find_next_after_delete(&self, deleted: &Path) -> Option<PathBuf> {
        // Cari di flat list
        let idx = self.flat.iter().position(|p| p == deleted);

        if let Some(idx) = idx {
            // Coba item berikutnya
            if idx + 1 < self.flat.len() {
                let next = &self.flat[idx + 1];
                // Pastikan bukan anak dari yang dihapus
                if !next.starts_with(deleted) {
                    return Some(next.clone());
                }
            }

            // Coba item sebelumnya
            if idx > 0 {
                let prev = &self.flat[idx - 1];
                if !prev.starts_with(deleted) {
                    return Some(prev.clone());
                }
            }
        }

        // Fallback: parent
        deleted.parent().map(|p| p.to_path_buf())
    }

    /// Cek apakah path ada di tree.
    pub fn contains_path(&self, path: &Path) -> bool {
        self.flat.iter().any(|p| p == path)
    }

    /// Resolve parent directory berdasarkan selected entry.
    /// Kalau folder → folder itu. Kalau file → parent file.
    /// Kalau tidak ada → root.
    fn resolve_parent_dir(&self) -> PathBuf {
        self.selected_entry()
            .map(|e| {
                if e.is_dir {
                    e.path.clone()
                } else {
                    e.path
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.root.clone())
                }
            })
            .unwrap_or_else(|| self.root.clone())
    }

    /// Reload folder tertentu di tree — update children.
    fn reload_folder(&mut self, folder: &Path) -> std::io::Result<()> {
        if let Some(entry) = find_in_entry_mut(&mut self.root_entry, folder) {
            entry.children = read_dir_entries(&entry.path);
            entry.loaded = true;
            entry.expanded = true;
        }
        self.rebuild_flat();
        Ok(())
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        let sel = self.selected_path.as_ref()?;
        self.find_entry(sel)
    }

    /// Info untuk render: (path, name, is_dir, depth, expanded)
    pub fn visible_entries(&self) -> Vec<(PathBuf, String, bool, usize, bool)> {
        let mut result = Vec::new();
        collect_visible(&self.root_entry, 0, &mut result);
        result
    }
}

fn flatten(entry: &Entry, out: &mut Vec<PathBuf>) {
    for child in &entry.children {
        out.push(child.path.clone());
        if child.is_dir && child.expanded {
            flatten(child, out);
        }
    }
}

fn collect_visible(
    entry: &Entry,
    depth: usize,
    out: &mut Vec<(PathBuf, String, bool, usize, bool)>,
) {
    for child in &entry.children {
        out.push((
            child.path.clone(),
            child.name.clone(),
            child.is_dir,
            depth,
            child.expanded,
        ));

        if child.is_dir && child.expanded {
            collect_visible(child, depth + 1, out);
        }
    }
}

fn find_in_entry<'a>(entry: &'a Entry, path: &Path) -> Option<&'a Entry> {
    if entry.path == path {
        return Some(entry);
    }

    for child in &entry.children {
        if let Some(found) = find_in_entry(child, path) {
            return Some(found);
        }
    }

    None
}

fn find_in_entry_mut<'a>(entry: &'a mut Entry, path: &Path) -> Option<&'a mut Entry> {
    if entry.path == path {
        return Some(entry);
    }

    for child in &mut entry.children {
        if let Some(found) = find_in_entry_mut(child, path) {
            return Some(found);
        }
    }

    None
}
