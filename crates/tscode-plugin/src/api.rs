use crate::event::{PluginAction, PluginEvent};
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

pub trait Plugin: Send + Sync {
    fn info(&self) -> PluginInfo;

    fn on_load(&mut self, api: &mut PluginApi) -> Result<()>;

    fn on_event(&mut self, event: &PluginEvent) -> Option<PluginAction>;

    fn on_unload(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct PluginApi {
    pub registered_themes: Vec<String>,
    pub registered_snippets: Vec<String>,
    pub registered_commands: Vec<String>,
}

impl PluginApi {
    pub fn new() -> Self {
        Self {
            registered_themes: Vec::new(),
            registered_snippets: Vec::new(),
            registered_commands: Vec::new(),
        }
    }

    pub fn register_theme(&mut self, name: &str) {
        self.registered_themes.push(name.to_string());
    }

    pub fn register_snippet(&mut self, name: &str) {
        self.registered_snippets.push(name.to_string());
    }

    pub fn register_command(&mut self, name: &str) {
        self.registered_commands.push(name.to_string());
    }
}

impl Default for PluginApi {
    fn default() -> Self {
        Self::new()
    }
}
