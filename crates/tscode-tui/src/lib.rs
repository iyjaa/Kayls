mod app;
mod ui;

pub use app::App;

use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

pub fn run(version: &str) -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run_loop(&mut terminal, version);
    ratatui::restore();
    result
}

fn run_loop(terminal: &mut DefaultTerminal, version: &str) -> Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| ui::render(frame, &app, version))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
    }

    Ok(())
}