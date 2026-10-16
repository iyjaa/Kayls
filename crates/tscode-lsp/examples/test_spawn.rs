use tscode_lsp::{LspClient, ServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = std::path::PathBuf::from("/tmp/lsp-test");

    println!("[TEST] Starting rust-analyzer in {}", root.display());

    let config = ServerConfig::rust_analyzer();
    let mut client = LspClient::new(config, root.clone());

    println!("[TEST] Spawning server...");
    client.start().await?;
    println!("[TEST] Server spawned");

    println!("[TEST] Sending initialize...");
    client.initialize().await?;
    println!("[TEST] Waiting for initialize response...");

    let init_response =
        tokio::time::timeout(std::time::Duration::from_secs(30), client.read_message()).await;

    match init_response {
        Ok(Ok(Some(_))) => {
            println!("[TEST] Got initialize response");
        }
        _ => {
            println!("[TEST] Failed to get initialize response");
            client.stop().await?;
            return Ok(());
        }
    }

    println!("[TEST] Sending initialized notification...");
    client.send_initialized().await?;

    let test_file = root.join("src/main.rs");
    println!("[TEST] Opening file: {}", test_file.display());
    client.did_open(&test_file, "rust").await?;

    println!("[TEST] Waiting for initial analysis (5s)...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    println!("[TEST] Sending didChange...");
    client.did_change(&test_file).await?;

    println!("[TEST] Waiting for diagnostics (max 120s)...");
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(120);

    loop {
        if tokio::time::Instant::now() >= deadline {
            println!("[TEST] Timeout");
            break;
        }

        match client.read_message().await {
            Ok(Some(v)) => {
                if v.get("method").and_then(|m| m.as_str())
                    == Some("textDocument/publishDiagnostics")
                {
                    client.handle_message(&v);
                    let count: usize = client
                        .diagnostics()
                        .values()
                        .map(|d| d.diagnostics.len())
                        .sum();
                    println!("[TEST] publishDiagnostics — total: {}", count);
                    if count > 0 {
                        println!("[TEST] Got real diagnostics!");
                        break;
                    }
                } else {
                    client.handle_message(&v);
                }
            }
            Ok(None) => {
                println!("[TEST] Server closed");
                break;
            }
            Err(e) => {
                println!("[TEST] Error: {}", e);
                break;
            }
        }
    }

    println!("\n[TEST] Diagnostics stored: {}", client.diagnostics().len());
    for (uri, diags) in client.diagnostics() {
        println!("  {} — {} diagnostics", uri, diags.diagnostics.len());
        for d in &diags.diagnostics {
            println!(
                "    [{:?}] {} ({}:{})",
                d.severity_or_default(),
                d.message,
                d.range.start.line + 1,
                d.range.start.character + 1
            );
        }
    }

    println!("\n[TEST] Stopping server...");
    client.stop().await?;
    println!("[TEST] Done");

    Ok(())
}