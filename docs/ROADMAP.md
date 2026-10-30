# Kayls Roadmap

## Current Status: v1.0.9 (Stable)

- [x] Performance: migrate Document to ropey, optimize FileTree, remove blocking I/O.
- [x] UI: dynamic theme-aware gradient colors for nested folders and files.
- [x] UI: centered themed Welcome Screen with ASCII mascot.
- [x] Keybinding switcher (Easy/Vim-inspired) with themed popup.
- [x] Editor: Search in file (Ctrl+F) with auto-scroll.
- [x] Editor: Read Mode (Ctrl+R) for distraction-free scrolling.
- [x] UI: File Picker (Ctrl+P) with fuzzy search and dynamic scroll.

### v1.0.8 — Stability & Data Safety
- [x] Fixed theme switch data loss (`set_theme` instead of `load_editor`)
- [x] Revision-based dirty tracking (now catches undo/redo, Ctrl+X/K/D)
- [x] Unified clipboard (Ctrl+C/X sync to App.clipboard)
- [x] Unicode-safe search (character-based via `windows()`)
- [x] Installer checksum verification
- [x] Adaptive polling (100ms with status, 1000ms idle)
- [x] Unified `EXPLORER_WIDTH` constant
- [x] CI for `feat/*` and `docs/*` branches
- [x] Release waits for CI check
- [x] `scripts/release.sh` for version sync

### v1.0.9 — Layout & Release Polish
- [x] Fixed editor layout offsets when Explorer is hidden (x=1, width-2)
- [x] `release.sh` now stages `Cargo.lock` and asks for confirmation
- [x] `release-android.yml` waits for `check` and generates checksum
- [x] Installer supports `shasum` (macOS) fallback
- [x] Added tests for Unicode-safe search and dirty tracking

## Next Up (v1.1.x)

- [ ] **Multi-Cursor** — Edit multiple lines simultaneously (Alt+D).
- [ ] **Tutorial Termux** — Step-by-step guide for Android users.
- [ ] **Multi-Tabs**: Open and switch between multiple files.
- [ ] **Responsive Layout**: Automatically hide/overlay Explorer on narrow screens.
- [ ] **True Vim Mode**: A strict Normal/Insert mode for Vim purists.

## Future (v1.2.x+)

- [ ] **LSP Support**: Autocomplete, go-to-definition.
- [ ] **Split Panes**: View two files side-by-side.
