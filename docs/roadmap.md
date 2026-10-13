# Tscode Roadmap

From v0.1.0 to v1.0.0.

---

## Current Status

**v0.1.0** — Workspace setup, hello Tscode

---

## Milestones

| Version | Focus | Target | Status |
|---------|-------|--------|--------|
| v0.1.0 | Workspace setup | - | Done |
| v0.2.0 | Basic editor (open, edit, save) | Month 1 | In progress |
| v0.3.0 | File tree + tabs | Month 2 | Planned |
| v0.4.0 | Syntax highlighting | Month 3 | Planned |
| v0.5.0 | Command palette | Month 4 | Planned |
| v0.6.0 | Integrated terminal | Month 5 | Planned |
| v0.7.0 | Configuration + themes | Month 6 | Planned |
| v0.8.0 | Search in & across files | Month 7 | Planned |
| v0.9.0 | Basic LSP | Month 8 | Planned |
| v0.10.0 | Polish & dogfooding | Month 9 | Planned |
| v1.0.0 | Stable release | Month 10 | Planned |

---

## Details per Version

### v0.2.0 — Basic Editor

**Target**: Can open, edit, and save files

**Phases**:
- [ ] **v0.2.0-a**: Static TUI layout (tab bar, file tree, editor, status bar)
- [ ] **v0.2.0-b**: Open file via argument (`tscode file.rs`)
- [ ] **v0.2.0-c**: Cursor movement & scroll
- [ ] **v0.2.0-d**: Edit & save (`Ctrl+S`)

**Crates touched**: `tscode-tui`, `tscode-core`, `tscode`

---

### v0.3.0 — File Tree + Tabs

**Target**: Navigate folders & multiple open files

**Phases**:
- [ ] File tree in sidebar (arrow key navigation)
- [ ] Open file with Enter
- [ ] Tab bar for open files
- [ ] Switch tabs with `Ctrl+Tab`
- [ ] Close tab with `Ctrl+W`

**Crates**: `tscode-tui`, `tscode-core`

---

### v0.4.0 — Syntax Highlighting

**Target**: Code coloring with tree-sitter

**Phases**:
- [ ] Integrate `tree-sitter` in `tscode-syntax`
- [ ] Highlighting for Rust
- [ ] Highlighting for other languages (Python, JS)
- [ ] Color themes

**Crates**: `tscode-syntax`, `tscode-tui`

---

### v0.5.0 — Command Palette

**Target**: `Ctrl+Shift+P` to search commands

**Phases**:
- [ ] Command registry
- [ ] Fuzzy search popup
- [ ] Basic commands: save, open, quit, toggle sidebar

**Crates**: `tscode-tui`, `tscode-core`

---

### v0.6.0 — Integrated Terminal

**Target**: Terminal panel below the editor

**Phases**:
- [ ] `portable-pty` to spawn shell
- [ ] `vte` parser for escape sequences
- [ ] Render terminal output
- [ ] Toggle with `Ctrl+``

**Crates**: `tscode-terminal`, `tscode-tui`

---

### v0.7.0 — Configuration + Themes

**Target**: `config.toml` file for settings

**Phases**:
- [ ] Config schema with `serde`
- [ ] Read `~/.config/tscode/config.toml`
- [ ] Custom keybindings
- [ ] Theme selection

**Crates**: `tscode-core`, `tscode`

---

### v0.8.0 — Search

**Target**: Search in files & across files

**Phases**:
- [ ] `Ctrl+F` — search in file
- [ ] `Ctrl+Shift+F` — search across files
- [ ] Highlight matches
- [ ] Next/prev navigation

**Crates**: `tscode-core`, `tscode-tui`

---

### v0.9.0 — Basic LSP

**Target**: Autocomplete & diagnostics

**Phases**:
- [ ] LSP client with `tower-lsp`
- [ ] Connect to `rust-analyzer`
- [ ] Autocomplete popup
- [ ] Diagnostics (error underline)

**Crates**: `tscode-core`, `tscode-tui`

---

### v0.10.0 — Polish

**Target**: Ready for v1.0

**Phases**:
- [ ] Bug fixes from dogfooding
- [ ] Performance tuning
- [ ] Unit tests for buffer & cursor
- [ ] Complete internal documentation

---

### v1.0.0 — Stable Release

**Target**: Ready to use & open for contributions

**Criteria**:
- [ ] All P0 & P1 features complete
- [ ] Dogfooding for 1 month without switching editors
- [ ] Complete README, CONTRIBUTING, ARCHITECTURE
- [ ] CI (automated build & test)
- [ ] Open for public contributions

---

## Versioning Rules (SemVer)

- **v0.x.0** — new features / breaking changes
- **v0.x.y** — bug fixes
- After v1.0.0:
  - **vX.0.0** — breaking changes
  - **vX.Y.0** — new features
  - **vX.Y.Z** — bug fixes

---

## Principles

> Make it work, make it right, make it fast.
> — Kent Beck

1. **Work** — v0.1.0 to v0.5.0: features first
2. **Right** — v0.6.0 to v0.9.0: clean architecture
3. **Fast** — v0.10.0 to v1.0.0: optimization & polish