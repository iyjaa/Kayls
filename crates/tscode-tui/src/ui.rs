use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
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

        // Render file tree
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
    let editor_focused = app.focus == Focus::Editor;

    let editor_border_style = if editor_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let editor_title = if editor_focused {
        " Editor [*] "
    } else {
        " Editor "
    };

    // Render editor dari ratatui-code-editor
    if let Some(editor) = app.editor.as_ref() {
        let inner = Block::default()
            .borders(Borders::ALL)
            .title(editor_title)
            .border_style(editor_border_style);

        let inner_area = inner.inner(editor_area);

        frame.render_widget(inner, editor_area);
        frame.render_widget(editor, inner_area);

        // Set posisi cursor terminal
        if editor_focused {
            if let Some((cx, cy)) = editor.get_visible_cursor(&inner_area) {
                frame.set_cursor_position((cx, cy));
            }
        }
    } else {
        // Fallback kalau editor belum di-load
        let content = match &app.document {
            Some(doc) => doc.content_as_string(),
            None => {
                "Welcome to Tscode!\n\nOpen a file: Esc then o\nPress Alt+Q to quit.".to_string()
            }
        };

        frame.render_widget(
            Paragraph::new(content).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(editor_title)
                    .border_style(editor_border_style),
            ),
            editor_area,
        );
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
                        focus_label, version, cursor_info, app.scroll_offset, app.scroll_offset_x
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

    let visible = tree.visible_entries();
    let selected = tree.selected_path.as_ref();

    visible
        .iter()
        .map(|(path, name, is_dir, depth, expanded)| {
            let indent = "  ".repeat(*depth);
            let icon = if *is_dir {
                if *expanded {
                    "-"
                } else {
                    "+"
                }
            } else {
                " "
            };
            let marker = if Some(path) == selected { ">" } else { " " };
            format!("{}{}{} {}", marker, indent, icon, name)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
