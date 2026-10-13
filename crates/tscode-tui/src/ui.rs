use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, version: &str) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // Tab bar
    let tab_title = app
        .document
        .as_ref()
        .map(|d| {
            d.path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "[No Name]".to_string())
        })
        .unwrap_or_else(|| "[No File]".to_string());

    frame.render_widget(
        Paragraph::new(format!(" {} ", tab_title))
            .style(Style::default().bg(Color::Blue).fg(Color::White)),
        rows[0],
    );

    // Bagi isi jadi 2 kolom
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(rows[1]);

    // File tree
    frame.render_widget(
        Paragraph::new("src/\n  main.rs\n  lib.rs")
            .block(Block::default().borders(Borders::ALL).title(" Explorer ")),
        cols[0],
    );

    // Editor — tampilkan isi file
    let editor_content = match &app.document {
        Some(doc) => {
            if doc.lines.is_empty() {
                String::new()
            } else {
                doc.lines.join("\n")
            }
        }
        None => "Welcome to Tscode!\n\nOpen a file: tscode <file>\nPress 'q' to quit.".to_string(),
    };

    frame.render_widget(
        Paragraph::new(editor_content)
            .block(Block::default().borders(Borders::ALL).title(" Editor ")),
        cols[1],
    );

    // Status bar
    let status = format!(" NORMAL   Tscode {}   Rust   UTF-8 ", version);
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        rows[2],
    );
}