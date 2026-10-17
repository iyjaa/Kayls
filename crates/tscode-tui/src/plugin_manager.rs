use anyhow::Result;
use std::path::PathBuf;
use tscode_plugin::{PluginLoader, PluginRegistry};
use tscode_plugin_api::{PluginApi, PluginEvent, PluginInfo};

pub struct PluginManager {
    loader: PluginLoader,
    registry: PluginRegistry,
    api: PluginApi,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::new(),
            registry: PluginRegistry::new(),
            api: PluginApi::new(),
        }
    }

    pub fn load_plugins_from_dir(&mut self, dir: &PathBuf) -> Result<usize> {
        if !dir.exists() {
            return Ok(0);
        }

        let mut loaded = 0;

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("wasm") {
                continue;
            }

            let name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            if let Ok(_module) = self.loader.load(&path) {
                let info = PluginInfo {
                    name: name.clone(),
                    version: "0.1.0".to_string(),
                    description: format!("Plugin from {}", path.display()),
                    author: "unknown".to_string(),
                };

                self.registry.add(info, path);
                loaded += 1;
            }
        }

        Ok(loaded)
    }

    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.registry.list()
    }

    pub fn api(&self) -> &PluginApi {
        &self.api
    }

    pub fn api_mut(&mut self) -> &mut PluginApi {
        &mut self.api
    }

    pub fn emit_event(&mut self, _event: &PluginEvent) {
        // Placeholder — plugin event dispatch
    }

    pub fn plugin_count(&self) -> usize {
        self.registry.list().len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
