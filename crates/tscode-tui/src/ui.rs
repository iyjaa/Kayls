use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, _app: &App, version: &str) {
    // Bagi layar jadi 3 baris: tab bar, isi, status bar
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Tab bar
            Constraint::Min(0),     // Isi
            Constraint::Length(1),  // Status bar
        ])
        .split(frame.area());

    // Tab bar
    frame.render_widget(
        Paragraph::new(" main.rs ")
            .style(Style::default().bg(Color::Blue).fg(Color::White)),
        rows[0],
    );

    // Bagi isi jadi 2 kolom: file tree (kiri), editor (kanan)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),  // File tree
            Constraint::Min(0),      // Editor
        ])
        .split(rows[1]);

    // File tree
    frame.render_widget(
        Paragraph::new("src/\n  main.rs\n  lib.rs")
            .block(Block::default().borders(Borders::ALL).title(" Explorer ")),
        cols[0],
    );

    // Editor
    frame.render_widget(
        Paragraph::new("Welcome to Tscode!\n\nPress 'q' to quit.")
            .block(Block::default().borders(Borders::ALL).title(" Editor ")),
        cols[1],
    );

    // Status bar - pakai versi dari env
    let status = format!(
        " NORMAL   Tscode {}   Rust   UTF-8 ",
        version
    );
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        rows[2],
    );
}