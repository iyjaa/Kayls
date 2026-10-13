# Tscode

Terminal Studio Code — A terminal-based IDE with a VS Code-like experience.

## Status

🚧 **v0.1.0** — Workspace setup. Actively in development.

## Build

```bash
cargo build --workspace
cargo run -p tscode
```

## Structure

Rust workspace with 5 crates:

| Crate | Purpose |
|-------|---------|
| `tscode-core` | Text buffer, cursor, editing |
| `tscode-syntax` | Syntax highlighting |
| `tscode-terminal` | PTY, shell integration |
| `tscode-tui` | Ratatui rendering |
| `tscode` | Main binary |

## Documentation

- [PRD](docs/prd.md) — Product Requirements Document
- [Roadmap](docs/roadmap.md) — Plan from v0.1.0 to v1.0.0
- [Dev Notes](docs/dev-notes.md) — Development notes

## License

MIT