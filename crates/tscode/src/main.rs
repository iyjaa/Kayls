use anyhow::Result;

fn main() -> Result<()> {
    println!("Tscode {}", env!("TSC_VERSION"));
    tscode_tui::run(env!("TSC_VERSION"))
}