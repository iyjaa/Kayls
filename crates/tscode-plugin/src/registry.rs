use tscode_plugin_api::{PluginError, PluginInfo, Result};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct PluginRegistry {
    plugins: HashMap<String, PluginEntry>,
}

pub struct PluginEntry {
    pub info: PluginInfo,
    pub path: PathBuf,
    pub enabled: bool,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn add(&mut self, info: PluginInfo, path: PathBuf) {
        self.plugins.insert(
            info.name.clone(),
            PluginEntry {
                info,
                path,
                enabled: true,
            },
        );
    }

    pub fn get(&self, name: &str) -> Option<&PluginEntry> {
        self.plugins.get(name)
    }

    pub fn list(&self) -> Vec<&PluginInfo> {
        self.plugins.values().map(|e| &e.info).collect()
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> Result<()> {
        match self.plugins.get_mut(name) {
            Some(entry) => {
                entry.enabled = enabled;
                Ok(())
            }
            None => Err(PluginError::NotFound(name.to_string())),
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
