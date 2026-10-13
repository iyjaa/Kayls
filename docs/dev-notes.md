# Dev Notes

Personal notes during Tscode development. Not public documentation.

---

## 2026-10-13 — Initial Setup

### Done
- Set up Rust workspace with 5 crates
- Root `Cargo.toml` with `[workspace]`
- Crates: tscode-core, tscode-syntax, tscode-terminal, tscode-tui, tscode
- Git init & push to GitHub (https://github.com/iyjaa/Tscode)
- `cargo build --workspace` succeeded
- `cargo run -p tscode` → `Tscode v0.1.0`

### Decisions
- **Workspace from the start** — to scale up to v1.0
- **Version 0.1.0** in root `[workspace.package]`, all crates use `version.workspace = true`
- **Non-modal** — VS Code keybindings, not Vim
- **Rust first** for syntax highlighting

### Notes
- `cargo new` creates `.git` & `.gitignore` inside each crate → must be removed
- Use `--vcs none` for new crates going forward
- First build is slow (~6 min), subsequent builds are fast

### Next TODO
- [ ] Clean up `Cargo.toml` (fix email, add description)
- [ ] `main.rs` with clap (`--version`, `--help`)
- [ ] v0.2.0: TUI layout (tab bar, file tree, editor, status bar)
- [ ] v0.2.0: Open file & display its contents

---

## Template for New Entries

```markdown
## YYYY-MM-DD — Title

### Done
- 

### Decisions
- 

### Issues & Solutions
- 

### Next TODO
- [ ] 