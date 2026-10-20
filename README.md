<p align="center">
  <img src="assets/icon.png" width="128" alt="Kayls">
</p>

# Kayls

> Terminal Studio Code — A terminal-based IDE

[![Release](https://img.shields.io/github/v/release/iyjaa/Kayls?include_prereleases&label=release)](https://github.com/iyjaa/Kayls/releases)
[![License](https://img.shields.io/github/license/iyjaa/Kayls)](LICENSE)
[![Issues](https://img.shields.io/github/issues/iyjaa/Kayls)](https://github.com/iyjaa/Kayls/issues)

Kayls is a terminal-based IDE with a VS Code-inspired layout. It runs in any terminal — desktop, SSH, Termux, or Acode — and uses a single lightweight Rust binary.

## Status

**Tsc-v1.1.4** — Smoother horizontal scroll in editor. Config file, theme persistence, explorer horizontal scroll, soft theme-aware colors, auto-pair, auto-indent, themes, file/folder management. Unstable, in development.

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
- **Config file** — ~/.config/kayls/config.toml with theme persistence
- **Command mode** — Vim-style commands (Esc then key)
- **Multi-language** — Rust, Python, JavaScript, TypeScript, Go, C/C++, TOML, JSON, Markdown, HTML, CSS, Bash
- **Lightweight** — ~2.4 MB binary, fast startup
- **Portable** — runs on desktop, SSH, Termux, Acode

## Install

### Debian / Ubuntu / Linux Mint / Raspberry Pi OS (APT)

The recommended way to install Kayls on Debian-based systems. The repository is automatically updated on every release.
```bash
curl -fsSL https://iyjaa.github.io/Kayls-repo/apt-key.asc | sudo gpg --dearmor -o /usr/share/keyrings/kayls.gpg
echo "deb [signed-by=/usr/share/keyrings/kayls.gpg] https://iyjaa.github.io/Kayls-repo stable main" | sudo tee /etc/apt/sources.list.d/kayls.list
sudo apt update
sudo apt install kayls
```

### Android — Proot-Distro (Ubuntu/Debian)

If you are using proot-distro (Ubuntu/Debian) inside Termux or Acode, you are already root inside the proot environment. Do not use sudo, as it is not installed by default. Use the following commands instead:

```bash
# 1. Install prerequisites
apt update && apt install -y curl gnupg

# 2. Add the GPG key
mkdir -p /etc/apt/keyrings
curl -fsSL https://iyjaa.github.io/Kayls-repo/apt-key.asc | gpg --dearmor -o /etc/apt/keyrings/kayls.gpg

# 3. Add the repository
echo "deb [signed-by=/etc/apt/keyrings/kayls.gpg] https://iyjaa.github.io/Kayls-repo stable main" > /etc/apt/sources.list.d/kayls.list

# 4. Update and install
apt update
apt install -y kayls
```
### Linux — Direct .deb (Manual Download)

If you prefer to download the .deb file directly from GitHub Releases:
```bash
curl -LO https://github.com/iyjaa/Kayls/releases/latest/download/kayls_1.1.4_amd64.deb
sudo dpkg -i kayls_1.1.4_amd64.deb

curl -LO https://github.com/iyjaa/Kayls/releases/latest/download/kayls_1.1.4_arm64.deb
sudo dpkg -i kayls_1.1.4_arm64.deb
```
### Linux — Pre-compiled Binary (Manual)

If you don't want to use apt or dpkg, download the raw binary from GitHub Releases:
```bash
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-x86_64-unknown-linux-gnu.tar.gz -o kayls.tar.gz
tar xzf kayls.tar.gz
sudo mv kayls /usr/local/bin/
kayls --version
```
### macOS (Intel & Apple Silicon)

Download the binary directly from GitHub Releases:
```bash
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-aarch64-apple-darwin.tar.gz -o kayls.tar.gz
tar xzf kayls.tar.gz
sudo mv kayls /usr/local/bin/
kayls --version
```
### Android (Termux / Acode, ARM64)

You can install Kayls inside Termux or an Acode Alpine/Ubuntu proot environment.
```bash
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-aarch64-unknown-linux-gnu.tar.gz -o kayls.tar.gz
tar xzf kayls.tar.gz
chmod +x kayls
mv kayls $PREFIX/bin/
kayls --version
```
### Build from source

Requirements: Rust 1.70+ and Git.
```bash
git clone https://github.com/iyjaa/Kayls
cd Kayls
cargo build --release
sudo cp target/release/kayls /usr/local/bin/
```
**Note**: Kayls uses a custom fork of ratatui-code-editor (with arboard removed for Android compatibility, plus auto-pair and auto-indent). The fork is pulled automatically via git-fetch-with-cli.

## Usage
```bash
kayls Cargo.toml
kayls
kayls --version
kayls --help
```
## Configuration

Kayls reads settings from ~/.config/kayls/config.toml. The file is created automatically when you change the theme.
```toml
[editor]
theme = "vesper"
show_explorer = true
```
## Themes

Available themes:

| Theme | Description |
|-------|-------------|
| **Vesper** | Default — soft green, orange, purple |
| **Gruvbox** | Warm, retro — green, yellow, mint |
| **Dracula** | Dark, soft — green, pink, purple |
| **Nord** | Cool, minimal — green, blue, cyan |

Switch theme with Esc then t. Theme is saved to the config file.

## Keybindings

Kayls uses Vim-style command mode. In normal mode, keys go to the editor. Press Esc to enter command mode.

### Editing (Normal Mode)

| Key | Action |
|-----|--------|
| Arrow keys | Move cursor |
| Any character | Insert character (auto-pair for brackets/quotes) |
| Backspace | Delete character |
| Enter | New line (auto-indent) |
| Tab | Indent |
| Ctrl+Z | Undo |
| Ctrl+Y | Redo |
| Ctrl+C | Copy |
| Ctrl+V | Paste |
| Ctrl+X | Cut |
| Ctrl+A | Select all |
| Ctrl+D | Duplicate line |
| Ctrl+K | Delete line |

### Explorer (when focused)

| Key | Action |
|-----|--------|
| Arrow Up/Down | Navigate |
| Arrow Left/Right | Horizontal scroll |
| Enter | Open file / expand folder |

### Command Mode (after Esc)

| Key | Action |
|-----|--------|
| o | Open file |
| n | New file |
| v | New folder |
| d | Delete (with confirmation) |
| t | Select theme |
| w | Save file |
| e | Toggle explorer |
| Esc | Cancel |

### Global

| Key | Action |
|-----|--------|
| Esc | Enter command mode |
| Alt+Q | Quit |

### Auto-pair

Typing any of these automatically inserts the matching closing character:

| Type | Result |
|------|--------|
| ( | () |
| { | {} |
| [ | [] |
| < | <> |
| " | "" |
| ' | '' |

Typing a closing character when one is already ahead will skip over it (no duplicate).

**Note**: A VS Code keybinding mode (Ctrl+S, Ctrl+O, Ctrl+B, etc.) is planned. See [docs/roadmap.md](docs/roadmap.md).

## Project Structure

Kayls is a Rust workspace with 3 crates:

| Crate | Purpose |
|-------|---------|
| kayls-core | FileTree, Document, Config |
| kayls-tui | App state, UI rendering |
| kayls | Main binary, CLI parsing |

## Documentation

- [PRD](docs/prd.md) — Product requirements
- [Roadmap](docs/roadmap.md) — Feature progress
- [Versioning](docs/versioning.md) — Version scheme
- [Issues](ISSUES.md) — Summary of GitHub issues
- [Contributing](CONTRIBUTING.md) — How to contribute

## Contributing

Kayls is open for contributions. The workflow:

1. **Discuss first** — open an issue before coding
2. **Get approval** — wait for maintainer feedback
3. **Fork & branch** — feature/your-feature or fix/your-bug
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