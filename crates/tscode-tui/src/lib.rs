mod app;
mod plugin_manager;
mod ui;

pub use app::App;
pub use plugin_manager::PluginManager;

use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::layout::Rect;
use ratatui::DefaultTerminal;
use std::path::PathBuf;
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

    // Load plugins
    let plugin_dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/tscode/plugins"))
        .unwrap_or_else(|_| PathBuf::from(".tscode/plugins"));

    let mut plugin_manager = PluginManager::new();
    match plugin_manager.load_plugins_from_dir(&plugin_dir) {
        Ok(count) => {
            if count > 0 {
                eprintln!("[TSCode] Loaded {} plugin(s)", count);
            }
        }
        Err(e) => {
            eprintln!("[TSCode] Plugin load error: {}", e);
        }
    }

    app.plugin_manager = Some(plugin_manager);

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

        terminal.draw(|frame| ui::render(frame, &app, version))?;

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
