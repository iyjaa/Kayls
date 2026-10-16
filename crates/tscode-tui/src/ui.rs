use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
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
        Paragraph::new(format!(" {} ", tab_title)).style(
            Style::default()
                .bg(tab_bg_color(app.theme))
                .fg(tab_fg_color(app.theme)),
        ),
        rows[0],
    );

    // Tentukan area editor
    let editor_area: Rect = if app.show_explorer {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(25), Constraint::Min(0)])
            .split(rows[1]);

        // Render file tree
        let tree_content = render_file_tree(app);
        let explorer_focused = app.focus == Focus::Explorer;

        let border_style = if explorer_focused {
            Style::default().fg(focus_color(app.theme))
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let explorer_title = if explorer_focused {
            " Explorer [*] "
        } else {
            " Explorer "
        };

        frame.render_widget(
            Paragraph::new(Text::from(tree_content)).block(
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
        Style::default().fg(focus_color(app.theme))
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
            None => "Welcome to Tscode!\n\nOpen a file: Esc then o\nPress Alt+Q to quit.".to_string(),
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
                    Style::default()
                        .bg(Color::Rgb(60, 90, 60))
                        .fg(Color::Rgb(200, 230, 200)),
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
                    Style::default()
                        .bg(Color::Rgb(50, 50, 70))
                        .fg(Color::Rgb(200, 200, 220)),
                )
            }
        }
        Mode::Command => (
            format!(
                " COMMAND   {} ",
                app.status_message.as_deref().unwrap_or(
                    "(o)pen (n)ew file (v)new folder (d)elete (w)rite (e)xplorer (Esc)cancel"
                )
            ),
            Style::default()
                .bg(Color::Rgb(90, 80, 40))
                .fg(Color::Rgb(230, 220, 180)),
        ),
        Mode::Prompt => (
            format!(" Open file: {}_ ", app.input_buffer),
            Style::default()
                .bg(Color::Rgb(50, 70, 90))
                .fg(Color::Rgb(200, 220, 240)),
        ),
        Mode::PromptNewFile => (
            format!(" New file: {}_ ", app.input_buffer),
            Style::default()
                .bg(Color::Rgb(70, 50, 90))
                .fg(Color::Rgb(220, 200, 240)),
        ),
        Mode::PromptNewFolder => (
            format!(" New folder: {}_ ", app.input_buffer),
            Style::default()
                .bg(Color::Rgb(70, 50, 90))
                .fg(Color::Rgb(220, 200, 240)),
        ),
        Mode::ConfirmDelete => {
            let name = app
                .pending_delete
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "?".to_string());
            (
                format!(" Delete '{}'? (y/n) ", name),
                Style::default()
                    .bg(Color::Rgb(90, 50, 50))
                    .fg(Color::Rgb(240, 200, 200)),
            )
        }
    };

    frame.render_widget(Paragraph::new(status).style(status_style), rows[2]);
}

fn render_file_tree(app: &App) -> Vec<Line<'static>> {
    use crate::app::Theme;

    let Some(tree) = app.file_tree.as_ref() else {
        return vec![Line::from("(no file tree)")];
    };

    if tree.is_empty() {
        return vec![Line::from("(empty)")];
    }

    // Warna soft berdasarkan theme
    let (color_file, color_folder, color_nested) = match app.theme {
        Theme::Vesper => (
            Color::Rgb(184, 212, 184), // #b8d4b8 (hijau soft)
            Color::Rgb(224, 192, 144), // #e0c090 (orange soft)
            Color::Rgb(192, 168, 224), // #c0a8e0 (ungu soft)
        ),
        Theme::Gruvbox => (
            Color::Rgb(184, 187, 38),  // #b8bb26 (hijau soft)
            Color::Rgb(250, 189, 47),  // #fabd2f (kuning soft)
            Color::Rgb(142, 192, 124), // #8ec07c (hijau mint)
        ),
        Theme::Dracula => (
            Color::Rgb(127, 212, 160), // #7fd4a0 (hijau soft)
            Color::Rgb(212, 160, 200), // #d4a0c8 (pink soft)
            Color::Rgb(184, 160, 224), // #b8a0e0 (ungu soft)
        ),
        Theme::Nord => (
            Color::Rgb(168, 200, 168), // #a8c8a8 (hijau soft)
            Color::Rgb(168, 192, 216), // #a8c0d8 (biru soft)
            Color::Rgb(168, 212, 212), // #a8d4d4 (cyan soft)
        ),
    };

    let color_selected_bg = Color::Rgb(60, 60, 80);

    let visible = tree.visible_entries();
    let selected = tree.selected_path.as_ref();

    visible
        .iter()
        .map(|(path, name, is_dir, depth, expanded)| {
            let is_selected = Some(path) == selected;

            let icon = if *is_dir {
                if *expanded {
                    "▾"
                } else {
                    "▸"
                }
            } else {
                " "
            };

            let indent = "  ".repeat(*depth);
            let prefix = format!("{}{} ", indent, icon);

            let text_color = if *is_dir {
                if *depth == 0 {
                    color_folder
                } else {
                    color_nested
                }
            } else {
                color_file
            };

            let mut style = Style::default().fg(text_color);
            if is_selected {
                style = style.bg(color_selected_bg).add_modifier(Modifier::BOLD);
            }

            let marker = if is_selected { "> " } else { "  " };

            Line::from(vec![
                Span::styled(marker, style),
                Span::styled(prefix, style),
                Span::styled(name.clone(), style),
            ])
        })
        .collect()
}

fn focus_color(theme: crate::app::Theme) -> Color {
    use crate::app::Theme;

    match theme {
        Theme::Vesper => Color::Rgb(212, 184, 150),  // #d4b896 (orange soft)
        Theme::Gruvbox => Color::Rgb(215, 153, 33),  // #d79921 (kuning soft)
        Theme::Dracula => Color::Rgb(212, 160, 200), // #d4a0c8 (pink soft)
        Theme::Nord => Color::Rgb(168, 192, 216),    // #a8c0d8 (biru soft)
    }
}

fn tab_bg_color(theme: crate::app::Theme) -> Color {
    use crate::app::Theme;

    match theme {
        Theme::Vesper => Color::Rgb(42, 42, 62),  // #2a2a3e
        Theme::Gruvbox => Color::Rgb(50, 48, 47), // #32302f
        Theme::Dracula => Color::Rgb(52, 55, 70), // #343746
        Theme::Nord => Color::Rgb(59, 66, 82),    // #3b4252
    }
}

fn tab_fg_color(theme: crate::app::Theme) -> Color {
    use crate::app::Theme;

    match theme {
        Theme::Vesper => Color::Rgb(197, 209, 192),  // #c5d1c0
        Theme::Gruvbox => Color::Rgb(213, 196, 161), // #d5c4a1
        Theme::Dracula => Color::Rgb(216, 216, 224), // #d8d8e0
        Theme::Nord => Color::Rgb(200, 208, 220),    // #c8d0dc
    }
}