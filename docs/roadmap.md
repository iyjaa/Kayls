# Tscode Roadmap

From Tsc-v0.1.0 onward.

> **This is the source of truth for feature progress.**
> Examples in other documents (like versioning.md) are illustrations only.

See [versioning.md](versioning.md) for the versioning scheme.

---

## Current Status

**Tsc-v1.1.2** — Explorer scroll, soft theme-aware colors, auto-pair, auto-indent, themes, file/folder management.

---

## Milestones

| Version | Status | Focus |
|---------|--------|-------|
| Tsc-v0.1.0 | Done | Workspace setup |
| Tsc-s0.1.0 | Done | Workspace setup verified |
| Tsc-v0.2.0 | Done | Static TUI layout |
| Tsc-s0.2.0 | Done | Layout verified |
| Tsc-v0.2.1 | Done | Open file via argument |
| Tsc-s0.2.1 | Done | Open file verified |
| Tsc-v0.2.2 | Done | Cursor movement |
| Tsc-s0.2.2 | Done | Cursor verified |
| Tsc-v0.2.3 | Done | Line numbers |
| Tsc-s0.2.3 | Done | Line numbers verified |
| Tsc-v0.2.4 | Done | Insert, delete, save, mode, hint |
| Tsc-s0.2.4 | Done | Basic editor verified |
| Tsc-v0.3.0 | Done | Read folder (FileTree) |
| Tsc-s0.3.0 | Done | Read folder verified |
| Tsc-v0.3.1 | Done | Render file tree |
| Tsc-s0.3.1 | Done | Render file tree verified |
| Tsc-v0.3.2 | Done | Focus system + arrow navigation |
| Tsc-s0.3.2 | Done | Focus system verified |
| Tsc-v0.3.3 | Done | Open file with Enter |
| Tsc-s0.3.3 | Done | Open file verified |
| Tsc-v1.0.0 | Done | Editor integration (X=1) |
| Tsc-s1.0.0 | Done | Editor integration verified |
| Tsc-v1.0.1 | Done | Cross-platform binary release |
| Tsc-s1.0.1 | Done | Binary release verified |
| Tsc-v1.0.2 | Done | Fix line number visibility |
| Tsc-s1.0.2 | Done | Line number fix verified |
| Tsc-v1.1.0 | Done | Editor polish (auto-pair, auto-indent, themes, file mgmt) |
| Tsc-s1.1.0 | Done | Editor polish verified |
| Tsc-v1.1.1 | Done | Soft theme-aware colors |
| Tsc-s1.1.1 | Done | Soft palette verified |
| Tsc-v1.1.2 | Done | Explorer scroll, UI polish |
| Tsc-s1.1.2 | Planned | Explorer scroll verified |
| Tsc-v1.1.3 | Planned | Multi-tabs |
| Tsc-v1.1.4 | Planned | Search in file |
| Tsc-v1.1.5 | Planned | Go to line |
| Tsc-v1.1.6 | Planned | Config file + keybinding |
| Tsc-v2.0.0 | Planned | Integrated terminal (X=2) |
| Tsc-v3.0.0 | Planned | LSP (X=3) |
| Tsc-v4.0.0 | Planned | AI agent (X=4) |

---

## Versioning Rules

Tscode uses `Tsc-vX.Y.Z` (unstable) and `Tsc-sX.Y.Z` (stable).

| Position | Meaning |
|----------|---------|
| X | Major feature (planned) |
| Y | Sub-feature (discovered during development) |
| Z | Bug fix count |

**Golden rule**: In `Tsc-v`, no new features. Only bug fixes. New features start after `Tsc-s`.

See [versioning.md](versioning.md) for full details.

---

## Details per Version

### Tsc-v0.1.0 — Workspace Setup

**Status**: Done (stable: Tsc-s0.1.0)

**Contents**:
- Rust workspace with 5 crates
- Git & GitHub setup
- Basic `main.rs`
- Custom versioning scheme

**Crates touched**: all

---

### Tsc-v0.2.0 — Static TUI Layout

**Status**: Done (stable: Tsc-s0.2.0)

**Contents**:
- TUI layout with ratatui (tab bar, file tree, editor, status bar)
- `q` to quit

**Crates touched**: `tscode-tui`, `tscode`

---

### Tsc-v0.2.1 — Open File

**Status**: Done (stable: Tsc-s0.2.1)

**Contents**:
- `Document` struct in `tscode-core`
- Argument parsing with `clap`
- Open file & display contents
- Dynamic tab bar (file name)

**Crates touched**: `tscode-core`, `tscode-tui`, `tscode`

---

### Tsc-v0.2.2 — Cursor Movement

**Status**: Done (stable: Tsc-s0.2.2)

**Contents**:
- `Cursor` struct (line, col)
- Arrow keys to move cursor
- Cursor highlight in editor (REVERSED)
- Cursor position in status bar
- Vertical & horizontal scroll

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v0.2.3 — Line Numbers

**Status**: Done (stable: Tsc-s0.2.3)

**Contents**:
- Line numbers on the left of the editor
- Gray color (non-intrusive)
- Fixed width (3 columns, 2 digits)
- Aligned with file content (border TOP + RIGHT)

**Crates touched**: `tscode-tui`

---

### Tsc-v0.2.4 — Basic Editor Complete

**Status**: Done (stable: Tsc-s0.2.4)

**Contents**:
- Insert character
- Delete character
- Save
- Modes: Normal / Command / Prompt
- Vim-style command: `Esc` -> `o` (open), `w` (write)
- Status bar with per-mode colors
- 2-second hint after save
- Quit via `Alt+Q`

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v0.3.0 — Read Folder (FileTree)

**Status**: Done (stable: Tsc-s0.3.0)

**Contents**:
- `FileTree` struct in `tscode-core`
- `Entry` struct (path, name, is_dir)
- `FileTree::from_path()` reads folder contents
- Sort: folders first, then files, alphabetical
- `fs::canonicalize()` to resolve absolute path

**Crates touched**: `tscode-core`

---

### Tsc-v0.3.1 — Render File Tree

**Status**: Done (stable: Tsc-s0.3.1)

**Contents**:
- `App` has `file_tree: Option<FileTree>` field
- `load_file_tree()` reads folder
- `ui.rs` renders actual file tree in explorer
- Folder icon `+`, file without icon
- `>` marker for selected item
- Explorer width 20 columns

**Crates touched**: `tscode-tui`

---

### Tsc-v0.3.2 — Focus System + Arrow Navigation

**Status**: Done (stable: Tsc-s0.3.2)

**Contents**:
- `Focus` enum (Editor / Explorer)
- `focus` field in `App`
- `Esc` -> `e` toggles explorer + switches focus
- Arrow keys affect editor or explorer based on focus
- Yellow border + `[*]` title on focused panel
- Status bar shows `[EDITOR]` / `[EXPLORER]`

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v0.3.3 — Open File with Enter

**Status**: Done (stable: Tsc-s0.3.3)

**Contents**:
- Enter in file tree -> open file
- `open_selected_from_tree()` method
- Get `tree.selected_entry()`
- If file -> `Document::open(path)`
- Switch focus to Editor
- If folder -> show message

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v1.0.0 — Editor Integration (X=1)

**Status**: Done (stable: Tsc-s1.0.0)

**Major feature**: Full editor integration with recursive file tree

**Contents**:
- Integrate `ratatui-code-editor` for editing
- Syntax highlighting (tree-sitter, built-in)
- Visible cursor (`get_visible_cursor`)
- Undo/redo, copy/paste, cut, duplicate
- Code folding
- Recursive file tree (tree structure)
- Expand/collapse folder with Enter
- Stable root (current directory)
- Remove `tscode-syntax` (redundant)
- Save via `editor.get_content()`

**Crates touched**: `tscode-core`, `tscode-tui`, `tscode`

---

### Tsc-v1.0.1 — Cross-Platform Binary Release

**Status**: Done (stable: Tsc-s1.0.1)

**Contents**:
- Binary release for Linux x86_64
- Binary release for Android ARM64
- GitHub Actions workflows (Linux + Android)
- Fork `ratatui-code-editor` (remove `arboard` for Android compatibility)
- `git-fetch-with-cli` in workflows
- Lazy fox icon

**Crates touched**: none (CI/assets only)

---

### Tsc-v1.0.2 — Fix Line Number Visibility

**Status**: Done (stable: Tsc-s1.0.2)

**Contents**:
- Brighter line number color (`Color::White` capitalization fix)
- Fork update commit

**Crates touched**: fork `ratatui-code-editor`

---

### Tsc-v1.1.0 — Editor Polish (Y=1)

**Status**: Done (stable: Tsc-s1.1.0)

**Contents**:

**Auto-pair** (in fork `ratatui-code-editor`):
- Typing `(` -> inserts `()`, cursor inside
- Typing `{` -> inserts `{}`, cursor inside
- Typing `[` -> inserts `[]`, cursor inside
- Typing `<` -> inserts `<>`, cursor inside
- Typing `"` -> inserts `""`, cursor inside
- Typing `'` -> inserts `''`, cursor inside
- Typing `` ` `` -> inserts ` `` `, cursor inside
- Typing closing char when one is ahead -> skip (no double)

**Auto-indent** (in fork `ratatui-code-editor`):
- Enter after `{`, `(`, `[` -> new line with indent
- Enter inside `{|}` -> moves `}` to new line and places cursor between

**Themes** (in fork `ratatui-code-editor`):
- Vesper (default)
- Gruvbox
- Dracula
- Nord

**File/folder management** (in `tscode-core` + `tscode-tui`):
- `Esc` -> `n` — create new file
- `Esc` -> `v` — create new folder
- `Esc` -> `d` — delete file/folder (with `y`/`n` confirmation)
- Tree auto-refresh after create/delete

**Crates touched**: fork `ratatui-code-editor`, `tscode-core`, `tscode-tui`

---

### Tsc-v1.1.1 — Soft Theme-Aware Colors

**Status**: Done (stable: Tsc-s1.1.1)

**Contents**:
- File tree colored by type (file, folder, nested folder)
- Tab bar background & foreground follow theme
- Focus border follows theme
- Status bar soft palette
- Theme-aware UI — switch theme, all colors change
- Soft palette — comfortable for long coding sessions
- `install.sh` for quick install (Linux, macOS, Android)
- Fix `install.sh` for Termux (`set -u` removal, default values)

**Crates touched**: `tscode-tui`, root (install.sh)

---

### Tsc-v1.1.2 — Explorer Scroll & UI Polish

**Status**: Done (stable: Tsc-s1.1.2 planned)

**Contents**:
- Explorer viewport scroll (`explorer_scroll` + `explorer_height`)
- `ensure_explorer_visible()` — auto-scroll on navigation
- `skip(scroll).take(height)` — render only visible items
- Linux ARM64 build via `cross`
- Improved `install.sh` — prioritize `/public/bin` for Acode

**Crates touched**: `tscode-tui`

**Known issues**:
- Multi-tabs not yet implemented
- Config file not yet implemented
- Search in file not yet implemented

---

### Tsc-v1.1.3 — Multi-Tabs

**Status**: Planned

**Contents**:
- Multiple file tabs
- Switch with `Ctrl+Tab`
- Close with `Ctrl+W`

**Crates touched**: `tscode-tui`

---

### Tsc-v1.1.4 — Search in File

**Status**: Planned

**Contents**:
- `Ctrl+F` search
- Highlight matches
- Next/prev navigation

**Crates touched**: fork `ratatui-code-editor`, `tscode-tui`

---

### Tsc-v1.1.5 — Go to Line

**Status**: Planned

**Contents**:
- `Ctrl+G` to go to line
- Input line number

**Crates touched**: `tscode-tui`

---

### Tsc-v1.1.6 — Config File + Keybinding

**Status**: Planned

**Contents**:
- `config.toml` for settings
- Theme selection (runtime)
- Custom keybindings
- VS Code keybinding mode

**Crates touched**: `tscode-core`, `tscode-tui`, `tscode`

---

### Tsc-v2.0.0 — Integrated Terminal (X=2)

**Status**: Planned

**Major feature**: Integrated terminal

**Sub-features** (discovered during development):

| Y | Sub-feature |
|---|-------------|
| 1 | Spawn shell |
| 2 | Send input |
| 3 | Display output |
| 4 | Resize terminal |
| 5 | Multiple terminal tabs |

The number of sub-features is not fixed. It is determined by the minimum needed for the feature to work.

**Crates touched**: `tscode-terminal`, `tscode-tui`

---

### Tsc-v3.0.0 — LSP (X=3)

**Status**: Planned

**Major feature**: Language Server Protocol integration

**Sub-features** (to be discovered):

- LSP client
- Connect to `rust-analyzer`
- Autocomplete popup
- Diagnostics (error underline)
- Go-to-definition
- Hover information

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v4.0.0 — AI Agent (X=4)

**Status**: Planned

**Major feature**: AI agent integration

**Sub-features** (to be discovered):

- AI project understanding (read project structure)
- AI type provider (select model: GPT, Claude, Gemini)
- AI provider integration (connect to ChatGPT, Claude, etc.)

**Crates touched**: TBD

---

## Principles

> Make it work, make it right, make it fast.
> — Kent Beck

1. **Work** — Tsc-v0.1.0 to Tsc-s1.1.2: features work first
2. **Right** — Tsc-v2.0.0 to Tsc-s3.0.0: solid foundation
3. **Fast** — Tsc-v4.0.0 onward: optimization and advanced features