use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, Focus, Mode, Theme};

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
            .constraints([Constraint::Length(20), Constraint::Min(0)])
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

    // Render editor dari tscode-editor
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
                "
                Welcome to Tscode!\n\nOpen a Editor: Esc then e\nOpen a file: Esc then o\nPress Alt+Q to quit.
                ".to_string()
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
                    "(o)pen (n)ew file (v)new folder (d)elete (t)heme (w)rite (e)xplorer (Esc)cancel"
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
        Mode::SelectTheme => (
            " Select Theme — ↑↓ navigate, 1-4 select, Enter confirm, Esc cancel ".to_string(),
            Style::default()
                .bg(Color::Rgb(50, 70, 90))
                .fg(Color::Rgb(200, 220, 240)),
        ),
    };

    frame.render_widget(Paragraph::new(status).style(status_style), rows[2]);

    // Render theme popup
    if let Mode::SelectTheme = app.mode {
        render_theme_popup(frame, app);
    }
}

fn render_theme_popup(frame: &mut Frame, app: &App) {
    let themes = [
        (Theme::Vesper, "Vesper"),
        (Theme::Gruvbox, "Gruvbox"),
        (Theme::Dracula, "Dracula"),
        (Theme::Nord, "Nord"),
    ];

    let area = frame.area();

    // Popup dimensions
    let popup_width: u16 = 40.min(area.width);
    let popup_height: u16 = (themes.len() as u16 + 6).min(area.height);

    // Center popup
    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Build list lines
    let mut lines: Vec<Line> = Vec::new();

    for (i, (theme, name)) in themes.iter().enumerate() {
        let is_active = *theme == app.theme;
        let is_cursor = i == app.theme_cursor;

        let marker = if is_cursor { "▸ " } else { "  " };
        let active_marker = if is_active { " ●" } else { "" };
        let text = format!("{}{}. {}{}", marker, i + 1, name, active_marker);

        let style = if is_cursor {
            Style::default()
                .fg(focus_color(app.theme))
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default().fg(tab_fg_color(app.theme))
        } else {
            Style::default().fg(Color::Rgb(180, 180, 190))
        };

        lines.push(Line::from(Span::styled(text, style)));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select Theme ")
        .border_style(Style::default().fg(focus_color(app.theme)))
        .style(Style::default().bg(Color::Rgb(30, 30, 45)));

    let list_area = Rect {
        x: popup_area.x,
        y: popup_area.y,
        width: popup_area.width,
        height: popup_area.height.saturating_sub(2),
    };

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Left);
    frame.render_widget(paragraph, list_area);

    // Footer
    if popup_area.height >= 3 {
        let footer_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + popup_area.height.saturating_sub(2),
            width: popup_area.width.saturating_sub(2),
            height: 1,
        };

        let footer = Paragraph::new("↑↓ navigate · 1-4 select · Enter confirm · Esc cancel")
            .style(
                Style::default()
                    .bg(Color::Rgb(30, 30, 45))
                    .fg(Color::Rgb(140, 140, 160)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(footer, footer_area);
    }
}

fn render_file_tree(app: &App) -> Vec<Line<'static>> {
    let Some(tree) = app.file_tree.as_ref() else {
        return vec![Line::from("(no file tree)")];
    };

    if tree.is_empty() {
        return vec![Line::from("(empty)")];
    }

    let (color_file, color_folder, color_nested) = match app.theme {
        Theme::Vesper => (
            Color::Rgb(184, 212, 184),
            Color::Rgb(224, 192, 144),
            Color::Rgb(192, 168, 224),
        ),
        Theme::Gruvbox => (
            Color::Rgb(184, 187, 38),
            Color::Rgb(250, 189, 47),
            Color::Rgb(142, 192, 124),
        ),
        Theme::Dracula => (
            Color::Rgb(127, 212, 160),
            Color::Rgb(212, 160, 200),
            Color::Rgb(184, 160, 224),
        ),
        Theme::Nord => (
            Color::Rgb(168, 200, 168),
            Color::Rgb(168, 192, 216),
            Color::Rgb(168, 212, 212),
        ),
    };

    let color_selected_bg = Color::Rgb(60, 60, 80);

    let visible = tree.visible_entries();
    let selected = tree.selected_path.as_ref();

    let scroll = app.explorer_scroll;
    let height = app.explorer_height as usize;
    let end = if height > 0 {
        (scroll + height).min(visible.len())
    } else {
        visible.len()
    };

    visible
        .iter()
        .enumerate()
        .skip(scroll)
        .take(end.saturating_sub(scroll))
        .map(|(_idx, (path, name, is_dir, depth, expanded))| {
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

            // Horizontal scroll — marker tetap di kiri
            let full_text = format!("{}{}", prefix, name);
            let scroll_x = app.explorer_scroll_x;

            let scrolled_text: String = full_text.chars().skip(scroll_x).collect();

            Line::from(vec![
                Span::styled(marker, style),
                Span::styled(scrolled_text, style),
            ])
        })
        .collect()
}

fn focus_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(212, 184, 150),
        Theme::Gruvbox => Color::Rgb(215, 153, 33),
        Theme::Dracula => Color::Rgb(212, 160, 200),
        Theme::Nord => Color::Rgb(168, 192, 216),
    }
}

fn tab_bg_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(42, 42, 62),
        Theme::Gruvbox => Color::Rgb(50, 48, 47),
        Theme::Dracula => Color::Rgb(52, 55, 70),
        Theme::Nord => Color::Rgb(59, 66, 82),
    }
}

fn tab_fg_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(197, 209, 192),
        Theme::Gruvbox => Color::Rgb(213, 196, 161),
        Theme::Dracula => Color::Rgb(216, 216, 224),
        Theme::Nord => Color::Rgb(200, 208, 220),
    }
}
