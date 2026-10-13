# Tscode

Terminal Studio Code — IDE berbasis terminal.

## Status

v0.1.0 — dalam pengembangan.

## Build

```bash
cargo build --release
cargo run -p tscode
```

## Struktur

- `crates/tscode-core` — text buffer, cursor, editing
- `crates/tscode-syntax` — syntax highlighting
- `crates/tscode-terminal` — PTY, shell
- `crates/tscode-tui` — rendering ratatui
- `crates/tscode` — binary utama