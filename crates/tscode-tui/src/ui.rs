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
        // File tree lebih kecil: 15% biar editor lega
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(12),   // File tree 12 kolom
                Constraint::Min(0),       // Editor sisa
            ])
            .split(rows[1]);

        frame.render_widget(
            Paragraph::new("src/\n  main\n  lib")
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
            render_editor(frame, doc, editor_area);
        }
        None => {
            frame.render_widget(
                Paragraph::new(
                    "Welcome to Tscode!\n\nOpen a file: tscode <file>\nPress 'q' to quit.",
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
        " NORMAL   {}   {}:{} ",
        version, cursor_info, ""
    );
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        rows[2],
    );
}

fn render_editor(frame: &mut Frame, doc: &tscode_core::Document, area: Rect) {
    // Bagi editor jadi 2: nomor baris + isi
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),   // nomor baris: 2 digit + spasi
            Constraint::Min(0),      // isi file
        ])
        .split(area);

    // Nomor baris
    let line_count = doc.lines.len();
    let numbers: String = (1..=line_count)
        .map(|n| format!("{:>2} ", n))
        .collect::<Vec<_>>()
        .join("\n");

    frame.render_widget(
        Paragraph::new(numbers)
            .style(Style::default().fg(Color::Gray)),
        cols[0],
    );

    // Isi file dengan cursor
    let lines = render_lines_with_cursor(doc);
    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" Editor ")),
        cols[1],
    );

    // Posisi cursor terminal
    let cursor_x = cols[1].x + 1 + doc.cursor.col as u16;
    let cursor_y = cols[1].y + 1 + doc.cursor.line as u16;
    frame.set_cursor_position((cursor_x, cursor_y));
}

fn render_lines_with_cursor(doc: &tscode_core::Document) -> Vec<Line<'_>> {
    doc.lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == doc.cursor.line {
                let col = doc.cursor.col;
                let chars: Vec<char> = line.chars().collect();

                if chars.is_empty() {
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