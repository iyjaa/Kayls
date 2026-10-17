use crate::error::PluginError;
use crate::Result;
use std::path::Path;

pub struct PluginLoader {
    engine: wasmtime::Engine,
}

impl PluginLoader {
    pub fn new() -> Self {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        let engine = wasmtime::Engine::new(&config).expect("Failed to create WASM engine");
        Self { engine }
    }

    pub fn load(&self, path: &Path) -> Result<wasmtime::Module> {
        if !path.exists() {
            return Err(PluginError::NotFound(path.display().to_string()));
        }
        let module = wasmtime::Module::from_file(&self.engine, path)
            .map_err(|e| PluginError::Wasm(e.to_string()))?;
        Ok(module)
    }

    pub fn engine(&self) -> &wasmtime::Engine {
        &self.engine
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
