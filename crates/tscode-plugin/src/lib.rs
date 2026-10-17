pub mod loader;
pub mod registry;

pub use loader::PluginLoader;
pub use registry::PluginRegistry;

// Re-export from API
pub use tscode_plugin_api::{Plugin, PluginApi, PluginError, PluginInfo, PluginAction, PluginEvent, Result};