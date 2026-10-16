# PRD: Tscode

**Terminal Studio Code — A terminal-based IDE inspired by VS Code**

| Field | Value |
|-------|-------|
| Project Name | Tscode |
| Tagline | Terminal Studio Code |
| Document Version | 2.1 |
| Date | 2026-10-16 |
| Status | Active |
| License | MIT |

---

## 1. Background

### 1.1 Problem

- Existing terminal editors (Vim, Nano, Helix) have a steep learning curve or feel unfamiliar to VS Code users.
- VS Code is heavy, requires a GUI, and cannot run in a pure terminal environment (SSH, Termux, server).
- There is no terminal editor that combines VS Code's layout with terminal efficiency.

### 1.2 Vision

> Tscode is a terminal-based IDE inspired by VS Code's layout and workflow. It supports two keybinding modes: VS Code (Ctrl+key) and Vim (Esc then key). The default mode adapts to the terminal environment.

---

## 2. Goals

### 2.1 Primary Goals (v1.0)

- A terminal editor that works out of the box
- Layout familiar to VS Code users (tab bar, file tree, editor, status bar)
- Two keybinding modes: VS Code and Vim
- Lightweight & fast on Termux and desktop
- A stable foundation for open-source contributions

### 2.2 Non-Goals (v1.0)

- Not a full VS Code replacement
- Not a modal-only editor (Vim mode is optional)
- Not a GUI
- Not an editor for all languages (Rust first)

---

## 3. Target Users

### 3.1 Primary Persona — Mobile Developer

- Codes on Android via Termux or Acode terminal
- Needs a comfortable editor on a small screen
- Pain point: Vim is hard, VS Code can't run

### 3.2 Secondary Persona — Remote Developer

- Works via SSH to a server
- Needs a lightweight IDE without a GUI

### 3.3 Anti-Persona

- Users who need a visual debugger, extension marketplace, or full GUI → use VS Code

---

## 4. Features

### 4.1 v1.x Features

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| F-01 | Open, edit, save file | P0 | Done |
| F-02 | File tree (recursive) | P0 | Done |
| F-03 | Syntax highlighting | P0 | Done |
| F-04 | Cursor movement | P0 | Done |
| F-05 | Line numbers | P0 | Done |
| F-06 | Focus system (editor/explorer) | P0 | Done |
| F-07 | Mode system (Normal/Command/Prompt) | P0 | Done |
| F-08 | Save with hint | P0 | Done |
| F-09 | Expand/collapse folder | P0 | Done |
| F-10 | Auto-pair brackets & quotes | P0 | Done |
| F-11 | Auto-indent | P0 | Done |
| F-12 | Themes (Vesper, Gruvbox, Dracula, Nord) | P0 | Done |
| F-13 | File/folder management | P0 | Done |
| F-14 | Explorer vertical & horizontal scroll | P0 | Done |
| F-15 | Config file (`config.toml`) | P0 | Done |
| F-16 | Theme persistence | P0 | Done |
| F-17 | Snippets (user custom) | P1 | Planned |
| F-18 | Multi-tabs | P1 | Planned |
| F-19 | Search in file | P1 | Planned |
| F-20 | Go to line | P2 | Planned |
| F-21 | VS Code keybinding mode | P2 | Planned |

### 4.2 Post-v1.x Features

- Multi-cursor
- Split editor
- Git integration
- Plugin system

---

## 5. Keybindings

Tscode supports two keybinding modes. **Currently only Vim mode is implemented.** VS Code mode is planned.

### 5.1 VS Code Mode (Planned, not yet implemented)

| Action | Keybinding |
|--------|------------|
| Save | Ctrl+S |
| Open file | Ctrl+O |
| Quit | Ctrl+Q |
| Toggle explorer | Ctrl+B |
| Find | Ctrl+F |
| Undo | Ctrl+Z |
| Redo | Ctrl+Y |
| Copy | Ctrl+C |
| Paste | Ctrl+V |
| Cut | Ctrl+X |
| Select all | Ctrl+A |
| Duplicate line | Ctrl+D |
| Delete line | Ctrl+K |

**Note**: Some `Ctrl+key` combinations may not work on Android terminals (especially Acode). Testing required.

### 5.2 Vim Mode (Implemented)

| Action | Keybinding |
|--------|------------|
| Move cursor | Arrow keys |
| Insert character | Any character key |
| Delete character | Backspace |
| New line | Enter |
| Indent | Tab |
| Undo | Ctrl+Z |
| Redo | Ctrl+Y |
| Copy | Ctrl+C |
| Paste | Ctrl+V |
| Cut | Ctrl+X |
| Select all | Ctrl+A |
| Duplicate line | Ctrl+D |
| Delete line | Ctrl+K |
| Enter command mode | Esc |
| Open file | Esc then o |
| Save file | Esc then w |
| Toggle explorer | Esc then e |
| New file | Esc then n |
| New folder | Esc then v |
| Delete | Esc then d |
| Select theme | Esc then t |
| Cancel command | Esc then Esc |
| Quit | Alt+Q |

### 5.3 Mode Selection

- Default mode: Vim (currently)
- User can override via `config.toml` (planned, Tsc-v1.1.8)

    [keybindings]
    mode = "vscode"   # or "vim"

---

## 6. Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2021 |
| TUI framework | ratatui 0.30 |
| Input | crossterm 0.29 |
| Editor engine | ratatui-code-editor 0.0.6 (fork) |
| Syntax | tree-sitter (via ratatui-code-editor) |
| CLI | clap 4 |
| Config | serde 1, toml 0.8 |
| Error | anyhow 1, thiserror 2 |

---

## 7. Architecture

### 7.1 Workspace Crates

| Crate | Responsibility |
|-------|----------------|
| tscode-core | FileTree, Document, Config |
| tscode-tui | App state, UI rendering |
| tscode | Main binary, CLI parsing |

### 7.2 Data Flow

    Input (crossterm)
        ↓
    Keybind handler (App)
        ↓
    Mode handler (Normal / Command / Prompt)
        ↓
    Focus handler (Editor / Explorer)
        ↓
    Action (edit, navigate tree, save, quit)
        ↓
    State update
        ↓
    Render (ui.rs)
        ↓
    Terminal

### 7.3 Key Modules

| Module | Location | Responsibility |
|--------|----------|----------------|
| `filetree` | `tscode-core` | Recursive file tree |
| `document` | `tscode-core` | File path, content (legacy cursor) |
| `cursor` | `tscode-core` | Cursor position (legacy) |
| `config` | `tscode-core` | Config file load/save |
| `app` | `tscode-tui` | App state, key handling |
| `ui` | `tscode-tui` | TUI rendering |
| `lib` | `tscode-tui` | Main loop |

---

## 8. Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Scope creep | Stick to versioning golden rule |
| Editor library limits | Accept trade-off for speed |
| Performance on Termux | Test at every milestone |
| Loss of motivation | Open-source, community support |

---

## 9. Success Metrics

- Startup < 100ms
- Input latency < 16ms
- Binary < 20 MB
- Dogfooding for 1 week without switching editors
- At least 1 external contributor

---

## 10. Principles

> **v1.0 first, public documentation later.**
> Focus on solo development. Open for contributions once the foundation is stable.

> **Make it work, make it right, make it fast.**
> — Kent Beck

1. Work — v0.1.0 to v1.0.0: features work first
2. Right — v1.2.0 onward: solid foundation
3. Fast — v2.0.0 onward: optimization

---

## 11. Versioning

Tscode uses a custom versioning scheme:

    Tsc-vX.Y.Z   ->  Unstable
    Tsc-sX.Y.Z   ->  Stable

| Position | Meaning |
|----------|---------|
| X | Major feature (planned) |
| Y | Sub-feature (discovered) |
| Z | Bug fix count |

**Golden rule**: In `Tsc-v`, no new features. Only bug fixes. New features start after `Tsc-s`.

See [versioning.md](versioning.md) for full details.
See [roadmap.md](roadmap.md) for actual feature progress.

---

## 12. Contributing

Tscode is open for contributions. The workflow:

1. **Discuss first** — open an issue to discuss the feature/bug before coding
2. **Get approval** — wait for maintainer approval
3. **Fork & branch** — create a branch for your change
4. **Submit PR** — reference the issue in your PR
5. **Review** — maintainer reviews and merges

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

**Areas that need help**:

- Snippets (user custom) — F-17
- Multi-tabs — F-18
- Search in file — F-19
- Go to line — F-20
- VS Code keybinding mode — F-21
- Documentation improvements
- Bug fixes

---

## 13. Current Status

- **Version**: Tsc-v1.1.3 (unstable)
- **Phase**: Config file + horizontal scroll
- **Crates**: 3 (tscode, tscode-core, tscode-tui)
- **Next milestone**: Tsc-s1.1.3 (after dogfooding)
- **Open for contributions**: Yes