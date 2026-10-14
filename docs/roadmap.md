# Tscode Roadmap

From Tsc-v0.1.0 onward.

> **This is the source of truth for feature progress.**
> Examples in other documents (like versioning.md) are illustrations only.

See [versioning.md](versioning.md) for the versioning scheme.

---

## Current Status

**Tsc-s0.2.4** — Editor dasar selesai (layout, buka file, cursor, nomor baris, insert, delete, save)

---

## Milestones

| Version | Status | Focus |
|---------|--------|-------|
| Tsc-v0.1.0 | Done | Workspace setup |
| Tsc-s0.1.0 | Done | Workspace setup verified |
| Tsc-v0.2.0 | Done | Layout TUI statis |
| Tsc-s0.2.0 | Done | Layout verified |
| Tsc-v0.2.1 | Done | Buka file via argumen |
| Tsc-s0.2.1 | Done | Buka file verified |
| Tsc-v0.2.2 | Done | Cursor movement |
| Tsc-s0.2.2 | Done | Cursor verified |
| Tsc-v0.2.3 | Done | Nomor baris |
| Tsc-s0.2.3 | Done | Nomor baris verified |
| Tsc-v0.2.4 | Done | Insert, delete, save, mode, hint |
| Tsc-s0.2.4 | Done | Editor dasar verified |
| Tsc-v0.3.0 | Planned | File tree fungsional |
| Tsc-v0.4.0 | Planned | Syntax highlighting |
| Tsc-v0.5.0 | Planned | Command palette |
| Tsc-v1.0.0 | Planned | Terminal (X=1) |
| Tsc-v2.0.0 | Planned | LSP (X=2) |
| Tsc-v3.0.0 | Planned | AI agent (X=3) |

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

### Tsc-v0.2.0 — Layout TUI Statis

**Status**: Done (stable: Tsc-s0.2.0)

**Contents**:
- Layout dengan ratatui (tab bar, file tree, editor, status bar)
- `q` untuk keluar

**Crates touched**: `tscode-tui`, `tscode`

---

### Tsc-v0.2.1 — Buka File

**Status**: Done (stable: Tsc-s0.2.1)

**Contents**:
- `Document` struct di `tscode-core`
- Parsing argumen dengan `clap`
- Buka file & tampilkan isinya
- Tab bar dinamis (nama file)

**Crates touched**: `tscode-core`, `tscode-tui`, `tscode`

---

### Tsc-v0.2.2 — Cursor Movement

**Status**: Done (stable: Tsc-s0.2.2)

**Contents**:
- `Cursor` struct (line, col)
- Arrow keys untuk gerakkan cursor
- Cursor highlight di editor (REVERSED)
- Posisi cursor di status bar
- Scroll vertikal & horizontal

**Crates touched**: `tscode-core`, `tscode-tui`

---

### Tsc-v0.2.3 — Nomor Baris

**Status**: Done (stable: Tsc-s0.2.3)

**Contents**:
- Nomor baris di kiri editor
- Warna abu-abu (tidak mengganggu)
- Lebar tetap (3 kolom, 2 digit)
- Sejajar dengan isi file (border TOP + RIGHT)

**Crates touched**: `tscode-tui`

---

### Tsc-v0.2.4 — Editor Dasar Selesai

**Status**: Done (stable: Tsc-s0.2.4)

**Contents**:
- Insert karakter (`Document::insert_char`)
- Delete karakter (`Document::delete_char`)
- Save (`Document::save`)
- Mode Normal / Command / Prompt
- Vim-style command: `Esc` → `o` (open), `w` (write)
- Status bar dengan warna per mode
- Hint 2 detik setelah save
- Quit via `Alt+Q`
- Explorer default hidden
- Autoresize terminal

**Crates touched**: `tscode-core`, `tscode-tui`

**Known issues**:
- Explorer masih teks statis (belum fungsional)
- Toggle explorer belum ada keybinding (sengaja, karena explorer default hidden)

---

### Tsc-v0.3.0 — File Tree Fungsional

**Status**: Planned

**Contents**:
- Baca folder asli (bukan hardcoded)
- Navigasi dengan arrow keys
- Buka file dengan Enter
- Expand/collapse folder
- Toggle explorer via `Esc` → `e`

**Crates touched**: `tscode-core`, `tscode-tui`

**Note**: No new features in this phase until stable.

---

### Tsc-v0.4.0 — Syntax Highlighting

**Status**: Planned

**Contents**:
- Integrate `tree-sitter` in `tscode-syntax`
- Highlight for Rust
- Highlight for Python, JavaScript
- Color theme (dark)

**Crates touched**: `tscode-syntax`, `tscode-tui`

**Note**: No new features in this phase until stable.

---

### Tsc-v0.5.0 — Command Palette

**Status**: Planned

**Contents**:
- `Ctrl+Shift+P` to open palette
- Fuzzy search for commands
- Basic commands: save, open, quit, toggle sidebar

**Crates touched**: `tscode-tui`

**Note**: No new features in this phase until stable.

---

### Tsc-v1.0.0 — Terminal (X=1)

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

### Tsc-v2.0.0 — LSP (X=2)

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

### Tsc-v3.0.0 — AI Agent (X=3)

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

1. **Work** — Tsc-v0.1.0 to Tsc-s0.5.0: features work first
2. **Right** — Tsc-v1.0.0 to Tsc-s2.0.0: solid foundation
3. **Fast** — Tsc-v3.0.0 onward: optimization and advanced features