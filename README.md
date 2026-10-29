<p align="center">
  <img src="assets/icon.png" width="128" alt="Kayls">
</p>

# Kayls

> Terminal Studio Code — A terminal-based IDE.

Kayls is a lightweight, fast terminal-based IDE written in Rust. It runs in any terminal.

## Status

**Kayls-v1.0.6** — Improved performance, dynamic Explorer colors, and new Welcome Screen.

See [docs/ROADMAP.md](docs/ROADMAP.md) for feature progress and [docs/VERSIONING.md](docs/VERSIONING.md) for versioning.

## Features

- **Editor** — syntax highlighting via tree-sitter, undo/redo, copy/paste, code folding.
- **Auto-pair & Auto-indent** — brackets and quotes auto-close.
- **File tree** — recursive, lazy-loaded, with dynamic theme-aware gradient colors.
- **Keybinding Modes** — Easy Mode (VS Code-style) and Command Mode (Vim-inspired) via Esc then m.
- **File Picker** — Fuzzy search popup (`Ctrl+P`) to open files quickly.
- **Search** — In-file search with `Ctrl+F`.
- **Read Mode** — `Ctrl+R` to scroll freely without cursor interference.
- **Themes** — Vesper (default), Gruvbox, Dracula, Nord.
- **Welcome Screen** — native ASCII mascot, centered and themed.

## Install

### Quick Install (Linux, macOS, Android/Termux)

```bash
curl -fsSL https://raw.githubusercontent.com/iyjaa/Kayls/main/install.sh | sh
```

### Debian / Ubuntu / Linux Mint / Raspberry Pi OS (APT)

```bash
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://iyjaa.github.io/Kayls-repo/apt-key.asc | sudo gpg --dearmor -o /etc/apt/keyrings/kayls.gpg
echo "deb [signed-by=/etc/apt/keyrings/kayls.gpg] https://iyjaa.github.io/Kayls-repo stable main" | sudo tee /etc/apt/sources.list.d/kayls.list
sudo apt update && sudo apt install kayls
```

### Android — Proot-Distro (Ubuntu/Debian)

```bash
apt update && apt install -y curl gnupg
mkdir -p /etc/apt/keyrings
curl -fsSL https://iyjaa.github.io/Kayls-repo/apt-key.asc | gpg --dearmor -o /etc/apt/keyrings/kayls.gpg
echo "deb [signed-by=/etc/apt/keyrings/kayls.gpg] https://iyjaa.github.io/Kayls-repo stable main" > /etc/apt/sources.list.d/kayls.list
apt update && apt install -y kayls
```

## Keybindings & Modes

Kayls supports two distinct modes, switchable via Esc then m:

1. **Easy Mode (VS Code-style)**: Direct shortcuts like Ctrl+S, Ctrl+O, Ctrl+Q.
2. **Command Mode (Vim-inspired)**: Press Esc to enter a command menu, then press a single key.

**Note:** This is not a 1:1 clone of Vim Normal Mode; it is a simplified command palette.

For a complete list, see [KEYBINDS.md](KEYBINDS.md).


[MIT](LICENSE)
