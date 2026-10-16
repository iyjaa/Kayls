use crate::config::ServerConfig;
use crate::diagnostics::PublishDiagnosticsParams;
use crate::error::{LspError, Result};
use crate::protocol::{encode_message, JsonRpcNotification, JsonRpcRequest};
use crate::server::ServerHandle;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct LspClient {
    config: ServerConfig,
    root: PathBuf,
    server: Option<ServerHandle>,
    next_id: u64,
    initialized: bool,
    /// URI -> diagnostics
    diagnostics: HashMap<String, PublishDiagnosticsParams>,
}

impl LspClient {
    pub fn new(config: ServerConfig, root: PathBuf) -> Self {
        Self {
            config,
            root,
            server: None,
            next_id: 1,
            initialized: false,
            diagnostics: HashMap::new(),
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

    pub fn diagnostics(&self) -> &HashMap<String, PublishDiagnosticsParams> {
        &self.diagnostics
    }

    pub fn diagnostics_for(&self, uri: &str) -> Option<&PublishDiagnosticsParams> {
        self.diagnostics.get(uri)
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
        let id = self.next_id;
        self.next_id += 1;

        let root_uri = format!("file://{}", self.root.display());
        eprintln!("[DEBUG] rootUri = {}", root_uri);

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

        self.send(&msg).await?;
        Ok(())
    }

    pub async fn send_initialized(&mut self) -> Result<()> {
        let notification = JsonRpcNotification::new("initialized", Some(json!({})));
        let msg = encode_message(&notification)?;
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn did_open(&mut self, path: &std::path::Path, language_id: &str) -> Result<()> {
        let uri = format!("file://{}", path.display());
        let text = std::fs::read_to_string(path)?;

        let params = json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": 1,
                "text": text
            }
        });
        let notification = JsonRpcNotification::new("textDocument/didOpen", Some(params));
        let msg = encode_message(&notification)?;
        self.send(&msg).await?;
        Ok(())
    }
    pub async fn did_change(&mut self, path: &std::path::Path) -> Result<()> {
        let uri = format!("file://{}", path.display());
        let text = std::fs::read_to_string(path)?;

        let params = json!({
            "textDocument": {
                "uri": uri,
                "version": 2
            },
            "contentChanges": [
                {
                    "text": text
                }
            ]
        });

        let notification = JsonRpcNotification::new("textDocument/didChange", Some(params));
        let msg = encode_message(&notification)?;
        self.send(&msg).await?;
        Ok(())
    }
    /// Read one message from server. Returns the parsed JSON.
    pub async fn read_message(&mut self) -> Result<Option<Value>> {
        let server = self.server.as_mut().ok_or(LspError::ServerNotRunning)?;

        let raw = match server.read_message().await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let value: Value = serde_json::from_str(&raw)?;
        Ok(Some(value))
    }

    /// Handle a message — process notifications, return request responses.
    pub fn handle_message(&mut self, value: &Value) {
        // Check if it's a notification (no id)
        if value.get("id").is_none() {
            if let Some(method) = value.get("method").and_then(|m| m.as_str()) {
                match method {
                    "textDocument/publishDiagnostics" => {
                        if let Some(params) = value.get("params") {
                            if let Ok(parsed) =
                                serde_json::from_value::<PublishDiagnosticsParams>(params.clone())
                            {
                                self.diagnostics.insert(parsed.uri.clone(), parsed);
                            }
                        }
                    }
                    "window/logMessage" | "$/progress" => {
                        // Ignore
                    }
                    _ => {
                        // Unknown notification
                    }
                }
            }
        }
        // Requests with id — responses handled by caller
    }

    async fn send(&mut self, msg: &str) -> Result<()> {
        let server = self.server.as_mut().ok_or(LspError::ServerNotRunning)?;
        server.send_message(msg).await
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut server) = self.server.take() {
            server.kill().await?;
        }
        self.initialized = false;
        Ok(())
    }
}
