use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    FileOpen { path: PathBuf },
    FileSave { path: PathBuf },
    BufferChange { path: PathBuf, line: u32 },
    KeyPress { key: String },
    CommandRun { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginAction {
    InsertText { text: String },
    ShowMessage { text: String },
    RunCommand { name: String },
    NoOp,
}
