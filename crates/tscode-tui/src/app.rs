use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tscode_core::Document;

pub struct App {
    pub should_quit: bool,
    pub document: Option<Document>,
    pub show_explorer: bool,
}

impl App {
    pub fn new(document: Option<Document>) -> Self {
        Self {
            should_quit: false,
            document,
            show_explorer: true,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Ctrl+\ — toggle explorer
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if let KeyCode::Tab = key.code {
                self.show_explorer = !self.show_explorer;
                return;
            }
        }

        // Arrow keys — gerakkan cursor
        if let Some(doc) = self.document.as_mut() {
            match key.code {
                KeyCode::Up => {
                    doc.cursor.move_up();
                    Self::clamp_cursor(doc);
                    return;
                }
                KeyCode::Down => {
                    doc.cursor.move_down(doc.line_count());
                    Self::clamp_cursor(doc);
                    return;
                }
                KeyCode::Left => {
                    doc.cursor.move_left();
                    return;
                }
                KeyCode::Right => {
                    doc.cursor.move_right(doc.current_line_len());
                    return;
                }
                _ => {}
            }
        }

        // Quit
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn clamp_cursor(doc: &mut Document) {
        let max = doc.current_line_len();
        if doc.cursor.col > max {
            doc.cursor.col = max;
        }
    }
}