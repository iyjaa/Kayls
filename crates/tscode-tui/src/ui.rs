use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Focus, Mode};

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
            .constraints([Constraint::Length(20), Constraint::Min(0)])
            .split(rows[1]);

        // Render file tree — border beda kalau fokus
        let tree_content = render_file_tree(app);
        let explorer_focused = app.focus == Focus::Explorer;

        let border_style = if explorer_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let explorer_title = if explorer_focused {
            " Explorer [*] "
        } else {
            " Explorer "
        };

        frame.render_widget(
            Paragraph::new(tree_content).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(explorer_title)
                    .border_style(border_style),
            ),
            cols[0],
        );

        cols[1]
    } else {
        rows[1]
    };

    // Editor
    match &app.document {
        Some(doc) => {
            let editor_focused = app.focus == Focus::Editor && app.show_explorer;

            render_editor(
                frame,
                doc,
                editor_area,
                app.scroll_offset,
                app.scroll_offset_x,
                editor_focused,
            );
        }
        None => {
            frame.render_widget(
                Paragraph::new(
                    "Welcome to Tscode!\n\nOpen a file: Esc then o\nPress Alt+Q to quit.",
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

    let focus_label = match app.focus {
        Focus::Editor => "EDITOR",
        Focus::Explorer => "EXPLORER",
    };

    let (status, status_style) = match app.mode {
        Mode::Normal => {
            if let Some(msg) = &app.status_message {
                (
                    format!(" {} ", msg),
                    Style::default().bg(Color::Green).fg(Color::Black),
                )
            } else {
                (
                    format!(
                        " NORMAL [{}]   {}   {}   v:{} h:{} ",
                        focus_label,
                        version,
                        cursor_info,
                        app.scroll_offset,
                        app.scroll_offset_x
                    ),
                    Style::default().bg(Color::DarkGray).fg(Color::White),
                )
            }
        }
        Mode::Command => (
            format!(
                " COMMAND   {} ",
                app.status_message
                    .as_deref()
                    .unwrap_or("(o)pen (w)rite (e)xplorer (Esc)cancel")
            ),
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
        Mode::Prompt => (
            format!(" Open file: {}_ ", app.input_buffer),
            Style::default().bg(Color::Cyan).fg(Color::Black),
        ),
    };

    frame.render_widget(Paragraph::new(status).style(status_style), rows[2]);
}

fn render_file_tree(app: &App) -> String {
    let Some(tree) = app.file_tree.as_ref() else {
        return "(no file tree)".to_string();
    };

    if tree.is_empty() {
        return "(empty)".to_string();
    }

    tree.entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let indent = "  ".repeat(entry.depth);
            let icon = if entry.is_dir { "+" } else { " " };
            let marker = if i == tree.selected { ">" } else { " " };
            format!("{}{}{} {}", marker, indent, icon, entry.name)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_editor(
    frame: &mut Frame,
    doc: &tscode_core::Document,
    area: Rect,
    scroll_y: usize,
    scroll_x: usize,
    focused: bool,
) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Viewport dimensi (kurangi border)
    let viewport_height = area.height.saturating_sub(2) as usize;
    let viewport_width = cols[1].width.saturating_sub(2) as usize;

    let start = scroll_y;
    let end = (scroll_y + viewport_height).min(doc.lines.len());

    // Nomor baris
    let numbers: String = (start + 1..=end)
        .map(|n| format!("{:>2} ", n))
        .collect::<Vec<_>>()
        .join("\n");

    frame.render_widget(
        Paragraph::new(numbers)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::TOP | Borders::RIGHT)),
        cols[0],
    );

    // Isi file dengan horizontal scroll
    let lines = render_lines_with_cursor(doc, start, end, scroll_x, viewport_width);

    // Border editor — beda kalau fokus
    let editor_border_style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let editor_title = if focused {
        " Editor [*] "
    } else {
        " Editor "
    };

    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(editor_title)
                .border_style(editor_border_style),
        ),
        cols[1],
    );

    // Posisi cursor terminal — hanya kalau editor fokus
    if focused {
        let cursor_screen_line = doc.cursor.line.saturating_sub(scroll_y);
        let cursor_screen_col = doc.cursor.col.saturating_sub(scroll_x);
        let cursor_x = cols[1].x + 1 + cursor_screen_col as u16;
        let cursor_y = cols[1].y + 1 + cursor_screen_line as u16;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

fn render_lines_with_cursor(
    doc: &tscode_core::Document,
    start: usize,
    end: usize,
    scroll_x: usize,
    viewport_width: usize,
) -> Vec<Line<'_>> {
    doc.lines
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .map(|(i, line)| {
            let chars: Vec<char> = line.chars().collect();
            let visible: Vec<char> = chars
                .iter()
                .skip(scroll_x)
                .take(viewport_width)
                .copied()
                .collect();

            if i == doc.cursor.line {
                let cursor_col_in_view = doc.cursor.col.saturating_sub(scroll_x);

                if visible.is_empty() || cursor_col_in_view >= visible.len() {
                    let text: String = visible.iter().collect();
                    Line::from(vec![
                        Span::raw(text),
                        Span::styled(" ", Style::default().add_modifier(Modifier::REVERSED)),
                    ])
                } else {
                    let before: String = visible.iter().take(cursor_col_in_view).collect();
                    let at: String = visible
                        .get(cursor_col_in_view)
                        .map(|c| c.to_string())
                        .unwrap_or_default();
                    let after: String = visible.iter().skip(cursor_col_in_view + 1).collect();

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
                Line::from(visible.into_iter().collect::<String>())
            }
        })
        .collect()
}