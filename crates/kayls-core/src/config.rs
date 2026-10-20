use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub editor: EditorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_show_explorer")]
    pub show_explorer: bool,
}

fn default_theme() -> String {
    "vesper".to_string()
}

fn default_show_explorer() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig::default(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_explorer: default_show_explorer(),
        }
    }
}

impl Config {
    /// Get config file path: ~/.config/kayls/config.toml
    pub fn config_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let path = PathBuf::from(home)
            .join(".config")
            .join("kayls")
            .join("config.toml");
        Some(path)
    }

    /// Load config from file. If not exist, return default.
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save config to file. Create directory if not exist.
    pub fn save(&self) -> std::io::Result<()> {
        let Some(path) = Self::config_path() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME not set",
            ));
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

        fs::write(&path, content)?;
        Ok(())
    }
}
