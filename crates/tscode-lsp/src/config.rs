use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub extensions: Vec<String>,
    pub root_markers: Vec<String>,
}

impl ServerConfig {
    pub fn rust_analyzer() -> Self {
        // Prefer rustup-installed rust-analyzer (matches toolchain version)
        let ra_path = std::env::var("RUSTUP_HOME")
            .ok()
            .map(|h| {
                format!(
                    "{}/toolchains/stable-aarch64-unknown-linux-musl/bin/rust-analyzer",
                    h
                )
            })
            .filter(|p| std::path::Path::new(p).exists())
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| {
                        format!(
                        "{}/.rustup/toolchains/stable-aarch64-unknown-linux-musl/bin/rust-analyzer",
                        h
                    )
                    })
                    .filter(|p| std::path::Path::new(p).exists())
            })
            .unwrap_or_else(|| "rust-analyzer".to_string());

        Self {
            name: "rust-analyzer".to_string(),
            command: ra_path,
            args: vec![],
            extensions: vec!["rs".to_string()],
            root_markers: vec!["Cargo.toml".to_string()],
        }
    }

    pub fn matches_extension(&self, path: &std::path::Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            self.extensions.iter().any(|e| e == ext)
        } else {
            false
        }
    }

    pub fn find_root(&self, start: &std::path::Path) -> Option<PathBuf> {
        let mut current = start.to_path_buf();

        if current.is_file() {
            current = current.parent()?.to_path_buf();
        }

        loop {
            for marker in &self.root_markers {
                if current.join(marker).exists() {
                    return Some(current);
                }
            }

            if !current.pop() {
                return None;
            }
        }
    }
}
