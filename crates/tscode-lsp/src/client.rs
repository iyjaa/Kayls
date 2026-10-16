use crate::config::ServerConfig;
use crate::error::{LspError, Result};
use crate::protocol::{encode_message, JsonRpcRequest};
use crate::server::ServerHandle;
use serde_json::json;
use std::path::PathBuf;

pub struct LspClient {
    config: ServerConfig,
    root: PathBuf,
    server: Option<ServerHandle>,
    next_id: u64,
    initialized: bool,
}

impl LspClient {
    pub fn new(config: ServerConfig, root: PathBuf) -> Self {
        Self {
            config,
            root,
            server: None,
            next_id: 1,
            initialized: false,
        }
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.server.is_some() {
            return Err(LspError::Other("Server already running".to_string()));
        }

        let handle =
            ServerHandle::spawn(&self.config.command, &self.config.args, self.root.clone()).await?;

        self.server = Some(handle);
        Ok(())
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let server = self.server.as_mut().ok_or(LspError::ServerNotRunning)?;

        let id = self.next_id;
        self.next_id += 1;

        let root_uri = format!("file://{}", self.root.display());

        let params = json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "synchronization": {
                        "dynamicRegistration": false,
                        "willSave": false,
                        "willSaveWaitUntil": false,
                        "didSave": true
                    },
                    "completion": {
                        "dynamicRegistration": false,
                        "completionItem": {
                            "snippetSupport": false
                        }
                    },
                    "hover": {
                        "dynamicRegistration": false
                    },
                    "publishDiagnostics": {
                        "relatedInformation": false
                    }
                }
            },
            "workspaceFolders": [
                {
                    "uri": root_uri,
                    "name": self.root.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("workspace")
                }
            ]
        });

        let request = JsonRpcRequest::new(id, "initialize", Some(params));
        let msg = encode_message(&request)?;

        server.send_message(&msg).await?;

        Ok(())
    }
    pub async fn read_response(&mut self) -> Result<Option<String>> {
        let server = self.server.as_mut().ok_or(LspError::ServerNotRunning)?;

        server.read_message().await
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut server) = self.server.take() {
            server.kill().await?;
        }
        self.initialized = false;
        Ok(())
    }
}
