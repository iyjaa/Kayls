use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl DiagnosticSeverity {
    pub fn color(&self) -> &'static str {
        match self {
            DiagnosticSeverity::Error => "#3a1a1a",
            DiagnosticSeverity::Warning => "#3a3a1a",
            DiagnosticSeverity::Information => "#1a2a3a",
            DiagnosticSeverity::Hint => "#2a2a2a",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default)]
    pub version: Option<i64>,
}

impl Diagnostic {
    /// Get severity — default to Error if not set
    pub fn severity_or_default(&self) -> DiagnosticSeverity {
        self.severity.unwrap_or(DiagnosticSeverity::Error)
    }

    pub fn color(&self) -> &'static str {
        self.severity_or_default().color()
    }
}
