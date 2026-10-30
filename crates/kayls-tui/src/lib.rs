mod app;
mod picker;
mod search;
mod ui;

pub use app::App;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use kayls_core::Document;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use std::time::Duration;

/// Lebar Explorer dalam kolom (harus sama dengan ui.rs)
pub const EXPLORER_WIDTH: u16 = 20;

pub fn run(document: Option<Document>) -> Result<()> {
    // Pastikan mouse capture dimatikan dan terminal di-restore saat panic
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = execute!(std::io::stdout(), DisableMouseCapture);
        ratatui::restore();
        default_hook(info);
    }));

    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), EnableMouseCapture)?;
    let result = run_loop(&mut terminal, document);
    let _ = execute!(std::io::stdout(), DisableMouseCapture);
    ratatui::restore();
    result
}

fn run_loop(terminal: &mut DefaultTerminal, document: Option<Document>) -> Result<()> {
    let mut app = App::new(document);

    while !app.should_quit {
        terminal.autoresize()?;

        let size = terminal.size()?;
        let editor_height = size.height.saturating_sub(4);
        let editor_width = if app.show_explorer {
            size.width.saturating_sub(EXPLORER_WIDTH + 2)
        } else {
            size.width.saturating_sub(2)
        };
        app.editor_height = editor_height;
        app.editor_width = editor_width;
        app.explorer_height = editor_height;
        app.explorer_width = EXPLORER_WIDTH;

        let area = Rect {
            x: if app.show_explorer {
                EXPLORER_WIDTH + 1
            } else {
                1
            },
            y: 2,
            width: editor_width,
            height: editor_height,
        };
        app.editor_area = area;

        app.tick();

        terminal.draw(|frame| ui::render(frame, &app))?;

        // Adaptive poll: 100ms jika ada status message (agar bisa hilang),
        // 1000ms jika idle (hemat baterai).
        let poll_duration = if app.status_message.is_some() {
            Duration::from_millis(100)
        } else {
            Duration::from_millis(1000)
        };

        if event::poll(poll_duration)? {
            match event::read()? {
                Event::Paste(text) => {
                    app.paste(&text);
                }
                Event::Key(key) => {
                    app.handle_key(key);
                }
                Event::Resize(_, _) => {}
                Event::Mouse(mouse_event) => {
                    app.handle_mouse(mouse_event);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
