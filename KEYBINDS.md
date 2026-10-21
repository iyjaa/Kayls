# Kayls Keybindings

Kayls uses a **Vim-style command mode**. In normal mode, keys go to the editor. Press `Esc` to enter command mode.

## Editing (Normal Mode)

| Key | Action | Alternatives |
|-----|--------|--------------|
| Arrow keys | Move cursor | `h` `j` `k` `l` (Vim) |
| Any character | Insert character (auto-pair) | - |
| Backspace | Delete character | `Delete` |
| Enter | New line (auto-indent) | - |
| Tab | Indent | - |
| `Ctrl+Z` | Undo | `Alt+U` |
| `Ctrl+Y` | Redo | `Alt+R` |
| `Ctrl+D` | Duplicate line | `Alt+D` |
| `Ctrl+K` | Delete line | `Alt+K` |
| `Ctrl+A` | Select all | `Alt+A` |

## Clipboard (Copy / Paste)

Terminal-based applications face a conflict: `Ctrl+C` is used by the terminal as **SIGINT** (interrupt signal). To avoid crashing or blocking, Kayls provides multiple alternatives.

### Copy (Select text first)

| Primary | Alternatives |
|---------|--------------|
| `Ctrl+Y` | `Ctrl+C`, `Ctrl+Shift+C`, `Ctrl+Insert`, `Alt+C` |

### Paste

| Primary | Alternatives |
|---------|--------------|
| `Ctrl+P` | `Ctrl+V`, `Ctrl+Shift+V`, `Shift+Insert`, `Alt+V`, Terminal Paste button |

### Notes by Platform

- **Linux / macOS Desktop:** Use `Ctrl+Y` / `Ctrl+P` for internal clipboard. To paste from the system clipboard (e.g., a browser), use your terminal's paste shortcut (usually `Ctrl+Shift+V` or right-click).
- **Android (Termux / Proot):** Use `Ctrl+Y` / `Ctrl+P` for internal clipboard. To paste from Android clipboard, use the **PASTE** button on the Termux keyboard (this triggers Bracketed Paste).
- **SSH:** Internal clipboard works inside the session. Local-to-remote paste depends on your SSH client (e.g., Windows Terminal, PuTTY) and whether it supports Bracketed Paste.

## Explorer

| Key | Action |
|-----|--------|
| `Esc` then `e` | Open Explorer and focus it (always opens) |
| `Esc` then `q` | Close Explorer and focus the Editor |

## Command Mode (after `Esc`)

| Key | Action | Alternatives |
|-----|--------|--------------|
| `o` | Open file | `Ctrl+O` |
| `n` | New file | `Ctrl+N` |
| `v` | New folder | `Ctrl+Shift+N` |
| `d` | Delete (with confirmation) | `Delete` |
| `t` | Select theme | `Ctrl+T` |
| `w` | Save file | `Ctrl+S` |
| `e` | Open Explorer (focus) | `Ctrl+E` |
| `q` | Close Explorer (focus Editor) | `Ctrl+Q` |
| `Esc` | Cancel | - |

## Global

| Key | Action | Alternatives |
|-----|--------|--------------|
| `Esc` | Enter command mode | `Ctrl+[` |
| `Alt+Q` | Quit | `Ctrl+Q`, `F10` |

## Auto-pair

Typing any of these automatically inserts the matching closing character:

| Type | Result |
|------|--------|
| `(` | `()` |
| `{` | `{}` |
| `[` | `[]` |
| `<` | `<>` |
| `"` | `""` |
| `'` | `''` |
| `` ` `` | `` ` ` `` |

Typing a closing character when one is already ahead will skip over it (no duplicate).

**Note**: A VS Code keybinding mode (`Ctrl+S`, `Ctrl+O`, `Ctrl+B`, etc.) is planned. See [docs/roadmap.md](docs/roadmap.md).
