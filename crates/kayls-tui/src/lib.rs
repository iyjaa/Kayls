mod app;
mod picker;
mod ui;

pub use app::App;

use anyhow::Result;
use crossterm::event::{self, Event, EnableMouseCapture, DisableMouseCapture};
use crossterm::execute;
use kayls_core::Document;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use std::time::Duration;

pub fn run(document: Option<Document>) -> Result<()> {
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
            size.width.saturating_sub(25)
        } else {
            size.width.saturating_sub(5)
        };
        app.editor_height = editor_height;
        app.editor_width = editor_width;
        app.explorer_height = editor_height;
        app.explorer_width = 23;

        let area = Rect {
            x: if app.show_explorer { 24 } else { 4 },
            y: 2,
            width: editor_width,
            height: editor_height,
        };
        app.editor_area = area;

        app.tick();

        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::poll(Duration::from_millis(100))? {
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
