use crate::app::Theme;
use crate::ui::{theme_accent_color, theme_bg_color, theme_fg_color};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
};
use std::fs;
use std::path::{Path, PathBuf};

pub struct PickerState {
    pub query: String,
    pub matches: Vec<PathBuf>,
    pub selected: usize,
    pub scroll: usize,
    pub visible_height: usize,
}

fn find_files_recursive(
    root: &Path,
    files: &mut Vec<PathBuf>,
    max_depth: usize,
    current_depth: usize,
) {
    if current_depth > max_depth {
        return;
    }
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name == ".git" || name == "target" || name == "node_modules" {
                    continue;
                }
                find_files_recursive(&path, files, max_depth, current_depth + 1);
            } else {
                files.push(path);
            }
        }
    }
}

impl PickerState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            matches: Vec::new(),
            selected: 0,
            scroll: 0,
            visible_height: 6,
        }
    }

    pub fn update_matches(&mut self, root: &std::path::Path) {
        self.matches.clear();
        self.selected = 0;
        self.scroll = 0;

        let mut all_files = Vec::new();
        find_files_recursive(root, &mut all_files, 5, 0);

        let query = self.query.to_lowercase();

        if query.is_empty() {
            self.matches = all_files.into_iter().take(50).collect();
            return;
        }

        for path in all_files {
            let path_str = path.to_string_lossy().to_lowercase();
            if path_str.contains(&query) {
                self.matches.push(path);
                if self.matches.len() >= 50 {
                    break;
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, root: &Path) -> Option<PathBuf> {
        match key.code {
            KeyCode::Esc => {
                self.query.clear();
                self.matches.clear();
                None
            }
            KeyCode::Enter => {
                if let Some(path) = self.matches.get(self.selected) {
                    let path = path.clone();
                    self.query.clear();
                    self.matches.clear();
                    Some(path)
                } else {
                    None
                }
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    // Scroll jika sorotan melewati batas atas
                    if self.selected < self.scroll {
                        self.scroll = self.selected;
                    }
                }
                None
            }
            KeyCode::Down => {
                if self.selected + 1 < self.matches.len() {
                    self.selected += 1;
                    // Scroll jika sorotan melewati batas bawah (10 item)
                    if self.selected >= self.scroll + 10 {
                        self.scroll += 1;
                    }
                }
                None
            }
            KeyCode::Char(c) => {
                self.query.push(c);
                self.update_matches(root);
                None
            }
            KeyCode::Backspace => {
                self.query.pop();
                self.update_matches(root);
                None
            }
            _ => None,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, theme: Theme) {
        let popup_area = crate::picker::centered_rect(80, 40, area);
        let accent = theme_accent_color(theme);
        let bg = theme_bg_color(theme);
        let fg = theme_fg_color(theme);

        let items: Vec<ListItem> = self
            .matches
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let display = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());
                let style = if i == self.selected {
                    Style::default().bg(accent).fg(Color::Rgb(20, 20, 30))
                } else {
                    Style::default().fg(fg)
                };
                ListItem::new(format!(" {}", display)).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" File Picker ({} matches) ", self.matches.len()))
                .border_style(Style::default().fg(accent))
                .style(Style::default().bg(bg)),
        );

        // ListState lokal untuk scroll otomatis
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(self.selected));

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(list, popup_area, &mut state);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
