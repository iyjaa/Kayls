# Kayls Roadmap

## Current Status: v1.0.10 (Stable)

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

### v1.0.10 — Progressive Search
- [x] Search logic moved to `search.rs` (app.rs is leaner)
- [x] Progressive search: 10k lines per scan, instant feedback
- [x] `release.yml` uses `find` (fixes macOS/Linux release)
- [x] `release.sh` fails loudly on `cargo update` error
- [x] `scripts/retag.sh` for re-releasing the same version
- [x] Installer supports `KAYLS_STRICT_CHECKSUM=1` for strict mode

### v1.0.9 — Layout, Release & Test Polish
- [x] Fixed editor layout offsets (Explorer hidden: x=1, width-2; Explorer shown: W-(EXPLORER_WIDTH+2))
- [x] `release.sh` stages `Cargo.lock`, runs `cargo update`, and asks for confirmation
- [x] `release-android.yml` waits for `check` and generates per-file checksums
- [x] Per-file checksums (`<archive>.sha256`) to avoid conflicts between workflows
- [x] Installer supports `shasum` (macOS) fallback and downloads `<archive>.sha256`
- [x] Panic hook disables mouse capture and restores terminal
- [x] Undo/Redo now increment revision (dirty tracking catches all edits)
- [x] Real tests for Unicode-safe search (`find_matches`) and dirty tracking

## Next Up (v1.1.x)

- [ ] **Multi-Cursor** — Edit multiple lines simultaneously (Alt+D).
- [ ] **Tutorial Termux** — Step-by-step guide for Android users.
- [ ] **Multi-Tabs**: Open and switch between multiple files.
- [ ] **Responsive Layout**: Automatically hide/overlay Explorer on narrow screens.
- [ ] **True Vim Mode**: A strict Normal/Insert mode for Vim purists.

## Future (v1.2.x+)

- [ ] **LSP Support**: Autocomplete, go-to-definition.
- [ ] **Split Panes**: View two files side-by-side.
