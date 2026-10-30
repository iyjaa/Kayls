# Kayls Keybindings

## Editing (Normal Mode)

| Key | Action |
|---|---|
| Arrow keys | Move cursor |
| Any character | Insert character (with auto-pair) |
| Backspace | Delete character |
| Enter | New line (auto-indent) |
| Tab | Indent |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+D` | Duplicate line |
| `Ctrl+K` | Delete line |
| `Ctrl+A` | Select all |

## Clipboard

| Primary | Alternatives |
|---|---|
| `Alt+C` (Copy) | `Ctrl+C`, `Ctrl+Insert` |
| `Ctrl+V` (Paste) | `Shift+Insert` |

**Note:** To paste from the system clipboard on Android/Termux, use the **PASTE** button on the Termux keyboard (this triggers Bracketed Paste).

## Explorer

| Key | Action |
|---|---|
| `Esc` then `e` | Open Explorer and focus it |
| `Esc` then `q` | Close Explorer and focus Editor |

## Command Mode (after Esc)

| Key | Action |
|---|---|
| `o` | Open file |
| `n` | New file |
| `v` | New folder |
| `d` | Delete (with confirmation) |
| `t` | Select theme |
| `w` | Save file |
| `m` | Switch keybinding mode (Vim/Easy) |
| `Esc` | Cancel |

## Global

| Key | Action |
|---|---|
| `Esc` | Enter command mode |
| `Alt+Q` | Quit |
| `Ctrl+F` | Search in file |
| `Ctrl+R` | Toggle Read Mode |
| `Ctrl+P` | Open File Picker |

## Auto-pair

Typing any of these automatically inserts the matching closing character:

| Type | Result |
|---|---|
| `(` | `()` |
| `{` | `{}` |
| `[` | `[]` |
| `<` | `<>` |
| `"` | `""` |
| `'` | `''` |

## Important Note on "Vim Mode"

Kayls uses a **Vim-inspired Command Mode**, not a strict 1:1 clone of Vim Normal Mode.
- In Vim, you press `i` to insert text and `Esc` to return to Normal Mode.
- In Kayls, you press `Esc` to open a **Command Menu**, then press a single key.
- This is designed for a low learning curve, similar to VS Code Command Palette.
