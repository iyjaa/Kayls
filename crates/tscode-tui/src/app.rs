use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tscode_core::Document;

pub struct App {
    pub should_quit: bool,
    pub document: Option<Document>,
}

impl App {
    pub fn new(document: Option<Document>) -> Self {
        Self {
            should_quit: false,
            document,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }
}