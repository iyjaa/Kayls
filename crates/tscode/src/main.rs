use anyhow::Result;
use clap::Parser;
use tscode_core::Document;

#[derive(Parser)]
#[command(name = "tscode", version, about = "Terminal Studio Code")]
struct Cli {
    /// File to open
    file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let document = match cli.file {
        Some(path) => match Document::open(&path) {
            Ok(doc) => Some(doc),
            Err(e) => {
                eprintln!("Error opening {}: {}", path, e);
                std::process::exit(1);
            }
        },
        None => None,
    };

    tscode_tui::run(env!("TSC_VERSION"), document)
}
