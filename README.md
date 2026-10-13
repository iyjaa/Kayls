# Tscode

Terminal Studio Code — A terminal-based IDE with a VS Code-like experience.

## Status

**Tsc-v0.1.0** — Workspace setup. Unstable, in development.

See [versioning.md](docs/versioning.md) for the versioning scheme.

## Build

  ```
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
- [Versioning](docs/versioning.md) — Version scheme
- [Roadmap](docs/roadmap.md) — Actual feature progress
- [Dev Notes](docs/dev-notes.md) — Development notes

> **Note**: Examples in [versioning.md](docs/versioning.md) (like X=1 Terminal, X=2 LSP, X=3 AI Agent) are **illustrations only** to explain the versioning scheme.
> For the **actual** feature progress, see [roadmap.md](docs/roadmap.md).

## License

MIT