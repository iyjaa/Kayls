use crate::error::{LspError, Result};
use crate::protocol::parse_content_length;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

pub struct ServerHandle {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<BufReader<ChildStdout>>,
    pub root: PathBuf,
}

impl ServerHandle {
    pub async fn spawn(command: &str, args: &[String], root: PathBuf) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .current_dir(&root)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take().map(BufReader::new);

        if stdin.is_none() || stdout.is_none() {
            return Err(LspError::Protocol(
                "Failed to capture stdin/stdout".to_string(),
            ));
        }

        Ok(Self {
            child,
            stdin,
            stdout,
            root,
        })
    }

    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        let stdin = self.stdin.as_mut().ok_or(LspError::ServerNotRunning)?;

        stdin.write_all(message.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }

    /// Read LSP message from server.
    ///
    /// LSP framing:
    ///   Content-Length: <n>\r\n
    ///   \r\n
    ///   <n bytes of JSON body>
    pub async fn read_message(&mut self) -> Result<Option<String>> {
        let stdout = self.stdout.as_mut().ok_or(LspError::ServerNotRunning)?;

        // 1. Read headers until empty line
        let mut content_length: Option<usize> = None;
        let mut line = String::new();

        loop {
            line.clear();
            let n = stdout.read_line(&mut line).await?;
            if n == 0 {
                return Ok(None); // EOF
            }

            let trimmed = line.trim_end_matches(['\r', '\n']);

            if trimmed.is_empty() {
                // End of headers
                break;
            }

            if let Some(len) = parse_content_length(trimmed) {
                content_length = Some(len);
            }
            // Ignore other headers (Content-Type)
        }

        let len = content_length
            .ok_or_else(|| LspError::Protocol("Missing Content-Length".to_string()))?;

        // 2. Read exactly `len` bytes
        let mut body = vec![0u8; len];
        stdout.read_exact(&mut body).await?;

        let body = String::from_utf8(body)
            .map_err(|e| LspError::Protocol(format!("Invalid UTF-8: {}", e)))?;

        Ok(Some(body))
    }

    pub async fn kill(&mut self) -> Result<()> {
        self.stdin.take();
        self.stdout.take();

        let _ = self.child.kill().await;
        let _ = self.child.wait().await;

        Ok(())
    }
}
