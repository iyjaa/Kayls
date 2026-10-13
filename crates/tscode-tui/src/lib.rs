mod app;
mod ui;

pub use app::App;

use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
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
        terminal.draw(|frame| ui::render(frame, &app, version))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
    }

    Ok(())
}