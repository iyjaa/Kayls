use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use kayls_core::{Config, Document, FileTree};
use kayls_editor::editor::Editor;
use kayls_editor::theme::{dracula, gruvbox, nord, vesper};
use ratatui::layout::Rect;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Easy,
    Command,
    Prompt,
    FilePicker,
    Search,
    PromptNewFile,
    PromptNewFolder,
    ConfirmDelete,
    SelectTheme,
    SelectKeybinding,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Editor,
    Explorer,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Vesper,
    Gruvbox,
    Dracula,
    Nord,
}

pub struct App {
    pub should_quit: bool,
    pub document: Option<Document>,
    pub editor: Option<Editor>,
    pub editor_area: Rect,
    pub file_tree: Option<FileTree>,
    pub show_explorer: bool,
    pub focus: Focus,
    pub theme: Theme,
    pub theme_cursor: usize,
    pub keybinding_cursor: usize,
    pub scroll_offset: usize,
    pub scroll_offset_x: usize,
    pub explorer_scroll: usize,
    pub explorer_scroll_x: usize,
    pub explorer_height: u16,
    pub explorer_width: u16,
    pub editor_height: u16,
    pub clipboard: String,
    pub editor_width: u16,
    pub search_query: String,
    pub search_matches: Vec<usize>,
    pub search_current_idx: usize,
    pub search_entered: bool,
    pub read_mode: bool,
    pub picker: crate::picker::PickerState,
    pub mode: Mode,
    pub saved_mode: Mode,
    pub input_buffer: String,
    pub pending_delete: Option<PathBuf>,
    pub status_message: Option<String>,
    pub status_time: Option<Instant>,
}

impl App {
    pub fn new(document: Option<Document>) -> Self {
        let config = Config::load();

        let theme = match config.editor.theme.to_lowercase().as_str() {
            "gruvbox" => Theme::Gruvbox,
            "dracula" => Theme::Dracula,
            "nord" => Theme::Nord,
            _ => Theme::Vesper,
        };

        let mut app = Self {
            should_quit: false,
            document,
            editor: None,
            editor_area: Rect::default(),
            file_tree: None,
            show_explorer: config.editor.show_explorer,
            focus: Focus::Explorer,
            theme,
            keybinding_cursor: 0,
            theme_cursor: match theme {
                Theme::Vesper => 0,
                Theme::Gruvbox => 1,
                Theme::Dracula => 2,
                Theme::Nord => 3,
            },
            scroll_offset: 0,
            scroll_offset_x: 0,
            explorer_scroll: 0,
            explorer_scroll_x: 0,
            explorer_height: 0,
            explorer_width: 0,
            editor_height: 0,
            clipboard: String::new(),
            editor_width: 0,
            search_query: String::new(),
            search_matches: Vec::new(),
            search_current_idx: 0,
            search_entered: false,
            read_mode: false,
            picker: crate::picker::PickerState::new(),
            mode: if config.editor.keybinding.to_lowercase() == "easy" {
                Mode::Easy
            } else {
                Mode::Normal
            },
            saved_mode: if config.editor.keybinding.to_lowercase() == "easy" {
                Mode::Easy
            } else {
                Mode::Normal
            },
            input_buffer: String::new(),
            pending_delete: None,
            status_message: None,
            status_time: None,
        };

        app.load_file_tree();
        app.load_editor();
        app
    }

    fn load_file_tree(&mut self) {
        let root = std::env::current_dir().ok();

        if let Some(root) = root {
            match FileTree::from_path(&root) {
                Ok(tree) => {
                    self.file_tree = Some(tree);
                }
                Err(e) => {
                    self.status_message = Some(format!("FileTree error: {}", e));
                }
            }
        }
    }

    fn load_editor(&mut self) {
        let Some(doc) = self.document.as_ref() else {
            return;
        };

        let lang = detect_language(&doc.path);
        let rope = doc.rope.clone();

        let theme_colors = match self.theme {
            Theme::Vesper => vesper(),
            Theme::Gruvbox => gruvbox(),
            Theme::Dracula => dracula(),
            Theme::Nord => nord(),
        };

        match Editor::from_rope(lang, rope, theme_colors) {
            Ok(mut ed) => {
                ed.show_line_numbers(true);
                self.editor = Some(ed);
            }
            Err(e) => {
                self.status_message = Some(format!("Editor error: {}", e));
                self.status_time = Some(Instant::now());
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::ALT)
            && let KeyCode::Char('q') = key.code
        {
            // Cek dirty sebelum quit
            if let Some(editor) = &self.editor
                && editor.dirty
            {
                self.status_message =
                    Some("Unsaved changes! Save first (Ctrl+S / Esc w).".to_string());
                self.status_time = Some(Instant::now());
                return;
            }
            self.should_quit = true;
            return;
        }

        match self.mode {
            Mode::Normal => self.handle_normal(key),
            Mode::Easy => self.handle_easy(key),
            Mode::Command => self.handle_command(key),
            Mode::Prompt => self.handle_prompt(key),
            Mode::PromptNewFile => self.handle_prompt_new_file(key),
            Mode::PromptNewFolder => self.handle_prompt_new_folder(key),
            Mode::ConfirmDelete => self.handle_confirm_delete(key),
            Mode::SelectTheme => self.handle_select_theme(key),
            Mode::SelectKeybinding => self.handle_select_keybinding(key),
            Mode::Search => self.handle_search(key),
            Mode::FilePicker => {
                let root = self
                    .file_tree
                    .as_ref()
                    .map(|t| t.root.clone())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));

                if let Some(path) = self.picker.handle_key(key, &root) {
                    self.mode = self.saved_mode;
                    if let Ok(doc) = Document::open(&path) {
                        self.document = Some(doc);
                        self.load_editor();
                    }
                } else if key.code == KeyCode::Esc {
                    self.mode = self.saved_mode;
                }
            }
        }
    }

    pub fn paste(&mut self, text: &str) {
        if let Some(editor) = &mut self.editor {
            use kayls_editor::actions::InsertText;
            editor.apply(InsertText {
                text: text.to_string(),
            });
            editor.dirty = true;
        }
    }

    fn handle_easy(&mut self, key: KeyEvent) {
        // Handle Ctrl+<key> shortcuts for VS Code-style
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('s') => {
                    self.saved_mode = Mode::Easy;
                    self.mode = Mode::Command;
                    self.handle_command(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
                    return;
                }
                KeyCode::Char('o') => {
                    // Cek dirty sebelum membuka file baru
                    if let Some(editor) = &self.editor
                        && editor.dirty
                    {
                        self.status_message =
                            Some("Unsaved changes! Save first (Ctrl+S).".to_string());
                        self.status_time = Some(Instant::now());
                        return;
                    }
                    self.saved_mode = Mode::Easy;
                    self.mode = Mode::Prompt;
                    self.input_buffer.clear();
                    self.status_message = Some("Open file:".to_string());
                    self.status_time = Some(Instant::now());
                    return;
                }
                KeyCode::Char('q') => {
                    // Quit
                    if let Some(editor) = &self.editor
                        && editor.dirty
                    {
                        self.status_message =
                            Some("Unsaved changes! Save first (Ctrl+S).".to_string());
                        self.status_time = Some(Instant::now());
                        return;
                    }
                    self.should_quit = true;
                    return;
                }
                KeyCode::Char('e') => {
                    // Toggle Explorer
                    self.show_explorer = !self.show_explorer;
                    if self.show_explorer {
                        self.focus = Focus::Explorer;
                    } else {
                        self.focus = Focus::Editor;
                    }
                    return;
                }
                KeyCode::Char('n') => {
                    // New file (sementara ke Mode::PromptNewFile)
                    self.mode = Mode::PromptNewFile;
                    return;
                }
                _ => {}
            }
        }
        // Passthrough to normal handler
        self.handle_normal(key);
    }

    fn handle_select_keybinding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.keybinding_cursor = self.keybinding_cursor.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.keybinding_cursor < 1 {
                    self.keybinding_cursor += 1;
                }
            }
            KeyCode::Enter => {
                // Simpan pilihan
                let new_mode = if self.keybinding_cursor == 1 {
                    Mode::Easy
                } else {
                    Mode::Normal
                };
                self.mode = new_mode;
                self.saved_mode = new_mode;
                // Update config
                let mut config = Config::load();
                config.editor.keybinding = if self.keybinding_cursor == 1 {
                    "easy".to_string()
                } else {
                    "vim".to_string()
                };
                let _ = config.save();
                self.status_message = Some(format!(
                    "Mode: {}",
                    if self.keybinding_cursor == 1 {
                        "Easy"
                    } else {
                        "Vim"
                    }
                ));
                self.status_time = Some(Instant::now());
            }
            KeyCode::Esc => {
                self.mode = self.saved_mode;
            }
            _ => {}
        }
    }

    fn handle_search(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.search_query.clear();
                self.search_matches.clear();
            }
            KeyCode::Enter => {
                if !self.search_matches.is_empty() {
                    // Jika ini Enter pertama, langsung ke match ke-0.
                    // Jika bukan, baru maju ke match berikutnya.
                    if self.search_entered {
                        if !self.search_query.is_empty() && self.search_matches.len() > 1 {
                            self.search_current_idx =
                                (self.search_current_idx + 1) % self.search_matches.len();
                        }
                    } else {
                        self.search_entered = true;
                    }

                    let match_char_idx = self.search_matches[self.search_current_idx];
                    let h = self.editor_height as usize;
                    if let Some(editor) = &mut self.editor {
                        editor.set_cursor(match_char_idx);
                        editor.scroll_to_cursor(h);
                    }
                    self.status_message = Some(format!(
                        "Match {}/{}",
                        self.search_current_idx + 1,
                        self.search_matches.len()
                    ));
                    self.status_time = Some(Instant::now());
                }
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.search_entered = false;
                self.perform_search();
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.search_entered = false;
                self.perform_search();
            }
            _ => {}
        }
    }

    fn perform_search(&mut self) {
        self.search_matches.clear();
        self.search_current_idx = 0;

        if self.search_query.is_empty() {
            return;
        }

        if let Some(editor) = &self.editor {
            let code = editor.get_content(); // Mengembalikan String
            let query_lower = self.search_query.to_lowercase();
            let code_lower = code.to_lowercase();

            // Gunakan pencarian char-based, bukan byte-based
            let mut byte_start = 0;
            while let Some(byte_pos) = code_lower[byte_start..].find(&query_lower) {
                let abs_byte_pos = byte_start + byte_pos;
                // Konversi byte offset ke char offset
                let char_pos = code[..abs_byte_pos].chars().count();
                self.search_matches.push(char_pos);
                byte_start = abs_byte_pos + query_lower.len();
            }
        }
    }

    pub fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        if let Some(editor) = &mut self.editor {
            let area = self.editor_area;
            let _ = editor.mouse(mouse, &area);
        }
    }

    fn handle_normal(&mut self, key: KeyEvent) {
        // Ctrl+P untuk File Picker (bisa diakses dari mana saja, termasuk Welcome View)
        if key.code == KeyCode::Char('p') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.mode = Mode::FilePicker;
            self.picker.query.clear();
            self.picker.selected = 0;
            self.picker.scroll = 0;
            let root = self
                .file_tree
                .as_ref()
                .map(|t| t.root.clone())
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            self.picker.update_matches(&root);
            return;
        }

        if let KeyCode::Esc = key.code {
            self.mode = Mode::Command;
            self.status_message = Some("-- COMMAND --".to_string());
            self.status_time = Some(Instant::now());
            return;
        }

        match self.focus {
            Focus::Editor => self.handle_editor_keys(key),
            Focus::Explorer => self.handle_explorer_keys(key),
        }
    }

    fn handle_editor_keys(&mut self, key: KeyEvent) {
        if let Some(editor) = self.editor.as_mut() {
            let area = self.editor_area;

            // Ctrl+R untuk Read Mode
            if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
                self.read_mode = !self.read_mode;
                return;
            }

            // Ctrl+F untuk Search
            if key.code == KeyCode::Char('f') && key.modifiers.contains(KeyModifiers::CONTROL) {
                self.mode = Mode::Search;
                self.search_query.clear();
                self.search_matches.clear();
                self.search_current_idx = 0;
                return;
            }

            // Alt+C untuk Copy (karena Shift tidak ada di Termux)
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::ALT) {
                use kayls_editor::actions::Copy;
                editor.apply(Copy {});
                if let Ok(text) = editor.get_clipboard() {
                    self.clipboard = text;
                }
                return;
            }
            // Ctrl+V untuk Paste dari App.clipboard (antar-file)
            if key.code == KeyCode::Char('v') && key.modifiers.contains(KeyModifiers::CONTROL) {
                if !self.clipboard.is_empty() {
                    use kayls_editor::actions::InsertText;
                    editor.apply(InsertText {
                        text: self.clipboard.clone(),
                    });
                }
                return;
            }

            // Jika Read Mode aktif, semua panah hanya scroll layar (tidak menyentuh kursor)
            if self.read_mode {
                match key.code {
                    KeyCode::Up => {
                        editor.scroll_up();
                        return;
                    }
                    KeyCode::Down => {
                        editor.scroll_down(self.editor_height as usize);
                        return;
                    }
                    KeyCode::Left => {
                        let current_x = editor.get_offset_x();
                        if current_x > 0 {
                            editor.set_offset_x(current_x - 1);
                        }
                        return;
                    }
                    KeyCode::Right => {
                        let current_x = editor.get_offset_x();
                        editor.set_offset_x(current_x + 1);
                        return;
                    }
                    _ => {}
                }
            }

            if let Err(e) = editor.input(key, &area) {
                self.status_message = Some(format!("Editor error: {}", e));
                self.status_time = Some(Instant::now());
            }
        }
    }

    fn handle_explorer_keys(&mut self, key: KeyEvent) {
        let Some(tree) = self.file_tree.as_mut() else {
            return;
        };

        match key.code {
            KeyCode::Up => {
                tree.move_up();
                self.ensure_explorer_visible();
            }
            KeyCode::Down => {
                tree.move_down();
                self.ensure_explorer_visible();
            }
            KeyCode::Left => {
                if self.explorer_scroll_x > 0 {
                    self.explorer_scroll_x -= 1;
                }
            }
            KeyCode::Right => {
                // Max scroll — biar tidak scroll terlalu jauh
                let max_scroll = self.max_explorer_scroll_x();
                if self.explorer_scroll_x < max_scroll {
                    self.explorer_scroll_x += 1;
                }
            }
            KeyCode::Enter => {
                let is_dir = tree.selected_entry().map(|e| e.is_dir).unwrap_or(false);

                if is_dir {
                    tree.toggle_expand_selected();
                    self.ensure_explorer_visible();
                } else {
                    let path = tree.selected_entry().map(|e| e.path.clone());
                    if let Some(path) = path {
                        self.open_file_from_path(path);
                    }
                }
            }
            _ => {}
        }
    }

    fn ensure_explorer_visible(&mut self) {
        let Some(tree) = self.file_tree.as_ref() else {
            return;
        };

        let Some(selected_idx) = tree.selected_index() else {
            return;
        };

        let height = self.explorer_height as usize;
        if height == 0 {
            return;
        }

        let scroll = self.explorer_scroll;

        // Vertical
        if selected_idx < scroll {
            self.explorer_scroll = selected_idx;
        } else if selected_idx >= scroll + height {
            self.explorer_scroll = selected_idx - height + 1;
        }

        // Posisi horizontal dibiarkan sepenuhnya statis saat navigasi vertikal.
    }

    fn max_explorer_scroll_x(&self) -> usize {
        let Some(tree) = self.file_tree.as_ref() else {
            return 0;
        };

        let width = self.explorer_width as usize;
        if width == 0 {
            return 0;
        }

        let mut max_len = 0;

        for (_path, name, _is_dir, depth, _expanded) in tree.iter_visible() {
            let indent = depth * 2;
            let icon_len = 2;
            let marker_len = 2;
            let total_len = marker_len + indent + icon_len + name.chars().count();
            if total_len > max_len {
                max_len = total_len;
            }
        }

        max_len.saturating_sub(width)
    }

    fn open_file_from_path(&mut self, path: PathBuf) {
        match Document::open(&path) {
            Ok(doc) => {
                self.document = Some(doc);
                self.scroll_offset = 0;
                self.scroll_offset_x = 0;
                self.status_message = Some(format!("Opened: {}", path.display()));
                self.status_time = Some(Instant::now());

                self.load_editor();

                self.focus = Focus::Editor;
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
                self.status_time = Some(Instant::now());
            }
        }
    }

    fn handle_command(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Char('o') => {
                // Cek dirty sebelum membuka file baru
                if let Some(editor) = &self.editor
                    && editor.dirty
                {
                    self.status_message = Some("Unsaved changes! Save first (Esc w).".to_string());
                    self.status_time = Some(Instant::now());
                    return;
                }
                self.mode = Mode::Prompt;
                self.input_buffer.clear();
                self.status_message = Some("Open file:".to_string());
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('n') => {
                self.mode = Mode::PromptNewFile;
                self.input_buffer.clear();
                self.status_message = Some("New file:".to_string());
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('v') => {
                self.mode = Mode::PromptNewFolder;
                self.input_buffer.clear();
                self.status_message = Some("New folder:".to_string());
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('d') => {
                if let Some(tree) = self.file_tree.as_ref()
                    && let Some(entry) = tree.selected_entry()
                {
                    self.pending_delete = Some(entry.path.clone());
                    self.mode = Mode::ConfirmDelete;
                    self.status_message = Some(format!("Delete '{}'? (y/n)", entry.name));
                    self.status_time = Some(Instant::now());
                }
            }
            KeyCode::Char('t') => {
                self.mode = Mode::SelectTheme;
                self.theme_cursor = match self.theme {
                    Theme::Vesper => 0,
                    Theme::Gruvbox => 1,
                    Theme::Dracula => 2,
                    Theme::Nord => 3,
                };
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Char('w') => {
                if let (Some(doc), Some(editor)) = (self.document.as_ref(), self.editor.as_ref()) {
                    let content = editor.get_content();
                    let path = doc.path.clone();

                    match std::fs::write(&path, content) {
                        Ok(_) => {
                            if let Some(editor) = self.editor.as_mut() {
                                editor.dirty = false;
                            }
                            self.status_message = Some(format!("Saved: {}", path.display()));
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
                self.mode = self.saved_mode;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('e') => {
                // Selalu buka explorer dan fokus ke sana
                self.show_explorer = true;
                self.focus = Focus::Explorer;
                self.status_message = Some("Explorer ON".to_string());
                self.mode = self.saved_mode;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('m') => {
                // Buka popup pilih mode (cek saved_mode, bukan mode yang sudah ditimpa)
                self.keybinding_cursor = if self.saved_mode == Mode::Easy { 1 } else { 0 };
                self.mode = Mode::SelectKeybinding;
            }
            KeyCode::Char('q') => {
                // Sembunyikan explorer dan fokus ke editor
                self.show_explorer = false;
                self.focus = Focus::Editor;
                self.status_message = Some("Focus: Editor".to_string());
                self.mode = self.saved_mode;
                self.status_time = Some(Instant::now());
            }
            _ => {
                self.mode = self.saved_mode;
                self.status_message = None;
                self.status_time = None;
            }
        }
    }

    fn handle_prompt(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Enter => {
                let path = self.input_buffer.clone();
                match Document::open(&path) {
                    Ok(doc) => {
                        self.document = Some(doc);
                        self.scroll_offset = 0;
                        self.scroll_offset_x = 0;
                        self.status_message = Some(format!("Opened: {}", path));
                        self.load_editor();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_time = Some(Instant::now());
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_prompt_new_file(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Enter => {
                let name = self.input_buffer.trim().to_string();
                if name.is_empty() {
                    self.status_message = Some("Empty name".to_string());
                    self.status_time = Some(Instant::now());
                    self.mode = self.saved_mode;
                    self.input_buffer.clear();
                    return;
                }

                let result = if let Some(tree) = self.file_tree.as_mut() {
                    tree.create_file(&name)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "No file tree",
                    ))
                };

                match result {
                    Ok(path) => {
                        self.status_message = Some(format!("Created: {}", path.display()));
                        if let Ok(doc) = Document::open(&path) {
                            self.document = Some(doc);
                            self.scroll_offset = 0;
                            self.scroll_offset_x = 0;
                            self.load_editor();
                        }
                        self.focus = Focus::Explorer;
                        self.ensure_explorer_visible();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }

                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_time = Some(Instant::now());
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_prompt_new_folder(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Enter => {
                let name = self.input_buffer.trim().to_string();
                if name.is_empty() {
                    self.status_message = Some("Empty name".to_string());
                    self.status_time = Some(Instant::now());
                    self.mode = self.saved_mode;
                    self.input_buffer.clear();
                    return;
                }

                let result = if let Some(tree) = self.file_tree.as_mut() {
                    tree.create_folder(&name)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "No file tree",
                    ))
                };

                match result {
                    Ok(path) => {
                        self.status_message = Some(format!("Created: {}", path.display()));
                        self.focus = Focus::Explorer;
                        self.ensure_explorer_visible();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }

                self.mode = self.saved_mode;
                self.input_buffer.clear();
                self.status_time = Some(Instant::now());
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_confirm_delete(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let Some(path) = self.pending_delete.take() else {
                    self.mode = self.saved_mode;
                    return;
                };

                let result = if let Some(tree) = self.file_tree.as_mut() {
                    tree.delete_selected()
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "No file tree",
                    ))
                };

                match result {
                    Ok(deleted) => {
                        self.status_message = Some(format!("Deleted: {}", deleted.display()));

                        if let Some(doc) = self.document.as_ref()
                            && doc.path == path
                        {
                            self.document = None;
                            self.editor = None;
                        }

                        self.ensure_explorer_visible();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }

                self.mode = self.saved_mode;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.pending_delete = None;
                self.mode = self.saved_mode;
                self.status_message = Some("Cancelled".to_string());
                self.status_time = Some(Instant::now());
            }
            _ => {}
        }
    }

    fn handle_select_theme(&mut self, key: KeyEvent) {
        const THEMES: [Theme; 4] = [Theme::Vesper, Theme::Gruvbox, Theme::Dracula, Theme::Nord];

        let new_theme = match key.code {
            KeyCode::Up => {
                if self.theme_cursor > 0 {
                    self.theme_cursor -= 1;
                }
                None
            }
            KeyCode::Down => {
                if self.theme_cursor + 1 < THEMES.len() {
                    self.theme_cursor += 1;
                }
                None
            }
            KeyCode::Char('1') => Some(Theme::Vesper),
            KeyCode::Char('2') => Some(Theme::Gruvbox),
            KeyCode::Char('3') => Some(Theme::Dracula),
            KeyCode::Char('4') => Some(Theme::Nord),
            KeyCode::Enter => Some(THEMES[self.theme_cursor]),
            KeyCode::Esc => {
                self.mode = self.saved_mode;
                self.status_message = Some("Cancelled".to_string());
                self.status_time = Some(Instant::now());
                return;
            }
            _ => None,
        };

        if let Some(theme) = new_theme {
            self.theme = theme;
            // Hanya ganti tema, JANGAN bangun ulang editor dari Document
            let theme_colors = match theme {
                Theme::Vesper => vesper(),
                Theme::Gruvbox => gruvbox(),
                Theme::Dracula => dracula(),
                Theme::Nord => nord(),
            };
            if let Some(editor) = &mut self.editor {
                editor.set_theme(theme_colors);
            }

            let name = match theme {
                Theme::Vesper => "Vesper",
                Theme::Gruvbox => "Gruvbox",
                Theme::Dracula => "Dracula",
                Theme::Nord => "Nord",
            };

            // Save to config
            let mut config = Config::load();
            config.editor.theme = name.to_lowercase();
            config.editor.show_explorer = self.show_explorer;
            if let Err(e) = config.save() {
                self.status_message = Some(format!("Theme: {} (save failed: {})", name, e));
            } else {
                self.status_message = Some(format!("Theme: {}", name));
            }

            self.mode = self.saved_mode;
            self.status_time = Some(Instant::now());
        }
    }

    pub fn tick(&mut self) {
        if let Some(t) = self.status_time
            && t.elapsed().as_secs() >= 2
        {
            self.status_message = None;
            self.status_time = None;
        }
    }
}

fn detect_language(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") | Some("jsx") => "javascript",
        Some("ts") | Some("tsx") => "typescript",
        Some("go") => "go",
        Some("java") => "java",
        Some("c") => "c",
        Some("cpp") | Some("cc") | Some("cxx") | Some("h") | Some("hpp") => "cpp",
        Some("cs") => "c_sharp",
        Some("toml") => "toml",
        Some("json") => "json",
        Some("yaml") | Some("yml") => "yaml",
        Some("md") => "markdown",
        Some("html") => "html",
        Some("css") => "css",
        Some("sh") | Some("bash") => "shell",
        _ => "text",
    }
}
