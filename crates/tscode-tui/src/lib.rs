mod app;
mod ui;

pub use app::App;

use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::time::Duration;
use tscode_core::Document;

pub fn run(version: &str, document: Option<Document>) -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run_loop(&mut terminal, version, document);
    ratatui::restore();
    result
}

fn run_loop(
    terminal: &mut DefaultTerminal,
    version: &str,
    document: Option<Document>,
) -> Result<()> {
    let mut app = App::new(document);

    while !app.should_quit {
        terminal.autoresize()?;

        let size = terminal.size()?;
        let editor_height = size.height.saturating_sub(4);
        let editor_width = if app.show_explorer {
            size.width.saturating_sub(17)
        } else {
            size.width.saturating_sub(5)
        };
        app.editor_height = editor_height;
        app.editor_width = editor_width;

        // Tick — clear status message kalau expired
        app.tick();

        terminal.draw(|frame| ui::render(frame, &app, version))?;

        // Poll dengan timeout 100ms — supaya tick() jalan rutin
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    app.handle_key(key);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    Ok(())
}