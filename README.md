<p align="center">
  <img src="assets/icon.png" width="128" alt="Tscode">
</p>

# Tscode

> Terminal Studio Code — A terminal-based IDE inspired by VS Code.

[![Release](https://img.shields.io/github/v/release/iyjaa/Tscode?include_prereleases&label=release)](https://github.com/iyjaa/Tscode/releases)
[![License](https://img.shields.io/github/license/iyjaa/Tscode)](LICENSE)
[![Issues](https://img.shields.io/github/issues/iyjaa/Tscode)](https://github.com/iyjaa/Tscode/issues)

Tscode is a terminal-based IDE with a VS Code-inspired layout. It runs in any terminal — desktop, SSH, Termux, or Acode — and uses a single lightweight Rust binary.

## Status

**Tsc-v1.1.3** — Config file, theme persistence, explorer horizontal scroll, soft theme-aware colors, auto-pair, auto-indent, themes, file/folder management. Unstable, in development.

See [docs/roadmap.md](docs/roadmap.md) for feature progress and [docs/versioning.md](docs/versioning.md) for the versioning scheme.

## Features

- **Editor** — syntax highlighting via tree-sitter, undo/redo, copy/paste, code folding
- **Auto-pair** — brackets, quotes, and angle brackets auto-close
- **Auto-indent** — indentation follows code structure
- **File tree** — recursive, expand/collapse folders, colored by type, vertical & horizontal scroll
- **File management** — create file/folder, delete with confirmation
- **Focus system** — switch between editor and explorer
- **Themes** — Vesper (default), Gruvbox, Dracula, Nord
- **Theme-aware UI** — tab bar, focus border, and file tree colors follow the active theme
- **Soft palette** — comfortable for long coding sessions
- **Config file** — `~/.config/tscode/config.toml` with theme persistence
- **Command mode** — Vim-style commands (`Esc` then key)
- **Multi-language** — Rust, Python, JavaScript, TypeScript, Go, C/C++, TOML, JSON, Markdown, HTML, CSS, Bash
- **Lightweight** — ~2.4 MB binary, fast startup
- **Portable** — runs on desktop, SSH, Termux, Acode

## Install

### Quick install (Linux, macOS, Android)

    curl -fsSL https://raw.githubusercontent.com/iyjaa/Tscode/main/install.sh | sh

This auto-detects your platform (Linux x86_64, Linux ARM64, Android ARM64, macOS) and installs to the appropriate directory.

### macOS — Homebrew (Intel & Apple Silicon)

    brew tap iyjaa/tscode https://github.com/iyjaa/Tscode
    brew install tscode

Or download directly from [GitHub Releases](https://github.com/iyjaa/Tscode/releases):

    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-aarch64-apple-darwin.tar.gz -o tscode.tar.gz
    tar xzf tscode.tar.gz
    sudo mv tscode /usr/local/bin/

### Linux — APT (Debian / Ubuntu / Mint / Raspberry Pi OS)

    curl -fsSL https://iyjaa.github.io/Tscode/apt-key.gpg | sudo gpg --dearmor -o /usr/share/keyrings/tscode.gpg
    echo "deb [signed-by=/usr/share/keyrings/tscode.gpg] https://iyjaa.github.io/Tscode/apt stable main" | sudo tee /etc/apt/sources.list.d/tscode.list
    sudo apt update
    sudo apt install tscode

### Linux — Direct .deb

Download the `.deb` from [GitHub Releases](https://github.com/iyjaa/Tscode/releases):

    sudo dpkg -i tscode_*_amd64.deb      # x86_64
    sudo dpkg -i tscode_*_arm64.deb      # ARM64 (Raspberry Pi 4/5, 64-bit)
    sudo dpkg -i tscode_*_armhf.deb      # ARM32 (Raspberry Pi 32-bit)

### Linux — Binary (manual)

    # x86_64
    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-x86_64-unknown-linux-gnu.tar.gz -o tscode.tar.gz

    # ARM64
    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-aarch64-unknown-linux-gnu.tar.gz -o tscode.tar.gz

    # ARM32 (Raspberry Pi 32-bit)
    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-armv7-unknown-linux-gnueabihf.tar.gz -o tscode.tar.gz

    tar xzf tscode.tar.gz
    sudo mv tscode /usr/local/bin/
    tscode --version

### Android (Termux / Acode, ARM64)

Quick install (recommended):

    curl -fsSL https://raw.githubusercontent.com/iyjaa/Tscode/main/install.sh | sh

Manual install:

    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-aarch64-linux-android.tar.gz -o tscode.tar.gz
    tar xzf tscode.tar.gz
    chmod +x tscode
    mv tscode $PREFIX/bin/
    tscode --version

### Build from source

Requirements: Rust 1.70+ and Git.

    git clone https://github.com/iyjaa/Tscode
    cd Tscode
    cargo build --release
    sudo cp target/release/tscode /usr/local/bin/

**Note**: Tscode uses a custom fork of `ratatui-code-editor` (with `arboard` removed for Android compatibility, plus auto-pair and auto-indent). The fork is pulled automatically via `git-fetch-with-cli`.

## Usage

    # Open a file
    tscode Cargo.toml

    # Open without a file
    tscode

    # Show version
    tscode --version

    # Show help
    tscode --help

## Configuration

Tscode reads settings from `~/.config/tscode/config.toml`. The file is created automatically when you change the theme.

    [editor]
    theme = "vesper"
    show_explorer = true

## Themes

Available themes:

| Theme | Description |
|-------|-------------|
| **Vesper** | Default — soft green, orange, purple |
| **Gruvbox** | Warm, retro — green, yellow, mint |
| **Dracula** | Dark, soft — green, pink, purple |
| **Nord** | Cool, minimal — green, blue, cyan |

Switch theme with `Esc` then `t`. Theme is saved to the config file.

## Keybindings

Tscode uses Vim-style command mode. In normal mode, keys go to the editor. Press `Esc` to enter command mode.

### Editing (Normal Mode)

| Key | Action |
|-----|--------|
| Arrow keys | Move cursor |
| Any character | Insert character (auto-pair for brackets/quotes) |
| Backspace | Delete character |
| Enter | New line (auto-indent) |
| Tab | Indent |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+X` | Cut |
| `Ctrl+A` | Select all |
| `Ctrl+D` | Duplicate line |
| `Ctrl+K` | Delete line |

### Explorer (when focused)

| Key | Action |
|-----|--------|
| Arrow Up/Down | Navigate |
| Arrow Left/Right | Horizontal scroll |
| Enter | Open file / expand folder |

### Command Mode (after `Esc`)

| Key | Action |
|-----|--------|
| `o` | Open file |
| `n` | New file |
| `v` | New folder |
| `d` | Delete (with confirmation) |
| `t` | Select theme |
| `w` | Save file |
| `e` | Toggle explorer |
| `Esc` | Cancel |

### Global

| Key | Action |
|-----|--------|
| `Esc` | Enter command mode |
| `Alt+Q` | Quit |

### Auto-pair

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

## Project Structure

Tscode is a Rust workspace with 3 crates:

| Crate | Purpose |
|-------|---------|
| `tscode-core` | FileTree, Document, Config |
| `tscode-tui` | App state, UI rendering |
| `tscode` | Main binary, CLI parsing |

## Documentation

- [PRD](docs/prd.md) — Product requirements
- [Roadmap](docs/roadmap.md) — Feature progress
- [Versioning](docs/versioning.md) — Version scheme
- [Issues](ISSUES.md) — Summary of GitHub issues
- [Contributing](CONTRIBUTING.md) — How to contribute

## Contributing

Tscode is open for contributions. The workflow:

1. **Discuss first** — open an issue before coding
2. **Get approval** — wait for maintainer feedback
3. **Fork & branch** — `feature/your-feature` or `fix/your-bug`
4. **Submit PR** — reference the issue
5. **Review** — maintainer reviews and merges

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

**Areas that need help:**

- Snippets (user custom)
- Multi-tabs
- Search in file
- Go to line
- VS Code keybinding mode
- Bug fixes and documentation

## License

[MIT](LICENSE)