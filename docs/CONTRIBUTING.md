# Contributing to Kayls

Thanks for your interest in contributing to Kayls! This document explains how to contribute.

## Versioning

**Important**: Kayls uses a custom versioning scheme. In `Kayls-v` versions, **no new features** are allowed — only bug fixes. New features start after `Kayls-s`. See [docs/VERSIONING.md](docs/VERSIONING.md) for details.

## Workflow

Kayls follows a **discuss-first** workflow. Before writing code:
1. Open an issue describing what you want to change.
2. Wait for maintainer approval.
3. Fork the repo and create a branch (`feature/your-feature` or `fix/your-bug`).
4. Submit a Pull Request referencing the issue.

## Project Structure

Kayls is a Rust workspace with the following crates:
- `kayls-core` — Core logic (FileTree, Document, Config)
- `kayls-tui` — TUI rendering and app state
- `kayls-editor` — Forked editor widget
- `kayls` — Main binary and CLI

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy` and fix warnings.
- Write tests for new features.

## Reporting Issues

- Check if the issue already exists in [GitHub Issues](https://github.com/iyjaa/Kayls/issues).
- Include as much detail as possible:
  - **OS & Terminal** — e.g., Ubuntu 24.04, Termux, Alacritty.
  - **Kayls version** — output of `kayls --version`.
  - **Steps to reproduce**.
  - **Expected vs actual behavior**.

## License

By contributing to Kayls, you agree that your contributions will be licensed under the [MIT License](LICENSE).

Every contribution matters — code, docs, bug reports, or feedback. Thank you for helping make Kayls better.

## Branching Strategy

The repository is divided into three main branches. Do not open PRs directly against main unless your change spans multiple areas.

| Branch | Scope | Key Directories |
|---|---|---|
| `feat/editor-core` | Buffer logic, Rope, tree-sitter, cursor, selection. | `crates/kayls-editor`, `crates/kayls-core` |
| `feat/ui-integration` | Ratatui widgets, layout, rendering, input routing. | `crates/kayls-tui` |
| `docs/improvement` | Documentation, branding, tutorials. | `docs/`, `*.md`, `assets/` |

## Integration Contract

The UI and Editor teams communicate through the API contract in `crates/kayls-editor/FEATURES.md`.

1. **Editor Team:** Update `FEATURES.md` when adding or changing public methods.
2. **UI Team:** Read `FEATURES.md` to discover new capabilities. Never access private fields directly.
3. **Breaking Changes:** Notify the UI team via GitHub issue if a method signature changes.
