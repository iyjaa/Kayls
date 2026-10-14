use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
        app
    }

    fn load_file_tree(&mut self) {
        let Some(doc) = self.document.as_ref() else {
            return;
        };

        let parent = if doc.path.is_file() {
            doc.path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .map(|p| p.to_path_buf())
        } else if doc.path.is_dir() {
            Some(doc.path.clone())
        } else {
            None
        };

        let parent = parent.or_else(|| std::env::current_dir().ok());

        if let Some(parent) = parent {
            match FileTree::from_path(&parent) {
                Ok(tree) => {
                    self.file_tree = Some(tree);
                }
                Err(e) => {
                    self.status_message = Some(format!("FileTree error: {}", e));
                }
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

        // Arrow keys: tergantung focus
        match self.focus {
            Focus::Editor => self.handle_editor_keys(key),
            Focus::Explorer => self.handle_explorer_keys(key),
        }
    }

    fn handle_editor_keys(&mut self, key: KeyEvent) {
        if let Some(doc) = self.document.as_mut() {
            match key.code {
                KeyCode::Up => {
                    doc.cursor.move_up();
                    Self::clamp_cursor(doc);
                    self.ensure_cursor_visible();
                    return;
                }
                KeyCode::Down => {
                    doc.cursor.move_down(doc.line_count());
                    Self::clamp_cursor(doc);
                    self.ensure_cursor_visible();
                    return;
                }
                KeyCode::Left => {
                    doc.cursor.move_left();
                    self.ensure_cursor_visible();
                    return;
                }
                KeyCode::Right => {
                    doc.cursor.move_right(doc.current_line_len());
                    self.ensure_cursor_visible();
                    return;
                }
                KeyCode::Backspace => {
                    doc.delete_char();
                    self.ensure_cursor_visible();
                    return;
                }
                KeyCode::Enter => {
                    return;
                }
                _ => {}
            }
        }

        if key.modifiers.is_empty() {
            if let KeyCode::Char(c) = key.code {
                if let Some(doc) = self.document.as_mut() {
                    doc.insert_char(c);
                }
                self.ensure_cursor_visible();
            }
        }
    }

    fn handle_explorer_keys(&mut self, key: KeyEvent) {
        if let Some(tree) = self.file_tree.as_mut() {
            match key.code {
                KeyCode::Up => {
                    tree.move_up();
                    return;
                }
                KeyCode::Down => {
                    tree.move_down();
                    return;
                }
                _ => {}
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
                if let Some(doc) = self.document.as_mut() {
                    match doc.save() {
                        Ok(_) => {
                            self.status_message = Some(format!("Saved: {}", doc.path.display()));
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
                // Toggle explorer + switch focus
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
                        self.load_file_tree();
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

    fn clamp_cursor(doc: &mut Document) {
        let max = doc.current_line_len();
        if doc.cursor.col > max {
            doc.cursor.col = max;
        }
    }

    pub fn ensure_cursor_visible(&mut self) {
        if self.editor_height == 0 {
            return;
        }

        let Some(doc) = self.document.as_ref() else {
            return;
        };

        let cursor_line = doc.cursor.line;
        let cursor_col = doc.cursor.col;
        let h = self.editor_height as usize;
        let w = self.editor_width as usize;

        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        }
        let bottom = self.scroll_offset + h;
        if cursor_line >= bottom {
            self.scroll_offset = cursor_line - h + 1;
        }

        if w > 0 {
            if cursor_col < self.scroll_offset_x {
                self.scroll_offset_x = cursor_col;
            }
            let right = self.scroll_offset_x + w;
            if cursor_col >= right {
                self.scroll_offset_x = cursor_col - w + 1;
            }
        }
    }
}