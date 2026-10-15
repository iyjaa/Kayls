# Tscode

> Terminal Studio Code — A terminal-based IDE inspired by VS Code.

[![Release](https://img.shields.io/github/v/release/iyjaa/Tscode?include_prereleases&label=release)](https://github.com/iyjaa/Tscode/releases)
[![License](https://img.shields.io/github/license/iyjaa/Tscode)](LICENSE)
[![Issues](https://img.shields.io/github/issues/iyjaa/Tscode)](https://github.com/iyjaa/Tscode/issues)

Tscode is a terminal-based IDE with a VS Code-inspired layout. It runs in any terminal — desktop, SSH, Termux, or Acode — and uses a single lightweight Rust binary.

## Status

**Tsc-v1.0.0** — Editor integration complete. Unstable, in development.

See [docs/roadmap.md](docs/roadmap.md) for feature progress and [docs/versioning.md](docs/versioning.md) for the versioning scheme.

## Features

- **Editor** — syntax highlighting via tree-sitter, undo/redo, copy/paste, code folding
- **File tree** — recursive, expand/collapse folders
- **Focus system** — switch between editor and explorer
- **Command mode** — Vim-style commands (`Esc` then key)
- **Multi-language** — Rust, Python, JavaScript, TypeScript, Go, C/C++, TOML, JSON, Markdown, HTML, CSS, Bash
- **Lightweight** — small binary, fast startup
- **Portable** — runs on desktop, SSH, Termux, Acode

## Install

### Download binary (Linux x86_64)

    curl -L https://github.com/iyjaa/Tscode/releases/latest/download/tscode-x86_64-unknown-linux-gnu.tar.gz -o tscode.tar.gz
    tar xzf tscode.tar.gz
    sudo mv tscode /usr/local/bin/
    tscode --version

### Build from source

Requirements: Rust 1.70+ and Git.

    git clone https://github.com/iyjaa/Tscode
    cd Tscode
    cargo build --release
    sudo cp target/release/tscode /usr/local/bin/

**For Termux / Acode (Android):**

    pkg install rust git
    git clone https://github.com/iyjaa/Tscode
    cd Tscode
    cargo build --release
    cp target/release/tscode $PREFIX/bin/

## Usage

    # Open a file
    tscode Cargo.toml

    # Open without a file
    tscode

    # Show version
    tscode --version

    # Show help
    tscode --help

## Keybindings

Tscode uses Vim-style command mode. In normal mode, keys go to the editor. Press `Esc` to enter command mode.

### Editing (Normal Mode)

| Key | Action |
|-----|--------|
| Arrow keys | Move cursor |
| Any character | Insert character |
| Backspace | Delete character |
| Enter | New line |
| Tab | Indent |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+X` | Cut |
| `Ctrl+A` | Select all |
| `Ctrl+D` | Duplicate line |
| `Ctrl+K` | Delete line |

### Command Mode (after `Esc`)

| Key | Action |
|-----|--------|
| `o` | Open file |
| `w` | Save file |
| `e` | Toggle explorer |
| `Esc` | Cancel |

### Global

| Key | Action |
|-----|--------|
| `Esc` | Enter command mode |
| `Alt+Q` | Quit |

**Note**: A VS Code keybinding mode (`Ctrl+S`, `Ctrl+O`, `Ctrl+B`, etc.) is planned. See [docs/roadmap.md](docs/roadmap.md).

## Project Structure

Tscode is a Rust workspace with 4 crates:

| Crate | Purpose |
|-------|---------|
| `tscode-core` | FileTree, Document |
| `tscode-terminal` | PTY, shell integration (planned) |
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

- VS Code keybinding mode
- Config file support
- Multiple file tabs
- Search in file
- Bug fixes and documentation

## License

[MIT](LICENSE)