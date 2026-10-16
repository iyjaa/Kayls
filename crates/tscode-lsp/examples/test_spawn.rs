use tscode_lsp::{LspClient, ServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServerConfig::rust_analyzer();
    let root = std::env::current_dir()?;

    println!("[TEST] Starting {} in {}", config.name, root.display());

    let mut client = LspClient::new(config, root.clone());

    println!("[TEST] Spawning server...");
    client.start().await?;
    println!("[TEST] Server spawned");

    println!("[TEST] Sending initialize request...");
    client.initialize().await?;
    println!("[TEST] Initialize sent");

    println!("[TEST] Waiting for response (max 30s)...");

    let result = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        client.read_response(),
    )
    .await;

    match result {
        Ok(Ok(Some(msg))) => {
            println!("[TEST] Got response ({} bytes):", msg.len());
            // Print first 500 chars
            let preview = if msg.len() > 500 { &msg[..500] } else { &msg };
            println!("{}", preview);
        }
        Ok(Ok(None)) => {
            println!("[TEST] Server closed connection (EOF)");
        }
        Ok(Err(e)) => {
            println!("[TEST] Error: {}", e);
        }
        Err(_) => {
            println!("[TEST] Timeout waiting for response");
        }
    }

    println!("[TEST] Stopping server...");
    client.stop().await?;
    println!("[TEST] Done");

    Ok(())
}