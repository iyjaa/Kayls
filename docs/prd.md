# PRD: Tscode

**Terminal Studio Code — A terminal-based IDE with a VS Code-like experience**

| Field | Value |
|-------|-------|
| Project Name | Tscode |
| Tagline | Terminal Studio Code |
| Document Version | 1.0 |
| Date | 2026-10-13 |
| Status | Active |

---

## 1. Background

### 1.1 Problem
- Existing terminal editors (Vim, Nano, Helix) have a steep learning curve or feel unfamiliar to VS Code users.
- VS Code is heavy, requires a GUI, and cannot run in a pure terminal environment (SSH, Termux, server).
- There is no terminal editor that **feels** like VS Code — with a file tree, tabs, command palette, and integrated terminal.

### 1.2 Vision
> Tscode is a terminal-based IDE that delivers a VS Code-like experience in a single lightweight Rust binary that runs anywhere.

---

## 2. Goals

### 2.1 Primary Goals (v1.0)
- A terminal editor that works out of the box with no complex configuration
- Keybindings & layout familiar to VS Code users
- Lightweight & fast on Termux and desktop
- A stable foundation for open-source contributions after v1.0

### 2.2 Non-Goals (v1.0)
-  Not a full VS Code replacement
- Not a modal editor (not a Vim clone)
-  Not a GUI
-  Not an editor for all languages (Rust first)

---

## 3. Target Users

### 3.1 Primary Persona — Mobile Developer
- Codes on Android via Termux
- Needs a comfortable editor on a small screen
- Pain point: Vim is hard, VS Code can't run

### 3.2 Secondary Persona — Remote Developer
- Works via SSH to a server
- Needs a lightweight IDE without a GUI

### 3.3 Anti-Persona
- Users who need a visual debugger, extension marketplace, or full GUI → use VS Code

---

## 4. Features

### 4.1 v1.0 Features (Required)

| ID | Feature | Priority |
|----|---------|----------|
| F-01 | Basic editor (open, edit, save) | P0 |
| F-02 | File tree | P0 |
| F-03 | Multiple file tabs | P0 |
| F-04 | Syntax highlighting (Rust first) | P0 |
| F-05 | Status bar | P0 |
| F-06 | Command palette | P1 |
| F-07 | Integrated terminal | P1 |
| F-08 | Configuration (config.toml) | P1 |
| F-09 | Themes (dark, light) | P1 |
| F-10 | Search in file | P2 |
| F-11 | Search across files | P2 |
| F-12 | Basic LSP | P2 |

### 4.2 Post-v1.0
- Multi-cursor, split editor, Git integration, WASM plugins, remote development, debugger

---

## 5. Keybindings (VS Code Familiar)

| Action | Keybinding |
|--------|------------|
| Command palette | Ctrl+Shift+P |
| Open file | Ctrl+O |
| Save | Ctrl+S |
| Close tab | Ctrl+W |
| Switch tab | Ctrl+Tab |
| Toggle file tree | Ctrl+B |
| Toggle terminal | Ctrl+` |
| Search in file | Ctrl+F |
| Undo / Redo | Ctrl+Z / Ctrl+Y |
| Quit | Ctrl+Q |

---

## 6. Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2024 |
| TUI rendering | ratatui |
| Input | crossterm |
| Text buffer | ropey |
| Syntax | tree-sitter |
| LSP | tower-lsp |
| Integrated terminal | portable-pty + vte |
| Config | serde + toml |
| CLI | clap |
| Error | anyhow + thiserror |

---

## 7. Architecture

### 7.1 Workspace Crates

| Crate | Responsibility |
|-------|----------------|
| tscode-core | Text buffer, cursor, editing |
| tscode-syntax | Syntax highlighting (tree-sitter) |
| tscode-terminal | PTY, shell integration |
| tscode-tui | Ratatui rendering |
| tscode | Main binary |

### 7.2 Data Flow
Input (crossterm) → Keybind → Command → App State → UI (ratatui) → Terminal

---

## 8. Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Scope creep | Stick to v1.0 non-goals |
| Rust learning curve | Learn by building |
| Performance on Termux | Test at every milestone |
| LSP complexity | LSP optional, post-v1.0 |
| Loss of motivation | Dogfooding, celebrate milestones |

---

## 9. Success Metrics

- Startup < 100ms
- Input latency < 16ms
- Binary < 20 MB
- Dogfooding for 1 month without switching editors
- Unit tests for buffer & cursor

---

## 10. Principles

> **v1.0 first, public documentation later.**
> Focus on solo development. Open for contributions once the foundation is stable.