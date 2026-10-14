use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use ratatui_code_editor::editor::Editor;
use ratatui_code_editor::theme::vesper;
use std::time::Instant;
use tscode_core::{Document, FileTree};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Command,
    Prompt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Editor,
    Explorer,
}

pub struct App {
    pub should_quit: bool,
    pub document: Option<Document>,
    pub editor: Option<Editor>,
    pub editor_area: Rect,
    pub file_tree: Option<FileTree>,
    pub show_explorer: bool,
    pub focus: Focus,
    pub scroll_offset: usize,
    pub scroll_offset_x: usize,
    pub editor_height: u16,
    pub editor_width: u16,
    pub mode: Mode,
    pub input_buffer: String,
    pub status_message: Option<String>,
    pub status_time: Option<Instant>,
}

impl App {
    pub fn new(document: Option<Document>) -> Self {
        let mut app = Self {
            should_quit: false,
            document,
            editor: None,
            editor_area: Rect::default(),
            file_tree: None,
            show_explorer: true,
            focus: Focus::Editor,
            scroll_offset: 0,
            scroll_offset_x: 0,
            editor_height: 0,
            editor_width: 0,
            mode: Mode::Normal,
            input_buffer: String::new(),
            status_message: None,
            status_time: None,
        };

        app.load_file_tree();
        app.load_editor();
        app
    }

    fn load_file_tree(&mut self) {
        // Root = current directory (tidak berubah tiap buka file)
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

        match Editor::new(lang, &content, vesper()) {
            Ok(ed) => {
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

        // Alt+Q — quit
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
            }
            KeyCode::Down => {
                tree.move_down();
            }
            KeyCode::Enter => {
                let is_dir = tree.selected_entry().map(|e| e.is_dir).unwrap_or(false);

                if is_dir {
                    tree.toggle_expand_selected();
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

    fn open_file_from_path(&mut self, path: std::path::PathBuf) {
        match Document::open(&path) {
            Ok(doc) => {
                self.document = Some(doc);
                self.scroll_offset = 0;
                self.scroll_offset_x = 0;
                self.status_message = Some(format!("Opened: {}", path.display()));
                self.status_time = Some(Instant::now());

                // TIDAK reload file tree — biar root tetap
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
                self.show_explorer = !self.show_explorer;

                if self.show_explorer {
                    self.focus = Focus::Explorer;
                    self.status_message = Some("Explorer ON".to_string());
                } else {
                    self.focus = Focus::Editor;
                    self.status_message = Some("Explorer OFF".to_string());
                }

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
                        // TIDAK reload file tree — biar root tetap
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