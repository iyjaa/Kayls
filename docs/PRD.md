# Product Requirements Document (PRD) - Kayls

## 1. Overview
**Kayls** is a terminal-based IDE written in Rust, designed to be a lightweight alternative to Vim, Nano, and Helix. It provides a modern, comfortable editing experience with syntax highlighting, a file tree, and multiple themes.

## 2. Target Audience
- Developers who work over SSH or in terminal environments.
- Users who find Vim too complex and Nano too simple.
- Android users (Termux, Acode) who need a portable IDE.

## 3. Core Requirements

### 3.1 Editor
- Syntax highlighting via tree-sitter.
- Undo/redo, copy/paste, code folding.
- Auto-pair and auto-indent.
- Multiple cursor support (future).

### 3.2 File Management
- Recursive file tree (lazy-loaded).
- Create, delete, and rename files/folders.
- Fuzzy file picker (future).

### 3.3 UI/UX
- Keybinding modes: Easy (VS Code-style) and Command (Vim-inspired).
- Themes: Vesper (default), Gruvbox, Dracula, Nord.
- Responsive layout for narrow screens (mobile).

### 3.4 Performance
- Fast startup (<50ms).
- Efficient memory usage via `ropey`.
- Smooth scrolling, even for large files.

## 4. Non-Goals
- Full IDE features (LSP, debugger) in the near term.
- GUI version (Kayls is terminal-only).

## 5. Success Metrics
- Binary size < 5MB.
- Startup time < 100ms.
- No panics on large files (10k+ lines).
- Active community contributions.

## 6. Versioning
See [VERSIONING.md](VERSIONING.md).

## 7. Roadmap
See [ROADMAP.md](ROADMAP.md).
