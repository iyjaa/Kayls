use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
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

    // Tentukan area editor
    let editor_area: Rect = if app.show_explorer {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ])
            .split(rows[1]);

        frame.render_widget(
            Paragraph::new("src/\n  main.rs\n  lib.rs")
                .block(Block::default().borders(Borders::ALL).title(" Explorer ")),
            cols[0],
        );

        cols[1]
    } else {
        rows[1]
    };

    // Editor
    match &app.document {
        Some(doc) => {
            let lines = render_lines_with_cursor(doc);
            frame.render_widget(
                Paragraph::new(lines)
                    .block(Block::default().borders(Borders::ALL).title(" Editor ")),
                editor_area,
            );

            // Set posisi cursor di terminal
            let cursor_x = editor_area.x + 1 + doc.cursor.col as u16;
            let cursor_y = editor_area.y + 1 + doc.cursor.line as u16;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        None => {
            frame.render_widget(
                Paragraph::new(
                    "Welcome to Tscode!\n\nOpen a file: tscode <file>\nPress 'q' to quit.\nPress Ctrl+B to toggle explorer.",
                )
                .block(Block::default().borders(Borders::ALL).title(" Editor ")),
                editor_area,
            );
        }
    }

    // Status bar
    let cursor_info = app
        .document
        .as_ref()
        .map(|d| format!("{}:{}", d.cursor.line + 1, d.cursor.col + 1))
        .unwrap_or_else(|| "-:-".to_string());

    let status = format!(
        " NORMAL   Tscode {}   Rust   UTF-8   {} ",
        version, cursor_info
    );
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        rows[2],
    );
}

fn render_lines_with_cursor(doc: &tscode_core::Document) -> Vec<Line<'_>> {
    doc.lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == doc.cursor.line {
                // Baris ini ada cursor
                let col = doc.cursor.col;
                let chars: Vec<char> = line.chars().collect();

                if chars.is_empty() {
                    // Baris kosong — tampilkan cursor sebagai spasi ber-highlight
                    Line::from(vec![Span::styled(
                        " ",
                        Style::default().add_modifier(Modifier::REVERSED),
                    )])
                } else {
                    let before: String = chars.iter().take(col).collect();
                    let at: String = chars.get(col).map(|c| c.to_string()).unwrap_or_default();
                    let after: String = chars.iter().skip(col + 1).collect();

                    Line::from(vec![
                        Span::raw(before),
                        Span::styled(
                            if at.is_empty() { " ".to_string() } else { at },
                            Style::default().add_modifier(Modifier::REVERSED),
                        ),
                        Span::raw(after),
                    ])
                }
            } else {
                Line::from(line.clone())
            }
        })
        .collect()
}