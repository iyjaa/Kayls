# Kayls Roadmap

This document outlines the planned features and progress for Kayls.

## Current Status: v1.0.3 (Stable)

- [x] Keybinding mode switcher (Vim/Easy) with themed popup
- [x] Multi-architecture CI/CD (Linux x86_64, ARM64, ARMv7, macOS, Android)
- [x] APT repository with GPG signing
- [x] Automated release pipeline (GitHub Actions)
- [x] Clipboard support (Ctrl+Y / Ctrl+P with alternatives)
- [x] Explorer focus system (Esc+e, Esc+q)
- [x] Comprehensive KEYBINDS.md documentation

## Next Up (v1.1.x — Q1 2026)

### High Priority
- [ ] **Search in File**: `Ctrl+F` to find text within the open document.
- [ ] **Multi-Tabs**: Open and switch between multiple files.
- [ ] **Go to Line**: Quick navigation to a specific line number.

### Improvements
- [ ] **Better Explorer Scrolling**: Smoother horizontal/vertical scrolling for large directories.
- [ ] **Theme Customization**: User-defined themes via config file.
- [ ] **Status Bar Refinements**: Show git branch, encoding, and file type.

## Future (v1.2.x+)

- [ ] **LSP Support**: Autocomplete, go-to-definition (using `tower-lsp`).
- [ ] **Split Panes**: View two files side-by-side.
- [ ] **Integrated Terminal**: A basic terminal emulator inside Kayls.
- [ ] **Plugin System**: Extend Kayls with custom scripts (TypeScript or Rust).
- [ ] **Session Restore**: Reopen files and cursor positions after restart.

## Completed

### v1.0.0 — Rebrand & Infrastructure
- [x] Renamed from Tscode to Kayls
- [x] Multi-arch build pipeline
- [x] APT repo with GPG signing

### v1.0.1 — Stability
- [x] Fix version fallback string
- [x] Fix badge URLs

### v1.0.2 — UX Fixes
- [x] Fix copy-paste across files
- [x] Fix auto-indent on paste
- [x] New explorer shortcuts (`Esc e`, `Esc q`)
- [x] Keybindings documentation

### v1.0.3 — Keybinding Modes
- [x] Added Vim/Easy mode switcher (`Esc m`)
- [x] Themed popup for mode selection
- [x] Config persistence for keybinding mode

## Versioning

See [docs/VERSIONING.md](VERSIONING.md) for the versioning scheme (`Kayls-vX.Y.Z` for stable, `Kayls-sX.Y.Z` for snapshots).
