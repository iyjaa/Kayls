use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use std::path::PathBuf;
use std::time::Instant;
use kayls_core::{Config, Document, FileTree};
use kayls_editor::editor::Editor;
use kayls_editor::theme::{dracula, gruvbox, nord, vesper};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Command,
    Prompt,
    PromptNewFile,
    PromptNewFolder,
    ConfirmDelete,
    SelectTheme,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Editor,
    Explorer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Vesper,
    Gruvbox,
    Dracula,
    Nord,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Vesper
    }
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
    pub scroll_offset: usize,
    pub scroll_offset_x: usize,
    pub explorer_scroll: usize,
    pub explorer_scroll_x: usize,
    pub explorer_height: u16,
    pub explorer_width: u16,
    pub editor_height: u16,
    pub clipboard: String,
    pub editor_width: u16,
    pub mode: Mode,
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
            mode: Mode::Normal,
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

        let content = doc.content_as_string();
        let lang = detect_language(&doc.path);

        let theme_colors = match self.theme {
            Theme::Vesper => vesper(),
            Theme::Gruvbox => gruvbox(),
            Theme::Dracula => dracula(),
            Theme::Nord => nord(),
        };

        match Editor::new(lang, &content, theme_colors) {
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

        if key.modifiers.contains(KeyModifiers::ALT) {
            if let KeyCode::Char('q') = key.code {
                self.should_quit = true;
                return;
            }
        }

        match self.mode {
            Mode::Normal => self.handle_normal(key),
            Mode::Command => self.handle_command(key),
            Mode::Prompt => self.handle_prompt(key),
            Mode::PromptNewFile => self.handle_prompt_new_file(key),
            Mode::PromptNewFolder => self.handle_prompt_new_folder(key),
            Mode::ConfirmDelete => self.handle_confirm_delete(key),
            Mode::SelectTheme => self.handle_select_theme(key),
        }
    }

    pub fn paste(&mut self, text: &str) {
        if let Some(editor) = &mut self.editor {
            use kayls_editor::actions::InsertText;
            editor.apply(InsertText { text: text.to_string() });
        }
    }

    fn handle_normal(&mut self, key: KeyEvent) {
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
            
            // Ctrl+Y untuk Copy ke App.clipboard (antar-file)
            if key.code == KeyCode::Char('y') && key.modifiers.contains(KeyModifiers::CONTROL) {
                use kayls_editor::actions::Copy;
                editor.apply(Copy {});
                // Simpan ke clipboard App
                if let Ok(text) = editor.get_clipboard() {
                    self.clipboard = text;
                }
                return;
            }
            // Ctrl+P untuk Paste dari App.clipboard (antar-file)
            if key.code == KeyCode::Char('p') && key.modifiers.contains(KeyModifiers::CONTROL) {
                if !self.clipboard.is_empty() {
                    use kayls_editor::actions::InsertText;
                    editor.apply(InsertText { text: self.clipboard.clone() });
                }
                return;
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

        // Horizontal — auto-scroll saat nama panjang
        let width = self.explorer_width as usize;
        if width == 0 {
            return;
        }

        let visible = tree.visible_entries();
        if let Some((_path, name, _is_dir, depth, _expanded)) = visible.get(selected_idx) {
            let indent = depth * 2;
            let icon_len = 2; // "▸ " atau "  "
            let marker_len = 2; // "> " atau "  "
            let total_len = marker_len + indent + icon_len + name.chars().count();

            let scroll_x = self.explorer_scroll_x;

            // Kalau item lebih panjang dari width, scroll kanan
            if total_len > scroll_x + width {
                self.explorer_scroll_x = total_len.saturating_sub(width);
            } else if scroll_x > 0 && total_len <= scroll_x {
                // Kalau item lebih pendek dari scroll, scroll kiri
                self.explorer_scroll_x = 0;
            }

            // Auto-scroll untuk indent dalam
            let indent_scroll_threshold = indent + icon_len + marker_len;
            if indent_scroll_threshold > scroll_x + width / 2 {
                self.explorer_scroll_x = indent_scroll_threshold.saturating_sub(width / 2);
            }
        }
    }

    fn max_explorer_scroll_x(&self) -> usize {
        let Some(tree) = self.file_tree.as_ref() else {
            return 0;
        };

        let visible = tree.visible_entries();
        let width = self.explorer_width as usize;
        if width == 0 {
            return 0;
        }

        let mut max_len = 0;

        for (_path, name, _is_dir, depth, _expanded) in visible.iter() {
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
                self.mode = Mode::Normal;
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Char('o') => {
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
                if let Some(tree) = self.file_tree.as_ref() {
                    if let Some(entry) = tree.selected_entry() {
                        self.pending_delete = Some(entry.path.clone());
                        self.mode = Mode::ConfirmDelete;
                        self.status_message = Some(format!("Delete '{}'? (y/n)", entry.name));
                        self.status_time = Some(Instant::now());
                    }
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
                            self.status_message = Some(format!("Saved: {}", path.display()));
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
                self.mode = Mode::Normal;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('e') => {
                // Selalu buka explorer dan fokus ke sana
                self.show_explorer = true;
                self.focus = Focus::Explorer;
                self.status_message = Some("Explorer ON".to_string());
                self.mode = Mode::Normal;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('q') => {
                // Sembunyikan explorer dan fokus ke editor
                self.show_explorer = false;
                self.focus = Focus::Editor;
                self.status_message = Some("Focus: Editor".to_string());
                self.mode = Mode::Normal;
                self.status_time = Some(Instant::now());
            }
            _ => {
                self.mode = Mode::Normal;
                self.status_message = None;
                self.status_time = None;
            }
        }
    }

    fn handle_prompt(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
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
                self.mode = Mode::Normal;
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
                self.mode = Mode::Normal;
                self.input_buffer.clear();
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Enter => {
                let name = self.input_buffer.trim().to_string();
                if name.is_empty() {
                    self.status_message = Some("Empty name".to_string());
                    self.status_time = Some(Instant::now());
                    self.mode = Mode::Normal;
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

                self.mode = Mode::Normal;
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
                self.mode = Mode::Normal;
                self.input_buffer.clear();
                self.status_message = None;
                self.status_time = None;
            }
            KeyCode::Enter => {
                let name = self.input_buffer.trim().to_string();
                if name.is_empty() {
                    self.status_message = Some("Empty name".to_string());
                    self.status_time = Some(Instant::now());
                    self.mode = Mode::Normal;
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

                self.mode = Mode::Normal;
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
                    self.mode = Mode::Normal;
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

                        if let Some(doc) = self.document.as_ref() {
                            if doc.path == path {
                                self.document = None;
                                self.editor = None;
                            }
                        }

                        self.ensure_explorer_visible();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }

                self.mode = Mode::Normal;
                self.status_time = Some(Instant::now());
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.pending_delete = None;
                self.mode = Mode::Normal;
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
                self.mode = Mode::Normal;
                self.status_message = Some("Cancelled".to_string());
                self.status_time = Some(Instant::now());
                return;
            }
            _ => None,
        };

        if let Some(theme) = new_theme {
            self.theme = theme;
            self.load_editor();

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

            self.mode = Mode::Normal;
            self.status_time = Some(Instant::now());
        }
    }

    pub fn tick(&mut self) {
        if let Some(t) = self.status_time {
            if t.elapsed().as_secs() >= 2 {
                self.status_message = None;
                self.status_time = None;
            }
        }
    }
}

fn detect_language(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("go") => "go",
        Some("c") => "c",
        Some("cpp") | Some("cc") | Some("cxx") => "cpp",
        Some("h") | Some("hpp") => "cpp",
        Some("toml") => "toml",
        Some("json") => "json",
        Some("md") => "markdown",
        Some("html") => "html",
        Some("css") => "css",
        Some("sh") | Some("bash") => "bash",
        _ => "text",
    }
}
