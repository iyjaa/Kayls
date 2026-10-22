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

**Kayls-s1.0.3** — Added keybinding mode switcher (Vim/Easy) with themed popup. Smoother horizontal scroll, config file, theme persistence, auto-pair, auto-indent, and themes. Unstable, in development.

See [docs/ROADMAP.md](docs/ROADMAP.md) for feature progress and [docs/VERSIONING.md](docs/VERSIONING.md) for the versioning scheme.

## Features

- **Editor** — syntax highlighting via tree-sitter, undo/redo, copy/paste, code folding
- **Auto-pair** — brackets, quotes, and angle brackets auto-close
- **Auto-indent** — indentation follows code structure
- **File tree** — recursive, expand/collapse folders, colored by type, vertical & horizontal scroll
- **File management** — create file/folder, delete with confirmation
- **Focus system** — switch between editor and explorer
- **Keybinding Modes** — Toggle between Vim (modal) and Easy (VS Code) via `Esc` then `m`
- **Themes** — Vesper (default), Gruvbox, Dracula, Nord
- **Theme-aware UI** — tab bar, focus border, file tree, and popups follow the active theme
- **Soft palette** — comfortable for long coding sessions
- **Config file** — `~/.config/kayls/config.toml` with theme and keybinding persistence
- **Command mode** — Vim-style commands (`Esc` then key)
- **Multi-language** — Rust, Python, JavaScript, TypeScript, Go, C/C++, TOML, JSON, Markdown, HTML, CSS, Bash
- **Lightweight** — ~2.4 MB binary, fast startup
- **Portable** — runs on desktop, SSH, Termux, Acode

## Install

### Quick Install (Linux, macOS, Android/Termux, Proot)

The fastest way to install Kayls is using the automated install script:

```bash
curl -fsSL https://raw.githubusercontent.com/iyjaa/Kayls/main/install.sh | sh
```

This script auto-detects your platform and installs the appropriate binary.

### Debian / Ubuntu / Linux Mint / Raspberry Pi OS (APT)

The recommended way to install Kayls on Debian-based systems. The repository is automatically updated on every release.

```bash
# 1. Add the GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://iyjaa.github.io/Kayls-repo/apt-key.asc | sudo gpg --dearmor -o /etc/apt/keyrings/kayls.gpg

# 2. Add the repository
echo "deb [signed-by=/etc/apt/keyrings/kayls.gpg] https://iyjaa.github.io/Kayls-repo stable main" | sudo tee /etc/apt/sources.list.d/kayls.list

# 3. Update and install
sudo apt update
sudo apt install kayls
```

### Linux — Direct .deb (Manual Download)

```bash
# x86_64
curl -LO https://github.com/iyjaa/Kayls/releases/latest/download/kayls_1.0.3_amd64.deb
sudo dpkg -i kayls_1.0.3_amd64.deb

# ARM64
curl -LO https://github.com/iyjaa/Kayls/releases/latest/download/kayls_1.0.3_arm64.deb
sudo dpkg -i kayls_1.0.3_arm64.deb
```

### macOS (Intel & Apple Silicon)

```bash
# Apple Silicon
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-aarch64-apple-darwin.tar.gz -o kayls.tar.gz

# Intel Mac
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-x86_64-apple-darwin.tar.gz -o kayls.tar.gz

tar xzf kayls.tar.gz
sudo mv kayls /usr/local/bin/
kayls --version
```

### Android (Termux / Acode, ARM64)

```bash
curl -L https://github.com/iyjaa/Kayls/releases/latest/download/kayls-aarch64-unknown-linux-gnu.tar.gz -o kayls.tar.gz
tar xzf kayls.tar.gz
chmod +x kayls
mv kayls $PREFIX/bin/
kayls --version
```

### Android — Proot-Distro (Ubuntu/Debian)

If you are using proot-distro inside Termux or Acode, you are already root inside the proot environment. Do not use sudo, as it is not installed by default.

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
# Open a file
kayls Cargo.toml

# Open without a file
kayls

# Show version
kayls --version

# Show help
kayls --help
```

## Configuration

Kayls reads settings from `~/.config/kayls/config.toml`. The file is created automatically when you change the theme or keybinding mode.

```toml
[editor]
theme = "vesper"
show_explorer = true
keybinding = "vim" # or "easy"
```

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

Kayls supports two keybinding modes:

1. **Vim (Modal)**: Default. Press `Esc` to enter command mode.
2. **Easy (VS Code)**: Direct `Ctrl+S`, `Ctrl+O`, `Ctrl+Q`, `Ctrl+E` shortcuts without `Esc`.

Switch mode with `Esc` then `m`. The selected mode is saved to the config file.

For a complete list of keybindings, see [KEYBINDS.md](KEYBINDS.md).

## Project Structure

Kayls is a Rust workspace with 4 crates:

| Crate | Purpose |
|-------|---------|
| `kayls-core` | FileTree, Document, Config |
| `kayls-tui` | App state, UI rendering |
| `kayls-editor` | Forked editor widget |
| `kayls` | Main binary, CLI parsing |

## Documentation

- [PRD](docs/PRD.md) — Product requirements
- [Roadmap](docs/ROADMAP.md) — Feature progress
- [Versioning](docs/VERSIONING.md) — Version scheme
- [Keybindings](KEYBINDS.md) — All shortcuts and clipboard alternatives
- [Issues](ISSUES.md) — Summary of GitHub issues
- [Contributing](docs/CONTRIBUTING.md) — How to contribute

## Contributing

Kayls is open for contributions. The workflow:

1. **Discuss first** — open an issue before coding
2. **Get approval** — wait for maintainer feedback
3. **Fork & branch** — `feature/your-feature` or `fix/your-bug`
4. **Submit PR** — reference the issue
5. **Review** — maintainer reviews and merges

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for details.

**Areas that need help:**

- Snippets (user custom)
- Multi-tabs
- Search in file
- Go to line
- Bug fixes and documentation

## License

[MIT](LICENSE)