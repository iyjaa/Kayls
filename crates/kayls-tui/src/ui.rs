use ratatui::widgets::{List, ListItem};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
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

    // Render editor dari kayls-editor
    if let Some(editor) = app.editor.as_ref() {
        let inner = Block::default()
            .borders(Borders::ALL)
            .title(editor_title)
            .border_style(editor_border_style);

        let inner_area = inner.inner(editor_area);

        frame.render_widget(inner, editor_area);
        frame.render_widget(editor, inner_area);

        // Set posisi cursor terminal
        if editor_focused
            && let Some((cx, cy)) = editor.get_visible_cursor(&inner_area) {
                frame.set_cursor_position((cx, cy));
            }
    } else {
        // Fallback kalau editor belum di-load
        if let Some(doc) = &app.document {
            frame.render_widget(
                Paragraph::new(doc.content_as_string()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(editor_title)
                        .border_style(editor_border_style),
                ),
                editor_area,
            );
        } else {
            let accent = theme_accent_color(app.theme);
            let text_color = theme_fg_color(app.theme);
            let dim_color = Color::Rgb(100, 100, 120);
            let version = env!("CARGO_PKG_VERSION");

            let mut menu_items = vec![
                Line::from(""),
                Line::from("        /\\_/\\        ").style(Style::default().fg(accent)),
                Line::from("       ( -   - )       ").style(Style::default().fg(accent)),
                Line::from("        >  w  <        ").style(Style::default().fg(accent)),
                Line::from(""),
                Line::from(format!("      Kayls v{}", version)).style(Style::default().add_modifier(Modifier::BOLD).fg(accent)),
                Line::from("  Terminal Studio Code").style(Style::default().fg(dim_color)),
                Line::from(""),
                Line::from(" ────────────────────────── ").style(Style::default().fg(dim_color)),
                Line::from(""),
            ];

            let shortcuts = vec![
                ("Open File", "Esc then o"),
                ("New File", "Esc then n"),
                ("Explorer", "Esc then e"),
                ("Theme", "Esc then t"),
                ("Mode", "Esc then m"),
                ("Quit", "Alt+Q"),
            ];

            for (label, key) in shortcuts {
                menu_items.push(Line::from(vec![
                    Span::styled(format!("  {:<12}", label), Style::default().fg(text_color)),
                    Span::styled(key, Style::default().fg(accent).add_modifier(Modifier::BOLD)),
                ]));
            }
            
            menu_items.push(Line::from(""));
            let mode_str = match app.mode {
                Mode::Easy => "Easy Mode",
                _ => "Command Mode",
            };
            menu_items.push(Line::from(format!("  Active: {}", mode_str)).style(Style::default().fg(dim_color)));

            frame.render_widget(
                Paragraph::new(menu_items)
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(editor_title)
                            .border_style(editor_border_style),
                    ),
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
        Mode::Easy => {
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
                        " EASY [{}]   {}   {}   v:{} h:{} ",
                        focus_label,
                        version,
                        cursor_info,
                        app.scroll_offset,
                        app.scroll_offset_x
                    ),
                    Style::default()
                        .bg(Color::Rgb(60, 60, 120))
                        .fg(Color::Rgb(200, 200, 255)),
                )
            }
        }
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
        Mode::SelectKeybinding => (
            " Select Keybinding Mode — ↑↓ navigate, Enter confirm, Esc cancel ".to_string(),
            Style::default()
                .bg(Color::Rgb(50, 70, 90))
                .fg(Color::Rgb(200, 220, 240)),
        ),
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
    if let Mode::SelectKeybinding = app.mode {
        render_keybinding_popup(frame, app);
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

    let color_selected_bg = Color::Rgb(60, 60, 80);

    let visible_iter = tree.iter_visible();
    let selected = tree.selected_path.as_ref();

    let scroll = app.explorer_scroll;
    let height = app.explorer_height as usize;

    // Kita enumerate dulu, lalu skip dan take. Tidak perlu .len().
    visible_iter
        .enumerate()
        .skip(scroll)
        .take(height.max(1))
        .map(|(_idx, (path, name, is_dir, depth, expanded))| {
            let is_selected = Some(path) == selected.map(|p| p.as_path());

            let icon = if is_dir {
                if expanded { "▾" } else { "▸" }
            } else {
                " "
            };

            let text_color = explorer_color(app.theme, is_dir, depth, name);

            let mut style = Style::default().fg(text_color);
            if is_selected {
                style = style.bg(color_selected_bg).add_modifier(Modifier::BOLD);
            }

            let marker = if is_selected { "> " } else { "  " };

            let scroll_x = app.explorer_scroll_x;
            let indent_str = "  ".repeat(depth);

            // Bangun teks lengkap untuk horizontal scroll
            let full_text = format!("{}{}{}{}", marker, indent_str, icon, name);
            let display_text: String = if scroll_x > 0 {
                full_text.chars().skip(scroll_x).collect()
            } else {
                full_text
            };

            Line::from(Span::styled(display_text, style))
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
fn render_keybinding_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 30, frame.area());
    let modes = ["Vim (Modal)", "Easy (VS Code)"];

    // Ambil warna dari tema aktif
    let accent = theme_accent_color(app.theme);
    let bg = theme_bg_color(app.theme);
    let fg = theme_fg_color(app.theme);

    let items: Vec<ListItem> = modes
        .iter()
        .enumerate()
        .map(|(i, mode)| {
            let style = if i == app.keybinding_cursor {
                Style::default()
                    .bg(accent)
                    .fg(Color::Rgb(20, 20, 30))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(fg)
            };
            ListItem::new(format!("  {}  ", mode)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent))
            .title(" Select Keybinding Mode ")
            .title_style(Style::default().fg(accent).add_modifier(Modifier::BOLD))
            .style(Style::default().bg(bg)),
    );

    frame.render_widget(Clear, area);
    frame.render_widget(list, area);
}

fn mix_color(c1: Color, c2: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    match (c1, c2) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
            let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
            let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => c1,
    }
}

fn explorer_color(theme: Theme, is_dir: bool, depth: usize, name: &str) -> Color {
    // Palet warna harmonis untuk setiap level kedalaman folder sesuai tema
    let folder_palette: &[Color] = match theme {
        Theme::Vesper => &[
            Color::Rgb(120, 160, 255), // L0: Bright Blue
            Color::Rgb(170, 140, 255), // L1: Soft Purple
            Color::Rgb(100, 200, 220), // L2: Cyan/Teal
            Color::Rgb(140, 150, 180), // L3+: Muted Slate
        ],
        Theme::Gruvbox => &[
            Color::Rgb(215, 153, 33),  // L0: Yellow
            Color::Rgb(214, 93, 14),   // L1: Orange
            Color::Rgb(184, 187, 38),  // L2: Green
            Color::Rgb(168, 153, 132), // L3+: Beige/Gray
        ],
        Theme::Dracula => &[
            Color::Rgb(189, 147, 249), // L0: Purple
            Color::Rgb(255, 121, 198), // L1: Pink
            Color::Rgb(139, 233, 253), // L2: Cyan
            Color::Rgb(120, 140, 180), // L3+: Muted Blue
        ],
        Theme::Nord => &[
            Color::Rgb(136, 192, 208), // L0: Cyan
            Color::Rgb(129, 161, 193), // L1: Light Blue
            Color::Rgb(94, 129, 172),  // L2: Deep Blue
            Color::Rgb(120, 130, 150), // L3+: Muted Gray
        ],
    };

    let depth_idx = depth.min(folder_palette.len() - 1);
    let folder_color = folder_palette[depth_idx];

    if is_dir {
        return folder_color;
    }

    // Untuk file: Gunakan warna ekstensi, lalu campur dengan warna folder induknya (25%)
    // agar terlihat menyatu dengan hierarki.
    let base_file_color = match theme {
        Theme::Vesper => Color::Rgb(197, 209, 192),
        Theme::Gruvbox => Color::Rgb(213, 196, 161),
        Theme::Dracula => Color::Rgb(216, 216, 224),
        Theme::Nord => Color::Rgb(200, 208, 220),
    };

    let ext_color = match name.rsplit('.').next().unwrap_or("") {
        "rs" => Color::Rgb(222, 165, 132), // Rust Orange
        "md" => Color::Rgb(81, 154, 186),  // Markdown Blue
        "toml" | "yml" | "yaml" => Color::Rgb(180, 140, 200), // Config Purple
        "json" => Color::Rgb(210, 210, 100), // JSON Yellow
        "gitignore" | "lock" => Color::Rgb(120, 120, 120), // Gray
        _ => base_file_color,
    };

    if depth == 0 {
        ext_color
    } else {
        // Campur warna ekstensi dengan warna folder tempat file itu berada
        mix_color(ext_color, folder_color, 0.25)
    }
}

fn theme_accent_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(212, 184, 150),
        Theme::Gruvbox => Color::Rgb(215, 153, 33),
        Theme::Dracula => Color::Rgb(212, 160, 200),
        Theme::Nord => Color::Rgb(168, 192, 216),
    }
}

fn theme_bg_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(42, 42, 62),
        Theme::Gruvbox => Color::Rgb(50, 48, 47),
        Theme::Dracula => Color::Rgb(52, 55, 70),
        Theme::Nord => Color::Rgb(59, 66, 82),
    }
}

fn theme_fg_color(theme: Theme) -> Color {
    match theme {
        Theme::Vesper => Color::Rgb(197, 209, 192),
        Theme::Gruvbox => Color::Rgb(213, 196, 161),
        Theme::Dracula => Color::Rgb(216, 216, 224),
        Theme::Nord => Color::Rgb(200, 208, 220),
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
