# Tscode Roadmap

From Tsc-v0.1.0 onward.

> **This is the source of truth for feature progress.**
> Examples in other documents (like versioning.md) are illustrations only.

See [versioning.md](versioning.md) for the versioning scheme.

---

## Current Status

**Tsc-v1.1.0** — Editor polish: auto-pair, auto-indent, themes, file/folder management.

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
| Tsc-s1.1.0 | Planned | Editor polish verified |
| Tsc-v1.1.1 | Planned | Multi-tabs |
| Tsc-v1.1.2 | Planned | Search in file |
| Tsc-v1.1.3 | Planned | Go to line |
| Tsc-v1.1.4 | Planned | Config file + keybinding |
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

### Tsc-v1.1.0 — Editor Polish (Y=1)

**Status**: Done (stable: Tsc-s1.1.0 planned)

**Contents**:

**Auto-pair** (in fork `ratatui-code-editor`):
- Typing `(` → inserts `()`, cursor inside
- Typing `{` → inserts `{}`, cursor inside
- Typing `[` → inserts `[]`, cursor inside
- Typing `<` → inserts `<>`, cursor inside
- Typing `"` → inserts `""`, cursor inside
- Typing `'` → inserts `''`, cursor inside
- Typing `` ` `` → inserts ` `` `, cursor inside
- Typing closing char when one is ahead → skip (no double)

**Auto-indent** (in fork `ratatui-code-editor`):
- Enter after `{`, `(`, `[` → new line with indent
- Enter inside `{|}` → moves `}` to new line and places cursor between

**Themes** (in fork `ratatui-code-editor`):
- Vesper (default)
- Gruvbox
- Dracula
- Nord

**File/folder management** (in `tscode-core` + `tscode-tui`):
- `Esc` → `n` — create new file
- `Esc` → `v` — create new folder
- `Esc` → `d` — delete file/folder (with `y`/`n` confirmation)
- Tree auto-refresh after create/delete

**Crates touched**: fork `ratatui-code-editor`, `tscode-core`, `tscode-tui`

**Known issues**:
- Theme selection is compile-time (config file planned)
- No rename yet
- No multi-tabs yet

---

### Tsc-v1.1.1 — Multi-Tabs

**Status**: Planned

**Contents**:
- Multiple file tabs
- Switch with `Ctrl+Tab`
- Close with `Ctrl+W`

**Crates touched**: `tscode-tui`

---

### Tsc-v1.1.2 — Search in File

**Status**: Planned

**Contents**:
- `Ctrl+F` search
- Highlight matches
- Next/prev navigation

**Crates touched**: fork `ratatui-code-editor`, `tscode-tui`

---

### Tsc-v1.1.3 — Go to Line

**Status**: Planned

**Contents**:
- `Ctrl+G` to go to line
- Input line number

**Crates touched**: `tscode-tui`

---

### Tsc-v1.1.4 — Config File + Keybinding

**Status**: Planned

**Contents**:
- `config.toml` for settings
- Theme selection (runtime)
- Custom keybindings
- VS Code keybinding mode

**Crates touched**: `tscode-core`, `tscode-tui`, `tscode`

---

## Principles

> Make it work, make it right, make it fast.
> — Kent Beck

1. **Work** — Tsc-v0.1.0 to Tsc-s1.1.0: features work first
2. **Right** — Tsc-v2.0.0 to Tsc-s3.0.0: solid foundation
3. **Fast** — Tsc-v4.0.0 onward: optimization and advanced features