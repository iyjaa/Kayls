use anyhow::Result;
use clap::Parser;
use kayls_core::Document;

#[derive(Parser)]
#[command(name = "kayls", version, about = "Terminal Studio Code")]
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

    kayls_tui::run(env!("KAYL_VERSION"), document)
}
